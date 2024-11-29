mod camera;
mod helpers;
mod models;
mod player;
mod world;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
use camera::CameraPlugin;
// use helpers::{get_async_planets, get_planets, use_blender};
use helpers::{get_planets, use_blender};
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldPlugin,
            PlayerPlugin,
            CameraPlugin,
            WorldInspectorPlugin::new(),
            ThirdPersonCameraPlugin,
        ))
        .run();
}

// #[tokio::main]
// async fn main() {
//     let planets = get_async_planets();
//     for planet in planets.await.iter() {
//         match use_blender(planet).await {
//             Ok(response) => {
//                 println!("Success: {:?}", response);
//             }
//             Err(e) => {
//                 println!("Error sending planet to Flask: {:?}", e);
//             }
//         }
//     }
// }
