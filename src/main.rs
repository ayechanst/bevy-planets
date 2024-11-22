mod camera;
mod helpers;
mod models;
mod player;
mod world;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::*;
use camera::CameraPlugin;
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
