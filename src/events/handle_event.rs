use std::error::Error;

use crossterm::event::{Event, KeyCode};

use crate::events::input_events::handle_input_key;
use crate::state::{AppState, Focus};

pub fn read_event(event: Event, app_state: &mut AppState) -> Result<bool, Box<dyn Error>> {
    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Esc => {
                app_state.app_exit();
            }
            KeyCode::Tab => {
                app_state.focus_next();
            }
            _ => match app_state.focus {
                Focus::Input => {
                    handle_input_key(app_state, key_event.code)?;
                }
                Focus::SearchResults => {
                    todo!()
                }
                Focus::History => {
                    todo!()
                }
                Focus::Favorites => {
                    todo!()
                }
            },
        },
        _ => {
            todo!()
        }
    }
    Ok(app_state.exit)
}
