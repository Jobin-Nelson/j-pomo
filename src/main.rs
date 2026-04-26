use ratatui::TerminalOptions;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::Command;
use std::{sync::mpsc, thread, time::Duration};

use crossterm::event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{LineGauge, List, ListItem, Widget};
use ratatui::{DefaultTerminal, Viewport, symbols};

const FOCUS_MINS: u32 = 1;
const BREAK_MINS: u32 = 5;
const LONG_BREAK_MINS: u32 = 15;

#[derive(Default)]
enum PomoKind {
    #[default]
    Focus,
    Break,
    LongBreak,
}

impl PomoKind {
    fn get_mins(&self) -> u32 {
        match self {
            PomoKind::Focus => FOCUS_MINS,
            PomoKind::Break => BREAK_MINS,
            PomoKind::LongBreak => LONG_BREAK_MINS,
        }
    }
}

impl Display for PomoKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PomoKind::Focus => write!(f, "FOCUS"),
            PomoKind::Break => write!(f, "BREAK"),
            PomoKind::LongBreak => write!(f, "LONG BREAK"),
        }
    }
}

type PomoProgress = f64;
type RemainingSecs = u32;

enum Event {
    Input(event::KeyEvent),
    Resize,
    PomoUpdate(RemainingSecs, PomoProgress),
    PomoDone,
}

enum PomoCommand {
    Start(u32),
    Pause,
    Resume,
    Quit,
}

#[derive(Debug, Default)]
enum PomoStatus {
    #[default]
    Running,
    Paused,
}

#[derive(Default)]
struct Pomo {
    kind: PomoKind,
    count: usize,
    progress: PomoProgress,
    status: PomoStatus,
    rem: RemainingSecs,
}

impl Pomo {
    fn new() -> Self {
        Default::default()
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(10),
    });

    let (event_tx, event_rx) = mpsc::channel();
    let (worker_tx, worker_rx) = mpsc::channel();

    handle_input(event_tx.clone());

    let event_to_worker = event_tx.clone();
    let worker = thread::spawn(move || {
        pomo_worker(event_to_worker, worker_rx);
    });

    let app_result = run(&mut terminal, event_rx, worker_tx);

    ratatui::restore();

    worker.join().unwrap();

    app_result
}

fn next_pomo(pomo: Pomo) -> Pomo {
    match pomo.kind {
        PomoKind::Focus => {
            if pomo.count > 0 && pomo.count.is_multiple_of(4) {
                Pomo {
                    kind: PomoKind::LongBreak,
                    count: pomo.count,
                    ..Default::default()
                }
            } else {
                Pomo {
                    kind: PomoKind::Break,
                    count: pomo.count,
                    ..Default::default()
                }
            }
        }
        _ => Pomo {
            kind: PomoKind::Focus,
            count: pomo.count + 1,
            ..Default::default()
        },
    }
}

fn handle_input(tx: mpsc::Sender<Event>) {
    thread::spawn(move || {
        loop {
            match event::read().unwrap() {
                event::Event::Key(key) => tx.send(Event::Input(key)).unwrap(),
                event::Event::Resize(_, _) => tx.send(Event::Resize).unwrap(),
                _ => {}
            }
        }
    });
}

fn pomo_worker(tx: mpsc::Sender<Event>, rx: mpsc::Receiver<PomoCommand>) {
    let mut total_secs = 0;
    let mut remaining_secs = 0;
    let mut is_paused = false;
    loop {
        let timeout = if is_paused {
            Duration::MAX
        } else {
            Duration::from_secs(1)
        };

        match rx.recv_timeout(timeout) {
            Ok(command) => match command {
                PomoCommand::Start(mins) => {
                    total_secs = mins * 60;
                    remaining_secs = total_secs;
                    is_paused = false;
                }
                PomoCommand::Pause => is_paused = true,
                PomoCommand::Resume => is_paused = false,
                PomoCommand::Quit => break,
            },
            Err(_) => {
                if is_paused {
                    continue;
                }
                if remaining_secs > 0 {
                    remaining_secs -= 1;
                    let progress = (total_secs - remaining_secs) * 100 / total_secs;
                    tx.send(Event::PomoUpdate(remaining_secs, progress as f64))
                        .unwrap();
                } else {
                    is_paused = true;
                    tx.send(Event::PomoDone).unwrap();
                }
            }
        }
    }
}

fn get_state_file() -> PathBuf {
    let state_file = std::env::var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".local/state")
        })
        .join("pomodoro/status.txt");

    if let Some(parent) = state_file.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    state_file
}

fn alert_user(pomo_kind: &PomoKind) {
    let _ = Command::new("notify-send")
        .arg("-u")
        .arg("critical")
        .arg("-a")
        .arg("pomo")
        .arg(format!(
            "START {}: {} mins",
            pomo_kind,
            pomo_kind.get_mins()
        ))
        .spawn();

    let _ = Command::new("paplay")
        .arg("/usr/share/sounds/freedesktop/stereo/complete.oga")
        .spawn()
        .ok();
}

fn run(
    terminal: &mut DefaultTerminal,
    event_rx: mpsc::Receiver<Event>,
    worker_tx: mpsc::Sender<PomoCommand>,
) -> std::io::Result<()> {
    let mut pomo = Pomo::new();
    worker_tx
        .send(PomoCommand::Start(pomo.kind.get_mins()))
        .unwrap();
    let state_file = get_state_file();
    loop {
        // terminal.draw(|frame| render(frame, &pomo))?;
        terminal.draw(|frame| frame.render_widget(&pomo, frame.area()))?;
        match event_rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                event::KeyCode::Char('q') => {
                    worker_tx.send(PomoCommand::Quit).unwrap();
                    break;
                }
                event::KeyCode::Char('p') => {
                    pomo.status = match pomo.status {
                        PomoStatus::Running => {
                            worker_tx.send(PomoCommand::Pause).unwrap();
                            PomoStatus::Paused
                        }
                        PomoStatus::Paused => {
                            worker_tx.send(PomoCommand::Resume).unwrap();
                            PomoStatus::Running
                        }
                    }
                }
                event::KeyCode::Char('r') => {
                    worker_tx
                        .send(PomoCommand::Start(pomo.kind.get_mins()))
                        .unwrap();
                    pomo.status = PomoStatus::Running;
                }
                event::KeyCode::Char('n') => {
                    pomo = next_pomo(pomo);
                    worker_tx
                        .send(PomoCommand::Start(pomo.kind.get_mins()))
                        .unwrap();
                    pomo.status = PomoStatus::Running;
                }
                _ => {}
            },
            Event::Resize => {
                terminal.autoresize()?;
            }
            Event::PomoUpdate(remaining_secs, progress) => {
                pomo.rem = remaining_secs;
                pomo.progress = progress;
                let _ = std::fs::write(
                    state_file.as_path(),
                    format!("{}: {}:{}", pomo.kind, pomo.rem / 60, pomo.rem % 60),
                );
            }
            Event::PomoDone => {
                pomo = next_pomo(pomo);
                alert_user(&pomo.kind);
                worker_tx
                    .send(PomoCommand::Start(pomo.kind.get_mins()))
                    .unwrap();
            }
        }
    }
    Ok(())
}

impl Widget for &Pomo {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let main_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![Constraint::Min(40), Constraint::Percentage(50)])
            .split(area);
        let main_layout = main_layouts[0];
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(main_layout);

        let header: Line = vec!["POMO".blue().bold()].into();
        header.centered().render(layout[0], buf);

        List::from_iter([
            ListItem::new(format!(" Session: {}", self.kind)),
            ListItem::new(format!(" Status: {:?}", self.status)),
            ListItem::new(format!(" Rem: {}:{}", self.rem / 60, self.rem % 60)),
        ])
        .render(layout[1], buf);

        LineGauge::default()
            .filled_style(Style::new().white().on_black().bold())
            .filled_symbol(symbols::line::THICK_HORIZONTAL)
            .ratio(self.progress / 100_f64)
            .render(layout[2], buf);
    }
}
