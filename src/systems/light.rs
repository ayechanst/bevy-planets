use bevy::prelude::*;

pub fn spawn_light(commands: &mut Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            range: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 5.0, 5.0), // Position the light
        ..default()
    });
}
