use anyhow::{Context, Result};

use crate::state::GeocodingResponse;

pub fn search_cities(query: &str) -> Result<GeocodingResponse> {
    let request_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=10&language=en",
        query,
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(request_url)
        .send()
        .context("send geocoding request")?
        .error_for_status()
        .context("geocoding response status")?;
    let geocoding = response
        .json()
        .context("decode geocoding response")?;

    Ok(geocoding)
}
