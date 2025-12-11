use crate::state::AppState;
use crossterm::event::{self, Event, KeyCode};
use tui_input::InputRequest;

pub fn handle_input_key(app_state: &mut AppState, key_event: KeyCode) {
    match key_event {
        KeyCode::Backspace => {
            app_state
                .city_input
                .input
                .handle(InputRequest::DeletePrevChar);
        }
        KeyCode::Char(c) => {
            app_state
                .city_input
                .input
                .handle(InputRequest::InsertChar(c));
        }
        _ => println!("Event"),
    }
}
