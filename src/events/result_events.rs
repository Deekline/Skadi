pub fn handle_city_pick(app: &mut AppState, key_event: KeyCode, mode: Mode) {
    match app.search_selected {
        Some(selected) => app.search_selected = Some(selected),
        None => app.search_selected = Some(0),
    };

    match key_event {
        KeyCode::Down => {
            let current_selected = app.search_selected.unwrap_or(0);
            if current_selected + 1 < app.search_results.len() {
                app.search_selected = Some(current_selected + 1);
            }
        }
        KeyCode::Up => {
            let current_selected = app.search_selected.unwrap_or(0);
            if current_selected > 0 {
                app.search_selected = Some(current_selected - 1);
            }
        }
        KeyCode::Enter => {
            get_weather_by_geo(app);
        }
        _ => {}
    };
}
