use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: meshes.add(Cuboid::default().mesh()),
        material: materials.add(Color::srgb(0.9, 0.0, 0.0)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };
    commands.spawn(player);
}
