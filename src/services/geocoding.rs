use crate::state::GeocodingResponse;

pub fn search_cities(query: &str) -> Result<GeocodingResponse, Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=10&language=en",
        query,
    );

    let client = reqwest::blocking::Client::new();
    let response = client.get(request_url).send()?;
    let geocoding = response.json()?;

    Ok(geocoding)
}
