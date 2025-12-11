use crate::{events::read_event, state::AppState, ui::draw_city_input};
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::io::Result;

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state = AppState::new();
    loop {
        terminal.draw(|x| draw_city_input(x, &app_state))?;

        let event = event::read()?;
        let should_break = read_event(event, &mut app_state);
        if should_break {
            break;
        }
    }
    Ok(())
}
