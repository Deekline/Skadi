use anyhow::Result;
use crate::controllers::{CitySelection, get_cities_by_name, get_weather_by_geo};
use crate::state::{AppState, CitySearchResult, Focus};
use crossterm::event::{Event, KeyCode};
use tui_input::InputRequest;

pub enum Mode {
    Search,
    History,
}

pub fn read_event(event: Event, app_state: &mut AppState) -> Result<bool> {
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
                    handle_city_pick(app_state, key_event.code, Mode::Search);
                }
                Focus::History => {
                    handle_city_pick(app_state, key_event.code, Mode::History);
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

pub fn handle_city_pick(app: &mut AppState, key_event: KeyCode, mode: Mode) {
    match app.search_selected {
        Some(selected) => app.search_selected = Some(selected),
        None => app.search_selected = Some(0),
    };

    let cities_list: &[CitySearchResult] = match mode {
        Mode::Search => &app.search_results,
        Mode::History => &app.history,
    };

    match key_event {
        KeyCode::Down => {
            match mode {
                Mode::Search => {
                    let current_selected = app.search_selected.unwrap_or(0);
                    if current_selected + 1 < cities_list.len() {
                        app.search_selected = Some(current_selected + 1);
                    }
                }
                Mode::History => {
                    let current_selected = app.history_selected.unwrap_or(0);
                    if current_selected + 1 < cities_list.len() {
                        app.history_selected = Some(current_selected + 1);
                    }
                }
            };
        }
        KeyCode::Up => {
            match mode {
                Mode::Search => {
                    let current_selected = app.search_selected.unwrap_or(0);
                    if current_selected > 0 {
                        app.search_selected = Some(current_selected - 1);
                    }
                }
                Mode::History => {
                    let current_selected = app.history_selected.unwrap_or(0);
                    if current_selected > 0 {
                        app.history_selected = Some(current_selected - 1);
                    }
                }
            };
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

pub fn handle_input_key(
    app_state: &mut AppState,
    key_event: KeyCode,
) -> Result<()> {
    match key_event {
        KeyCode::Backspace => {
            app_state.city_input.handle(InputRequest::DeletePrevChar);
        }
        KeyCode::Char(c) => {
            app_state.city_input.handle(InputRequest::InsertChar(c));
        }
        KeyCode::Enter => {
            get_cities_by_name(app_state)?;
        }
        _ => println!("Event"),
    }
    Ok(())
}
