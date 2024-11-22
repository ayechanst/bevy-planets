use crate::helpers::{get_planets, make_mesh, make_transform};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_planets));
    }
}

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planets = get_planets();
    let mut index = 0;
    // for (i, planet) in planets.iter().enumerate() {
    for planet in planets {
        // let transform = Transform::from_xyz(i as f32, 0.0, 0.0);
        let transform = make_transform(index);
        println!("Location: {:?}", transform);
        let planet_bundle = PbrBundle {
            mesh: make_mesh(&planet, &mut meshes),
            // transform: make_transform(index),
            ..default()
        };
        index += 1;
        commands.spawn(planet_bundle);
        println!(
            "Spawning planet {} at transform: {:?}",
            planet.name, transform
        );
        println!("Spawned Planet: {}", planet.name);
    }
}

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
