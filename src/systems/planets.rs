use crate::{helpers::make_mesh, resources::PlanetsResource};
use bevy::prelude::*;

pub fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planets_resource: Res<PlanetsResource>,
    // this should take in meshes
) {
    let debug_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.5, 0.5, 1.0),
        ..default()
    });

    for (i, planet) in planets_resource.planets.iter().enumerate() {
        // let scale = planet.diameter / 12742.0; // Example scaling factor
        let mesh = make_mesh(&planet, &mut meshes);
        // create_material(planet) => outputs material
        commands.spawn(PbrBundle {
            mesh: mesh,
            material: debug_material.clone(),
            transform: Transform::from_xyz(
                (i as f32 * 4.0) - (planets_resource.planets.len() as f32 * 2.0),
                0.0,
                0.0,
            ),
            ..default()
        });
    }
}
