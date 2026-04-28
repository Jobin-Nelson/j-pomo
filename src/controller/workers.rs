use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

use ratatui::crossterm::event;

use crate::controller::events::{PomoCommand, PomoEvent};

pub fn get_input(tx: mpsc::Sender<PomoEvent>) {
    thread::spawn(move || {
        loop {
            if let Ok(event) = event::read() {
                let result = match event {
                    event::Event::Key(key) => tx.send(PomoEvent::Input(key)),
                    event::Event::Resize(_, _) => tx.send(PomoEvent::Resize),
                    _ => Ok(()),
                };
                // if receiver is dropped, stop listening
                if result.is_err() {
                    break;
                }
            }
        }
    });
}

pub fn pomo_worker(tx: mpsc::Sender<PomoEvent>, rx: mpsc::Receiver<PomoCommand>) -> JoinHandle<()> {
    thread::spawn(move || {
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
                        // if send fails main thread has hung up. Time to exit.
                        if tx
                            .send(PomoEvent::PomoUpdate(remaining_secs, progress as f64))
                            .is_err()
                        {
                            break;
                        }
                    } else {
                        is_paused = true;
                        // if send fails main thread has hung up. Time to exit.
                        if tx.send(PomoEvent::PomoDone).is_err() {
                            break;
                        }
                    }
                }
            }
        }
    })
}
