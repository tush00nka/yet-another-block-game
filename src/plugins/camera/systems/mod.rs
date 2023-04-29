use bevy::prelude::*;

use super::components::CameraComponent;

#[allow(unused)]
pub fn setup_camera_system(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3::new(12.0, 3.0, 24.0),
            ..default()
        },
        projection: Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::PI / 2.0,
            ..default()
        }),
        ..default()
    })
    .insert(CameraComponent);
}

pub fn setup_light_system(
    mut commands: Commands,
) {
    // directional 'sun' light
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: false,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(8.0, 5.0, 8.0),
    //         rotation: Quat::from_rotation_x(-std::f32::consts::PI / 3.),
    //         ..default()
    //     },
    //     // The default cascade config is designed to handle large scenes.
    //     // As this example has a much smaller world, we can tighten the shadow
    //     // bounds for better visual quality.
    //     cascade_shadow_config: CascadeShadowConfigBuilder {
    //         first_cascade_far_bound: 2.0,
    //         maximum_distance: 100.0,
            
    //         ..default()
    //     }
    //     .into(),
    //     ..default()
    // });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0
    });
}