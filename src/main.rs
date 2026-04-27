use j_pomo::Result;
use j_pomo::controller::run;

use ratatui::{TerminalOptions, Viewport};

fn main() -> Result<()> {
    let terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(13),
    });

    let app_result = run(terminal);

    ratatui::restore();

    app_result
}
