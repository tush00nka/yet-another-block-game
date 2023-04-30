use bevy::prelude::Plugin;

use self::systems::{movement_system, jump_system, camera_rotation_system, player_setup_system};

pub(crate) mod systems;
pub(crate) mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_startup_system(player_setup_system)
            .add_systems((movement_system, jump_system))
            .add_system(camera_rotation_system);
    }
}