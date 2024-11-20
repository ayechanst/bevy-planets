use std::f32::consts::PI;
mod models;
use models::Planet;

use bevy::prelude::*;
use tokio::runtime::Runtime;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_planets)
        .run();
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
        let scale = planet.diameter / 12742.0; // Earth's diameter for reference

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: scale * 2.0, // Scale diameter into radius
                subdivisions: 32,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 1.0),
                ..default()
            }),
            transform: Transform::from_xyz(i as f32 * 4.0, 0.0, 0.0),
            ..default()
        });
    }
}

async fn fetch_planets_from_api() -> Result<Vec<Planet>, reqwest::Error> {
    let url = "http://127.0.0.1:8000/api/planets"; // Replace with your actual API URL
    let response = reqwest::get(url).await?.json::<Vec<Planet>>().await?;
    Ok(response)
}

// fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
//     for mut transform in &mut query {
//         transform.rotate_y(time.delta_seconds() / 2.);
//     }
// }
