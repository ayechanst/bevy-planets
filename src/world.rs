use crate::{
    helpers::{get_planets, make_mesh, make_transform},
    models::Planet,
};
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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let planets = get_planets();
    for (i, planet) in planets.iter().enumerate() {
        let mesh = make_mesh(planet, &mut meshes);
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.7, 0.9),
            ..default()
        });
        commands.spawn(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(i as f32 * 4.0, 0.0, 0.0),
            ..default()
        });
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
            intensity: 3500.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };
    commands.spawn(light);
}
