use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error getting camera: {}", e)).unwrap(),
        };
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }
        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            // mesh: meshes.add(Cuboid::default().mesh()),
            mesh: meshes.add(Mesh::from(Sphere { radius: 5.0 })),
            material: materials.add(Color::srgb(0.9, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Speed(30.0),
        Player,
        ThirdPersonCameraTarget,
    );
    commands.spawn(player);
}
