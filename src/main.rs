use j_pomo::Result;
use j_pomo::controller::run;

fn main() -> Result<()> {
    let terminal = ratatui::init();

    let app_result = run(terminal);

    ratatui::restore();

    app_result
}
