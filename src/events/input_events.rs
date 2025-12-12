use crate::{controllers::get_cities_by_name, state::AppState};
use crossterm::event::KeyCode;
use tui_input::InputRequest;

pub fn handle_input_key(app_state: &mut AppState, key_event: KeyCode) {
    match key_event {
        KeyCode::Backspace => {
            app_state.city_input.handle(InputRequest::DeletePrevChar);
        }
        KeyCode::Char(c) => {
            app_state.city_input.handle(InputRequest::InsertChar(c));
        }
        KeyCode::Enter => {
            get_cities_by_name(app_state);
        }
        _ => println!("Event"),
    }
}
