use anyhow::{Context, Result};

use crate::state::WeatherResponse;

pub fn get_weather(lat: &f64, lon: &f64) -> Result<WeatherResponse> {
    let mut request_url = reqwest::Url::parse("https://api.open-meteo.com/v1/forecast")
        .context("build weather url")?;
    request_url
        .query_pairs_mut()
        .append_pair("latitude", &lat.to_string())
        .append_pair("longitude", &lon.to_string())
        .append_pair("timezone", "auto")
        .append_pair(
            "current",
            "temperature_2m,apparent_temperature,relative_humidity_2m,precipitation,weather_code,wind_speed_10m,wind_direction_0m,visibility",
        )
        .append_pair(
            "hourly",
            "temperature_2m,relative_humidity_2m,weather_code,precipitation,precipitation_probability,wind_speed_10m,wind_direction_10m",
        )
        .append_pair(
            "daily",
            "weather_code,temperature_2m_max,temperature_2m_min,precipitation_sum,precipitation_probability_max",
        )
        .append_pair("forecast_days", "10");

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(request_url)
        .send()
        .context("send weather request")?
        .error_for_status()
        .context("weather response status")?;
    let text = response.text().context("read weather body")?;
    let weather = serde_json::from_str(&text).context("parse weather json")?;

    Ok(weather)
}
