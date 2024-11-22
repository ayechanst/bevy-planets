use crate::{
    helpers::{get_planets, make_mesh, make_transform},
    models::Planet,
};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_pbr_bundles));
        // app.add_systems(Startup, render_planets);
    }
}

fn spawn_pbr_bundles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planets = get_planets();
    // for i in 0..num_entities {
    for (i, planet) in planets.iter().enumerate() {
        // let mesh = meshes.add(Mesh::from(Sphere { radius: 2.0 }));
        let default_radius = 10.0;
        let radius = match &planet.diameter {
            Some(d) => d.parse::<f32>().unwrap_or(default_radius) / 2.0,
            None => default_radius,
        };
        println!("Radius: {}", radius);
        let mesh = meshes.add(Mesh::from(Sphere {
            radius: radius / 1000.0,
            // subdivisions: 32, // Number of subdivisions (higher number = smoother sphere)
        }));
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.7, 0.9),
            ..default()
        });
        // let transform = Transform::from_xyz(i as f32 * 2.0, 0.0, 0.0);
        commands.spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(i as f32 * 2.0, 0.0, 0.0),
            ..default()
        });
    }
}

// fn spawn_planets(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     // mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let planets = get_planets();
//     for (i, planet) in planets.iter().enumerate() {
//         let planet_bundle = PbrBundle {
//             mesh: make_mesh(&planet., &mut meshes),
//             transform: Transform::from_xyz(0.0, i as f32 * 2.0, 0.0),
//             ..default()
//         };
//         commands.spawn(planet_bundle);
//         println!("Spawning planet: {}", planet.name);
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
            intensity: 2500.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };
    commands.spawn(light);
}
