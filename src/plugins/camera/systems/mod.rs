use bevy::prelude::*;

use crate::plugins::player::components::PlayerCamera;

pub fn setup_camera_system(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3::ZERO,
            ..default()
        },
        projection: Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::PI / 2.0,
            ..default()
        }),
        ..default()
    })
    .insert(PlayerCamera::default());
}

pub fn setup_light_system(
    mut commands: Commands,
) {
    commands.insert_resource(ClearColor(Color::hex("8fd3ff").unwrap()));
}