use crate::{
    services::get_weather,
    state::{
        AppState, Weather, WeatherResponse,
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

pub fn get_weather_by_geo(state: &mut AppState) {
    let city = if let Some(fav) = state.favorite.as_ref() {
        fav
    } else {
        let Some(selected_idx) = state.search_selected else {
            return;
        };
        let Some(city) = state.search_results.get(selected_idx) else {
            return;
        };
        city
    };
    match get_weather(&city.coordinates.lat, &city.coordinates.lon) {
        Ok(weather) => {
            let parsed_weather = parse_weather(weather);
            state.current_city = Some(city.clone());
            state.weather = Some(parsed_weather);
            state.weather_error = None;
        }
        Err(err) => {
            state.weather = None;
            state.weather_error = Some(err.to_string());
        }
    }
}
