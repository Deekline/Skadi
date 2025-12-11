use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::state::AppState;
use std::io::Result;

pub fn read_event(event: Event, app_state: &mut AppState) -> bool {
    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Char('q') => {
                app_state.app_exit();
                app_state.exit
            }
            _ => app_state.exit,
        },
        _ => app_state.exit,
    }
}
