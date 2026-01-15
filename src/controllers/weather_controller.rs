use crate::{
    configuration::Config,
    services::get_weather,
    state::{AppState, CitySearchResult, Weather},
};

#[derive(Clone, Copy, Debug)]
pub enum CitySelection {
    Search(usize),
    Favorite,
    History(usize),
}

fn selected_city(state: &AppState, selected: CitySelection) -> Option<CitySearchResult> {
    match selected {
        CitySelection::Search(i) => state.search_results.get(i).cloned(),
        CitySelection::Favorite => state.favorite.clone(),
        CitySelection::History(i) => state.history.get(i).cloned(),
    }
}

pub fn get_weather_by_geo(state: &mut AppState, city_selected: CitySelection) {
    let Some(city) = selected_city(state, city_selected) else {
        return;
    };
    match get_weather(&city.coordinates.lat, &city.coordinates.lon) {
        Ok(weather) => {
            let parsed_weather = Weather::from(weather);
            let city = city.clone();
            state.current_city = Some(city.clone());
            state.weather = Some(parsed_weather);
            state.weather_error = None;

            state.history.retain(|c| c != &city);
            state.history.insert(0, city.clone());
            state.history.truncate(20);
            state.history_selected = Some(0);

            if let Err(err) = Config::save_history(city.clone()) {
                eprintln!("Failed to save history: {err}");
            }
        }
        Err(err) => {
            state.weather = None;
            state.weather_error = Some(err.to_string());
        }
    }
}
