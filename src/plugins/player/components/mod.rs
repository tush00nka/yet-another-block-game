use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
}

#[derive(Component)]
pub struct PlayerCamera {
    pub focus: Vec3,
    pub upside_down: bool,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        PlayerCamera {
            focus: Vec3::ZERO,
            upside_down: false,
        }
    }
}

#[derive(Component)]
pub struct JumpDuration {
    pub time: Stopwatch,
}