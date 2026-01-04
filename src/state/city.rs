use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct GeocodingResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: String,
    pub country_code: String,
    pub admin1: Option<String>,
    pub admin2: Option<String>,
    pub timezone: String,
}

#[derive(Deserialize, Debug)]
pub struct GeocodingResponse {
    pub results: Vec<GeocodingResult>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct CitySearchResult {
    pub name: String,
    pub country: String,
    pub coordinates: GeoCoordinates,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GeoCoordinates {
    pub lat: f64,
    pub lon: f64,
}
