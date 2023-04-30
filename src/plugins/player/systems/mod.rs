use bevy::{prelude::*, input::mouse::MouseMotion, time::Stopwatch};
use bevy_rapier3d::prelude::*;

use super::components::{Player, PlayerCamera, JumpDuration};

pub fn movement_system(
    mut player_query: Query<(&mut Velocity, &Player), Without<PlayerCamera>>,
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut velocity, player) = player_query.single_mut();
    let camera_transform = camera_query.single();

    let mut x_axis = 0.0;
    if keyboard.pressed(KeyCode::D) {
        x_axis = 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        x_axis = -1.0;
    }

    let mut z_axis = 0.0;
    if keyboard.pressed(KeyCode::S) {
        z_axis = 1.0;
    }
    if keyboard.pressed(KeyCode::W) {
        z_axis = -1.0;
    }   

    let movement_input = Quat::from_rotation_y(camera_transform.rotation.to_euler(EulerRot::YXZ).0) * Vec3::new(x_axis, 0.0, z_axis);
    let movement_direction = movement_input.normalize_or_zero();

    velocity.linvel = Vec3::new(movement_direction.x * player.speed * time.delta_seconds(), velocity.linvel.y, movement_direction.z * player.speed * time.delta_seconds());
}

pub fn jump_system(
    time: Res<Time>,
    mut player_query: Query<(&mut JumpDuration, &mut Velocity, &Player)>,
    kbd: Res<Input<KeyCode>>,
) {
    // assume we have exactly one player that jumps with Spacebar
    let (mut jump, mut velocity, player) = player_query.single_mut();

    if kbd.just_pressed(KeyCode::Space) {
        jump.time.reset();
    }

    if kbd.pressed(KeyCode::Space) && jump.time.elapsed_secs() < 0.15 {
        jump.time.tick(time.delta());
        velocity.linvel.y = player.jump_force;
    }
}

pub fn camera_rotation_system(
    windows_query: Query<&Window>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut PlayerCamera), Without<Player>>,
    mut ev_motion: EventReader<MouseMotion>,
) {
    let player_transform = player_query.single();

    let mut rotation_move = Vec2::ZERO;
    for ev in ev_motion.iter() {
            rotation_move += ev.delta;
    }

    let (mut camera_transform, mut player_camera) = camera_query.single_mut();

    if rotation_move.length_squared() > 0.0 {
        let window = get_primary_window_size(&windows_query);
        let delta_x = {
            let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
            if player_camera.upside_down { -delta } else { delta }
        };
        let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        camera_transform.rotation = yaw * camera_transform.rotation; // rotate around global y axis
        camera_transform.rotation = camera_transform.rotation * pitch; // rotate around local x axis
    }

    player_camera.focus = player_transform.translation;
    camera_transform.translation = player_camera.focus + Vec3::new(0.0, 1.0, 0.0);
}

fn get_primary_window_size(windows: &Query<&Window>) -> Vec2 {
    let window = windows.single();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

pub fn player_setup_system(
    mut commands: Commands,
) {
    let player = commands.spawn(PbrBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 130.0, 0.0),
            ..Default::default()
        },
        ..default()
    })
    .id();

    commands.entity(player)
        .insert(Player { speed: 400.0, jump_force: 10.0 })
        .insert(JumpDuration { time: Stopwatch::new()})
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED_Z | LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Y)
        .insert(Collider::capsule_y(0.05, 0.4))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, 0.0),
            angvel: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(GravityScale(9.8))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());
}