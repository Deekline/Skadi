use crate::controllers::{CitySelection, get_cities_by_name, get_weather_by_geo};
use crate::state::{AppState, CitySearchResult, Focus};
use anyhow::Result;
use crossterm::event::{Event, KeyCode};
use tui_input::InputRequest;

pub enum Mode {
    Search,
    History,
}

pub fn read_event(event: Event, app: &mut AppState) -> Result<bool> {
    match event {
        Event::Key(key_event) => match key_event.code {
            KeyCode::Esc => {
                if app.history_popup {
                    app.history_popup = false;
                } else if app.search_popup {
                    app.search_popup = false;
                } else {
                    app.app_exit();
                }
            }
            KeyCode::Char(c) => {
                if c == 's' {
                    app.search_popup = true;
                }
                if c == 'h' {
                    app.history_popup = true;
                }
            }
            _ => {
                if app.history_popup {
                    handle_city_pick(app, key_event.code, Mode::History);
                }
                if app.search_popup {
                    match app.focus {
                        Focus::Input => {
                            handle_input_key(app, key_event.code)?;
                        }
                        Focus::SearchResults => {
                            handle_city_pick(app, key_event.code, Mode::Search);
                        }
                    }
                }
            }
        },
        _ => {
            todo!()
        }
    }
    Ok(app.exit)
}

pub fn handle_city_pick(app: &mut AppState, key_event: KeyCode, mode: Mode) {
    match app.search_selected {
        Some(selected) => app.search_selected = Some(selected),
        None => app.search_selected = Some(0),
    };

    let cities_list_len: usize = match mode {
        Mode::Search => app.search_results.len(),
        Mode::History => app.history.len(),
    };

    match key_event {
        KeyCode::Down => {
            match mode {
                Mode::Search => {
                    let current_selected = app.search_selected.unwrap_or(0);
                    if current_selected + 1 >= cities_list_len {
                        app.search_selected = Some(0)
                    } else {
                        app.search_selected = Some(current_selected + 1);
                    }
                }
                Mode::History => {
                    let current_selected = app.history_selected.unwrap_or(0);
                    if current_selected + 1 >= cities_list_len {
                        app.history_selected = Some(0)
                    } else {
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

pub fn handle_input_key(app: &mut AppState, key_event: KeyCode) -> Result<()> {
    match key_event {
        KeyCode::Backspace => {
            app.city_input.handle(InputRequest::DeletePrevChar);
        }
        KeyCode::Char(c) => {
            app.city_input.handle(InputRequest::InsertChar(c));
        }
        KeyCode::Enter => {
            get_cities_by_name(app)?;
        }
        _ => println!("Event"),
    }
    Ok(())
}
