use crate::{
    helpers::{get_planets, make_mesh, make_transform},
    models::Planet,
};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_pbr_bundles));
        // app.add_systems(Startup, render_planets);
    }
}

fn spawn_pbr_bundles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_entities = 10;

    for i in 0..num_entities {
        // Create a new mesh
        let mesh = meshes.add(Mesh::from(Sphere { radius: 2.0 }));

        // Create a material
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.7, 0.9),
            ..default()
        });

        // Compute the transform for this entity
        let transform = Transform::from_xyz(i as f32 * 2.0, 0.0, 0.0);

        // Create and spawn the PbrBundle
        commands.spawn(PbrBundle {
            mesh,
            material,
            transform,
            ..default()
        });
    }
}

#[derive(Component)]
pub struct PlanetComponent {
    pub name: String,
    pub diameter: f32,
    pub position: Vec3,
}

fn spawn_planet_component(mut commands: Commands) {
    let planets = get_planets();
    for (i, planet) in planets.iter().enumerate() {
        let position = Vec3::new(i as f32 * 2.0, 0.0, 0.0);
        let default_diameter = 1.0;
        let diameter = match &planet.diameter {
            Some(d) => d.parse::<f32>().unwrap_or(default_diameter),
            None => default_diameter,
        };
        commands.spawn((
            PlanetComponent {
                name: planet.name.clone(),
                diameter,
                position,
            },
            Transform::from_translation(position),
            GlobalTransform::default(),
        ));
    }
}

fn render_planets(
    mut commands: Commands,
    mut query: Query<(Entity, &PlanetComponent, &Transform), Without<Handle<Mesh>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, planet, transform) in query.iter_mut() {
        commands.entity(entity).insert(PbrBundle {
            mesh: make_mesh(&planet.diameter, &mut meshes),
            // material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: *transform,
            ..default()
        });
    }
}

// fn spawn_planets(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     // mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let planets = get_planets();
//     for (i, planet) in planets.iter().enumerate() {
//         let planet_bundle = PbrBundle {
//             mesh: make_mesh(&planet., &mut meshes),
//             transform: Transform::from_xyz(0.0, i as f32 * 2.0, 0.0),
//             ..default()
//         };
//         commands.spawn(planet_bundle);
//         println!("Spawning planet: {}", planet.name);
//     }
// }

fn spawn_floor(
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

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };
    commands.spawn(light);
}
