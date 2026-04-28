use std::path::PathBuf;
use std::sync::mpsc;

use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::config::{StateFileGuard, create_state_file};
use crate::controller::actions::alert_user;
use crate::controller::events::{PomoCommand, PomoEvent};
use crate::controller::support::{next_pomo, prev_pomo};
use crate::controller::workers::{get_input, pomo_worker};
use crate::models::{App, AppMode, PomoStatus};
use crate::{Error, Result};

pub fn run(terminal: DefaultTerminal) -> Result<()> {
    let state_file = create_state_file()?;
    let _guard = StateFileGuard {
        path: state_file.clone(),
    };

    let (event_tx, event_rx) = mpsc::channel();
    let (worker_tx, worker_rx) = mpsc::channel();

    get_input(event_tx.clone());

    let event_to_worker = event_tx.clone();
    let worker = pomo_worker(event_to_worker, worker_rx);

    let app = App::new();

    let app_result = app.run_loop(terminal, event_rx, worker_tx, state_file);

    worker.join().unwrap();

    app_result
}

impl App {
    fn run_loop(
        mut self,
        mut terminal: DefaultTerminal,
        event_rx: mpsc::Receiver<PomoEvent>,
        worker_tx: mpsc::Sender<PomoCommand>,
        state_file: PathBuf,
    ) -> Result<()> {
        worker_tx
            .send(PomoCommand::Start(self.pomo.kind.get_mins()))
            .unwrap();
        loop {
            terminal
                .draw(|frame| frame.render_widget(&self, frame.area()))
                .map_err(Error::Io)?;
            match event_rx.recv().unwrap() {
                PomoEvent::Input(event) => match self.mode {
                    AppMode::Progress => match (event.modifiers, event.code) {
                        (_, KeyCode::Char('q')) => {
                            let _ = worker_tx.send(PomoCommand::Quit);
                            break;
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                            let _ = worker_tx.send(PomoCommand::Quit);
                            break;
                        }
                        (_, KeyCode::Char('p')) => {
                            self.pomo.status = match self.pomo.status {
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
                        (_, KeyCode::Char('r')) => {
                            worker_tx
                                .send(PomoCommand::Start(self.pomo.kind.get_mins()))
                                .unwrap();
                            self.pomo.status = PomoStatus::Running;
                        }
                        (_, KeyCode::Char('n')) => {
                            self.pomo = next_pomo(self.pomo);
                            worker_tx
                                .send(PomoCommand::Start(self.pomo.kind.get_mins()))
                                .unwrap();
                            self.pomo.status = PomoStatus::Running;
                        }
                        (_, KeyCode::Char('N')) => {
                            self.pomo = prev_pomo(self.pomo);
                            worker_tx
                                .send(PomoCommand::Start(self.pomo.kind.get_mins()))
                                .unwrap();
                            self.pomo.status = PomoStatus::Running;
                        }
                        (_, KeyCode::Char('s')) => {
                            self.mode = AppMode::SessionName;
                            self.pomo.status = PomoStatus::Running;
                        }
                        _ => {}
                    },
                    AppMode::SessionName => match (event.modifiers, event.code) {
                        (_, KeyCode::Esc) => self.mode = AppMode::Progress,
                        (_, KeyCode::Enter) | (KeyModifiers::CONTROL, KeyCode::Char('m')) => {
                            self.mode = AppMode::Progress;
                            let session_name = self.session_name.lines().join("\n");
                            self.pomo.session = if session_name.is_empty() {
                                None
                            } else {
                                Some(session_name)
                            }
                        }
                        _ => {
                            let _ = self.session_name.input(event);
                        }
                    },
                },
                PomoEvent::Resize => {
                    terminal.autoresize()?;
                }
                PomoEvent::PomoUpdate(remaining_secs, progress) => {
                    self.pomo.rem = remaining_secs;
                    self.pomo.progress = progress;
                    let _ = std::fs::write(
                        state_file.as_path(),
                        format!(
                            "{}: {}:{}",
                            self.pomo.kind,
                            self.pomo.rem / 60,
                            self.pomo.rem % 60
                        ),
                    );
                }
                PomoEvent::PomoDone => {
                    self.pomo = next_pomo(self.pomo);
                    alert_user(&self.pomo.kind);
                    worker_tx
                        .send(PomoCommand::Start(self.pomo.kind.get_mins()))
                        .unwrap();
                }
            }
        }
        Ok(())
    }
}
