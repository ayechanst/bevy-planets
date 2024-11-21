mod helpers;
mod models;
mod resources;
mod systems;
// mod systems;
// use crate::resources::PlanetsResource;
use bevy::{color::palettes::css::DARK_GREEN, prelude::*};
use helpers::{get_planets, make_mesh};
use models::Planet;
use systems::{spawn_camera, spawn_floor, spawn_light, spawn_player};
// use systems::{spawn_camera, spawn_floor, spawn_light};

// use resources::{
//     loading_state::{self, LoadingState},
//     // planets_resource::setup_planets_resource,
// };
// use systems::{camera::spawn_camera, light::spawn_light, planets::spawn_planets};
// use std::f32::consts::PI;

fn main() {
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     // .add_systems(Startup, setup_planets_resource)
    //     .add_systems(Startup, setup)
    //     .run();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_floor, spawn_camera, spawn_light, spawn_player),
        )
        .run();

    // let planets = get_planets();
    // for planet in planets {
    //     println!("{:?}", planet);
    // }
}

// #[derive(Resource)]
// pub struct PlanetsResource {
//     pub planets: Vec<Planet>,
// }

// pub fn setup_planets_resource(mut commands: Commands) {
//     let planets = get_planets();
//     if !planets.is_empty() {
//         commands.insert_resource(PlanetsResource { planets });
//     } else {
//         println!("Error in setup_planets_resource");
//         return;
//     }
// }

// pub fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     planets_resource: ResMut<PlanetsResource>,
// ) {
//     let debug_material = materials.add(StandardMaterial {
//         base_color: Color::rgb(0.5, 0.5, 1.0),
//         ..default()
//     });
//     // let planets = get_planets();
//     // commands.insert_resource(PlanetsResource { planets });

//     for (i, planet) in planets_resource.planets.iter().enumerate() {
//         // let scale = planet.diameter / 12742.0; // Example scaling factor
//         let mesh = make_mesh(&planet, &mut meshes);
//         commands.spawn(PbrBundle {
//             mesh: mesh,
//             material: debug_material.clone(),
//             transform: Transform::from_xyz(
//                 (i as f32 * 4.0) - (planets_resource.planets.len() as f32 * 2.0),
//                 0.0,
//                 0.0,
//             ),
//             ..default()
//         });
//     }

//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             range: 1000.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(5.0, 5.0, 5.0), // Position the light
//         ..default()
//     });

//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
//         ..default()
//     });
// }
