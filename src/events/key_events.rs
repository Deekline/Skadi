use crate::controllers::{CitySelection, get_cities_by_name, get_weather_by_geo};
use crate::state::{AppState, Focus};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use tui_input::InputRequest;

const MAX_CITY_INPUT_LEN: usize = 20;

pub enum Mode {
    Search,
    History,
}

pub fn read_event(event: Event, app: &mut AppState) -> Result<bool> {
    match event {
        Event::Key(key_event) => handle_key_event(app, key_event.code)?,
        _ => {
            todo!()
        }
    }
    Ok(app.exit)
}

fn handle_key_event(app: &mut AppState, key_code: KeyCode) -> Result<()> {
    match key_code {
        KeyCode::Esc => handle_escape(app),
        KeyCode::Char(c) => handle_char_key(app, c)?,
        KeyCode::Tab if app.search_popup => app.focus_next(),
        _ => handle_popup_keys(app, key_code)?,
    }
    Ok(())
}

fn handle_escape(app: &mut AppState) {
    if app.history_popup {
        app.history_popup = false;
    } else if app.search_popup {
        app.search_popup = false;
    } else {
        app.app_exit();
    }
}

fn handle_char_key(app: &mut AppState, character: char) -> Result<()> {
    if app.search_popup && matches!(app.focus, Focus::Input) {
        handle_input_key(app, KeyCode::Char(character))?;
        return Ok(());
    }

    match character {
        's' => app.search_popup = true,
        'h' => app.history_popup = true,
        _ => {}
    }

    Ok(())
}

fn handle_popup_keys(app: &mut AppState, key_event: KeyCode) -> Result<()> {
    if app.history_popup {
        handle_city_pick(app, key_event, Mode::History);
    }

    if app.search_popup {
        match app.focus {
            Focus::Input => handle_input_key(app, key_event)?,
            Focus::SearchResults => handle_city_pick(app, key_event, Mode::Search),
        }
    }

    Ok(())
}

pub fn handle_city_pick(app: &mut AppState, key_event: KeyCode, mode: Mode) {
    match app.search_selected {
        Some(selected) => app.search_selected = Some(selected),
        None => app.search_selected = Some(0),
    };

    let (cities_list_len, selected) = match mode {
        Mode::Search => (app.search_results.len(), &mut app.search_selected),
        Mode::History => (app.history.len(), &mut app.history_selected),
    };

    match key_event {
        KeyCode::Down => {
            let current_selected = selected.unwrap_or(0);
            if current_selected + 1 >= cities_list_len {
                *selected = Some(0);
            } else {
                *selected = Some(current_selected + 1);
            }
        }
        KeyCode::Up => {
            let current_selected = selected.unwrap_or(0);
            if current_selected > 0 {
                *selected = Some(current_selected - 1);
            }
        }
        KeyCode::Enter => match mode {
            Mode::Search => {
                get_weather_by_geo(
                    app,
                    CitySelection::Search(app.search_selected.expect("Seach city is not selected")),
                );
            }
            Mode::History => {
                get_weather_by_geo(
                    app,
                    CitySelection::History(
                        app.history_selected.expect("History city is not selected"),
                    ),
                );
            }
        },
        _ => {}
    };
}

pub fn handle_input_key(app: &mut AppState, key_event: KeyCode) -> Result<()> {
    match key_event {
        KeyCode::Backspace => {
            app.city_input.handle(InputRequest::DeletePrevChar);
        }
        KeyCode::Char(c) => {
            if app.city_input.value().chars().count() < MAX_CITY_INPUT_LEN {
                app.city_input.handle(InputRequest::InsertChar(c));
            }
        }
        KeyCode::Enter => {
            get_cities_by_name(app)?;
            app.focus_next();
        }
        _ => (),
    }
    Ok(())
}
