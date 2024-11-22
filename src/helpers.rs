use crate::models::Planet;
use bevy::prelude::*;
use tokio::runtime::Runtime;

pub async fn fetch_planets_from_api() -> Result<Vec<Planet>, reqwest::Error> {
    let response = reqwest::get("http://127.0.0.1:8000/api/planets")
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

// pub fn make_mesh(planet: &Planet, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
//     let default_radius = 1.0;
//     let radius = match &planet.diameter {
//         Some(d) => d.parse::<f32>().unwrap_or(default_radius) / 2.0,
//         None => default_radius,
//     };
//     let mesh_handle = meshes.add(Mesh::from(Sphere {
//         radius,
//         // subdivisions: 32, // Number of subdivisions (higher number = smoother sphere)
//     }));
//     mesh_handle
// }

pub fn make_mesh(planet_diameter: &f32, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let mesh_handle = meshes.add(Mesh::from(Sphere {
        radius: planet_diameter / 2.0,
        // subdivisions: 32, // Number of subdivisions (higher number = smoother sphere)
    }));
    mesh_handle
}

pub fn make_transform(index: u32) -> Transform {
    let index_float = index as f32;
    let transform = Transform::from_xyz(index_float * 2.0, 0.0, 0.0);
    transform
}
