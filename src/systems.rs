use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: meshes.add(Cuboid::default().mesh()),
        material: materials.add(Color::srgb(0.9, 0.0, 0.0)),
        ..default()
    };
    commands.spawn(player);
}

pub fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };
    commands.spawn(light);
}

pub fn spawn_camera(mut commands: Commands) {
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    commands.spawn(camera);
}

pub fn spawn_floor(
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
