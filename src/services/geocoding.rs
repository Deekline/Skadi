use anyhow::{Context, Result};

use crate::state::GeocodingResponse;

pub fn search_cities(query: &str) -> Result<GeocodingResponse> {
    let mut request_url = reqwest::Url::parse("https://geocoding-api.open-meteo.com/v1/search")
        .context("build geocoding url")?;
    request_url
        .query_pairs_mut()
        .append_pair("name", query)
        .append_pair("count", "10")
        .append_pair("language", "en");

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(request_url)
        .send()
        .context("send geocoding request")?
        .error_for_status()
        .context("geocoding response status")?;
    let geocoding = response.json().context("decode geocoding response")?;

    Ok(geocoding)
}
