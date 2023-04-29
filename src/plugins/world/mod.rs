use std::collections::HashMap;

use bevy::prelude::*;
use noise::Perlin;

use self::{systems::{generate_world_system, generate_chunks_from_player_movement, deque_chunks, unload_far_chunks}, chunk::components::BlockType};

mod components;
mod systems;

mod chunk;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WorldMap { chunks: HashMap::new(), chunk_entities: HashMap::new() })
            .insert_resource(ChunkQueue { queue: vec![] })
            .add_startup_system(generate_world_system)
            .add_system(generate_chunks_from_player_movement)
            .add_system(deque_chunks)
            .add_system(unload_far_chunks);
    }
}

#[derive(Resource)]
pub struct WorldMap {
    pub chunks: HashMap<(i32, i32), Vec<Vec<Vec<BlockType>>>>,
    pub chunk_entities: HashMap<(i32,i32), Entity>,
}

#[derive(Resource)]
pub struct SeededPerlin {
    pub noise: Perlin,
}

#[derive(Resource)]
pub struct ChunkQueue {
    pub queue: Vec<(i32, i32)>,
}

// #[derive(Resource)]
// pub struct ChunkQueueConfig {
//     timer: Timer,
// }