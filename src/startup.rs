use crate::{events::read_event, state::AppState, ui::render};
use crossterm::event;
use ratatui::DefaultTerminal;
use std::io::Result;

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state = AppState::new();
    loop {
        terminal.draw(|x| render(x, &app_state))?;

        let event = event::read()?;
        let should_break = read_event(event, &mut app_state);
        match should_break {
            Ok(should_exit) => {
                if should_exit {
                    break;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
    Ok(())
}
