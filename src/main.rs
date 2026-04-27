use std::sync::mpsc;

use j_pomo::config::{StateFileGuard, get_state_file};
use j_pomo::controller::run;
use j_pomo::controller::workers::{handle_input, pomo_worker};

use ratatui::{TerminalOptions, Viewport};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(13),
    });

    let state_file = get_state_file();
    let _guard = StateFileGuard {
        path: state_file.clone(),
    };

    let (event_tx, event_rx) = mpsc::channel();
    let (worker_tx, worker_rx) = mpsc::channel();

    handle_input(event_tx.clone());

    let event_to_worker = event_tx.clone();
    let worker = pomo_worker(event_to_worker, worker_rx);

    let app_result = run(&mut terminal, event_rx, worker_tx, state_file);

    ratatui::restore();

    worker.join().unwrap();

    app_result
}
