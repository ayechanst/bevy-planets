use crate::models::Planet;
use bevy::prelude::*;
use tokio::runtime::Runtime;

pub async fn fetch_planets_from_api() -> Result<Vec<Planet>, reqwest::Error> {
    let response = reqwest::get("http://127.0.0.1:8000/api/planets/size")
        .await?
        .json::<Vec<Planet>>()
        .await?;
    Ok(response)
}

pub fn get_planets() -> Vec<Planet> {
    let runtime = Runtime::new().unwrap();
    let planets: Vec<Planet> = runtime
        .block_on(fetch_planets_from_api())
        .unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            vec![]
        });
    planets
}

pub fn make_mesh(planet: &Planet, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let default_radius = 1000.0;
    let radius = match &planet.diameter {
        Some(d) => d.parse::<f32>().unwrap_or(default_radius) / 2.0,
        None => default_radius,
    };
    let mesh_handle = meshes.add(Mesh::from(Sphere {
        radius: radius / 1500.0,
        // subdivisions: 32, // Number of subdivisions (higher number = smoother sphere)
    }));
    println!("Planet: {}, Mesh: {}", &planet.name, radius);
    mesh_handle
}

pub fn make_material(
    planet: &Planet,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let base_color = match &planet.climate {
        Some(climate) => match climate.as_str() {
            "unknown" => Color::srgb(0.8, 0., 0.),
            "temperate" => Color::srgb(0.1, 0.8, 0.),
            "arid" => Color::srgb(0.9, 0.0, 0.1),
            "tropical" => Color::srgb(0.0, 0.5, 0.5),
            "frozen" | "artic" | "subartic" | "frigid" => Color::srgb(1.0, 1.0, 1.0),
            "superheated" => Color::srgb(1.0, 0., 0.),
            _ => Color::srgb(0., 0., 0.),
        },
        None => Color::srgb(0.5, 0.5, 0.5),
    };
    materials.add(StandardMaterial {
        base_color,
        ..default()
    })
}

pub fn make_transform(index: u32) -> Transform {
    let index_float = index as f32;
    let transform = Transform::from_xyz(index_float * 2.0, 0.0, 0.0);
    transform
}
