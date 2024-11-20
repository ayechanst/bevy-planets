use serde::Deserialize;

#[derive(Deserialize)]
pub struct Planet {
    pub name: String,
    pub rotation_period: String,
    pub orbital_period: String,
    pub diameter: String,
    pub surface_area: f32,
    pub comparison_factor: f32,
    pub climate: String,
    pub gravity: String,
    pub terrain: String,
    pub surface_water: String,
    pub population: String,
    pub population_density: f32,
    pub population_word: String,
    pub url: String,
}
