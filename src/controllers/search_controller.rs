use std::error::Error;

use crate::{
    services::search_cities,
    state::{AppState, CitySearchResult, GeoCoordinates},
};

pub fn get_cities_by_name(app: &mut AppState) -> Result<(), Box<dyn Error>> {
    let query = app.city_input.to_string();
    if query.trim().is_empty() {
        return Ok(());
    }
    let cities = search_cities(&query).map_err(|e| {
        eprintln!("Error fetching cities: {e}");
        e
    })?;

    let parsed_cities = cities
        .results
        .into_iter()
        .map(|city| {
            let coordinates = GeoCoordinates {
                lat: city.latitude,
                lon: city.longitude,
            };
            CitySearchResult {
                name: city.name,
                country: city.country,
                coordinates,
            }
        })
        .collect();

    app.search_results = parsed_cities;

    Ok(())
}
