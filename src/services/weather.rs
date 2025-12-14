use reqwest::Error;

pub fn get_weather(query: &str) -> Result<(), Error> {
    let request_url = format!(
        "https://api.open-meteo.com/v1/forecast?{}=51.1079&{}=17.0385&timezone=auto&current=temperature_2m,apparent_temperature,relative_humidity_2m,precipitation,weather_code,wind_speed_10m,wind_direction_0m,visibility&hourly=temperature_2m,relative_humidity_2m,weather_code,precipitation,precipitation_probability,wind_speed_10m,wind_direction_10m&daily=weather_code,temperature_2m_max,temperature_2m_min,precipitation_sum,precipitation_probability_max&forecast_days=10",
        query.lat, query.lon
    );
    todo!()
}
