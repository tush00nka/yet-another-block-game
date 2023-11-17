use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct ChunkPlugin;

#[allow(unused)]
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        //app.add_startup_system(systems::generate_chunk_mesh);
    }
}