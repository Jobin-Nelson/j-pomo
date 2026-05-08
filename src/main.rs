use j_pomo::Result;
use j_pomo::controller::run;

fn main() -> Result<()> {
    let arg = std::env::args().nth(1);
    if let Some("clean") = arg.as_deref() {
        return j_pomo::config::clean_state_file();
    }

    let terminal = ratatui::init();

    let app_result = run(terminal);

    ratatui::restore();

    app_result
}
