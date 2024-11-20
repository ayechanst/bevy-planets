use std::f32::consts::PI;
mod helpers;
mod models;
use bevy::prelude::*;
use helpers::{fetch_planets_from_api, get_planets};
use models::Planet;
use tokio::runtime::Runtime;

fn main() {
    let planets = get_planets();
    for planet in &planets {
        println!("{:?}", planet);
    }
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Startup, setup)
    //     .add_systems(Startup, spawn_planets)
    //     .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Start a blocking async runtime
    let runtime = Runtime::new().unwrap();

    let planets: Vec<Planet> = runtime
        .block_on(fetch_planets_from_api())
        .unwrap_or_else(|_| vec![]);

    for (i, planet) in planets.into_iter().enumerate() {
        // let planet_diameter = planet
        //     .diameter
        //     .parse()
        //     .expect("Failed to parse String as Float");
        // let scale = (planet_diameter) / 12742.0;

        commands.spawn(PbrBundle {
            // mesh: meshes.add(Mesh::from(shape::Icosphere {
            //     radius: scale * 2.0,
            //     subdivisions: 32,
            // })),
            mesh: meshes.add(Sphere::default()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(i as f32 * 4.0, 0.0, 0.0),
            ..default()
        });
    }
}
