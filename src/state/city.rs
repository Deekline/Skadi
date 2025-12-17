use serde::Deserialize;

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

#[derive(Clone)]
pub struct CitySearchResult {
    pub name: String,
    pub country: String,
    pub country_code: String,
    pub admin1: Option<String>,
    pub coordinates: GeoCoordinates,
    pub timezone: String,
    pub is_favorite: bool,
}

#[derive(Clone)]
pub struct GeoCoordinates {
    pub lat: f64,
    pub lon: f64,
}
