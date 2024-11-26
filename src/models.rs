use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Planet {
    pub name: String,
    pub rotation_period: Option<String>,
    pub orbital_period: Option<String>,
    pub diameter: Option<String>,
    pub surface_area: Option<f32>,
    pub comparison_factor: Option<f32>,
    pub climate: Option<String>,
    pub gravity: Option<String>,
    pub terrain: Option<String>,
    pub surface_water: Option<String>,
    pub population: Option<String>,
    pub population_density: Option<f32>,
    pub population_word: Option<String>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BlenderResponse {
    pub status: String,
    pub file_path: Option<String>,
}
