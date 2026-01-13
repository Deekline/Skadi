use anyhow::Result;
use weather::startup::run;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    Ok(result?)
}
