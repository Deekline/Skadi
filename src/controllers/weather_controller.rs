use crate::{
    configuration::Config,
    services::get_weather,
    state::{
        AppState, CitySearchResult, Weather, WeatherResponse,
        weather::{CurrentWeather, DailyWeather, HourlyWeather},
    },
};

fn parse_weather(weather_response: WeatherResponse) -> Weather {
    let current_weather = CurrentWeather {
        time: weather_response.current.time,
        interval: weather_response.current.interval,
        temperature_2m: weather_response.current.temperature_2m,
        apparent_temperature: weather_response.current.apparent_temperature,
        relative_humidity_2m: weather_response.current.relative_humidity_2m,
        precipitation: weather_response.current.precipitation,
        weather_code: weather_response.current.weather_code,
        wind_speed_10m: weather_response.current.wind_speed_10m,
        wind_direction_0m: weather_response.current.wind_direction_0m,
        visibility: weather_response.current.visibility,
    };

    let hourly_weather = HourlyWeather {
        time: weather_response.hourly.time,
        temperature: weather_response.hourly.temperature_2m,
        humidity: weather_response.hourly.relative_humidity_2m,
        weather_code: weather_response.hourly.weather_code,
        precipitation: weather_response.hourly.precipitation,
        precipitation_probability: weather_response.hourly.precipitation_probability,
        wind_speed: weather_response.hourly.wind_speed_10m,
    };

    let daily_weather = DailyWeather {
        date: weather_response.daily.time,
        min_temp: weather_response.daily.temperature_2m_min,
        max_temp: weather_response.daily.temperature_2m_max,
        weather_code: weather_response.daily.weather_code,
        precipitation_sum: weather_response.daily.precipitation_sum,
        precipitation_probability_max: weather_response.daily.precipitation_probability_max,
    };

    Weather {
        current: current_weather,
        hourly: hourly_weather,
        daily: daily_weather,
    }
}

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
            let parsed_weather = parse_weather(weather);
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
