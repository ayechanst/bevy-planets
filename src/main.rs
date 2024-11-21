mod helpers;
mod models;
mod resources;
mod systems;
use crate::resources::PlanetsResource;
use bevy::{color::palettes::css::SILVER, prelude::*};
use resources::planets_resource::setup_planets_resource;
use systems::{camera::spawn_camera, light::spawn_light, planets::spawn_planets};
// use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_planets_resource)
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planets_resource: Res<PlanetsResource>,
) {
    spawn_camera(&mut commands);
    spawn_light(&mut commands);
    spawn_planets(commands, meshes, materials, planets_resource);

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
    //     material: materials.add(Color::from(SILVER)),
    //     ..default()
    // });
}
