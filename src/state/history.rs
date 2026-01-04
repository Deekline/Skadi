use crate::state::GeoCoordinates;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct History {
    pub name: String,
    pub country: String,
    pub coordinates: GeoCoordinates,
}
