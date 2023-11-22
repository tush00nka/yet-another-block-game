use bevy::{prelude::*, input::mouse::MouseMotion, ecs::event::ManualEventReader};




pub(crate) mod block_manipulation;
pub(crate) mod player_movement;

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}