use crate::{
    helpers::{get_planets, make_mesh},
    models::Planet,
};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor, spawn_planets));
    }
}

// Just calls the API and puts them into the PlanetsResource
fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planets = get_planets();
    for planet in planets {
        let mesh = make_mesh(&planet, &mut meshes);
        let planet_bundle = PbrBundle {
            mesh: mesh,
            ..default()
        };
        commands.spawn(planet_bundle);
    }
}

// fn setup_planets_resource(mut commands: Commands) {
//     let planets = get_planets();
//     if !planets.is_empty() {
//         // commands.insert_resource(PlanetsResource { planets });
//     } else {
//         println!("Error in setup_planets_resource");
//         return;
//     }
// }

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(15., 15.)),
        material: materials.add(Color::srgb(0.0, 0.66, 0.0)),
        ..default()
    };
    commands.spawn(floor);
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };
    commands.spawn(light);
}
