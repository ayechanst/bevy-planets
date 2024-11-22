mod camera;
mod helpers;
mod models;
mod player;
mod world;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldPlugin,
            CameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}
