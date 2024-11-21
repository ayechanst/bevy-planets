use crate::{helpers::get_planets, models::Planet};
use bevy::prelude::{Commands, Resource};

#[derive(Resource)]
pub struct PlanetsResource {
    pub planets: Vec<Planet>,
}

// Just calls the API and puts them into the PlanetsResource
pub fn setup_planets_resource(mut commands: Commands) {
    let planets = get_planets();
    commands.insert_resource(PlanetsResource { planets });
}
