use std::path::PathBuf;
use std::sync::mpsc;

use crossterm::event;
use ratatui::DefaultTerminal;

use crate::controller::actions::alert_user;
use crate::controller::events::{Event, PomoCommand};
use crate::controller::support::{next_pomo, prev_pomo};
use crate::models::{Pomo, PomoStatus};

pub fn run(
    terminal: &mut DefaultTerminal,
    event_rx: mpsc::Receiver<Event>,
    worker_tx: mpsc::Sender<PomoCommand>,
    state_file: PathBuf,
) -> std::io::Result<()> {
    let mut pomo = Pomo::new();
    worker_tx
        .send(PomoCommand::Start(pomo.kind.get_mins()))
        .unwrap();
    loop {
        terminal.draw(|frame| frame.render_widget(&pomo, frame.area()))?;
        match event_rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                event::KeyCode::Char('q') => {
                    let _ = worker_tx.send(PomoCommand::Quit);
                    break;
                }
                event::KeyCode::Char('c')
                    if event.modifiers.contains(event::KeyModifiers::CONTROL) =>
                {
                    let _ = worker_tx.send(PomoCommand::Quit);
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
                event::KeyCode::Char('N') => {
                    pomo = prev_pomo(pomo);
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
