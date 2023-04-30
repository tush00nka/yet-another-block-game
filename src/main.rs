use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, window::PresentMode};
use bevy_rapier3d::prelude::{RapierPhysicsPlugin, NoUserData};
use plugins::{camera::CameraPlugin, world::WorldPlugin, player::PlayerPlugin};

mod plugins;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;
pub const RENDER_DISTANCE: i32 = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "yet another block game".to_string(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}