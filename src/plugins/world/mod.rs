use std::collections::HashMap;

use bevy::prelude::*;
use noise::Perlin;

use crate::{CHUNK_WIDTH, CHUNK_HEIGHT};

use self::{systems::{generate_world_system, generate_chunks_from_player_movement, deque_chunks, unload_far_chunks}, chunk::components::BlockType};

mod components;

pub(crate) mod chunk;
pub(crate) mod systems;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WorldMap { chunks: HashMap::new(), chunk_entities: HashMap::new(), water_chunk_entities: HashMap::new(), reserved_chunk_data: HashMap::new() })
            .insert_resource(ChunkQueue { queue: vec![], is_next_ready: true })
            .add_systems(Startup, generate_world_system)
            .add_systems(Update, (generate_chunks_from_player_movement, deque_chunks, unload_far_chunks));
    }
}

#[derive(Resource)]
pub struct WorldMap {
    pub chunks: HashMap<(i32, i32), [BlockType; CHUNK_WIDTH*CHUNK_HEIGHT*CHUNK_WIDTH]>,
    pub chunk_entities: HashMap<(i32,i32), Entity>,
    pub water_chunk_entities: HashMap<(i32, i32), Entity>,
    pub reserved_chunk_data: HashMap<(i32, i32), [BlockType; CHUNK_WIDTH*CHUNK_HEIGHT*CHUNK_WIDTH]>,
}

#[derive(Resource)]
pub struct SeededPerlin {
    pub seed: u32,
    pub terrain_noise: Perlin,
    pub tree_noise: Perlin,
    pub temperature_noise: Perlin,
    pub moisture_noise: Perlin,
}

#[derive(Resource)]
pub struct ChunkQueue {
    pub queue: Vec<(i32, i32)>,
    pub is_next_ready: bool,
}

// #[derive(Resource)]
// pub struct ChunkQueueConfig {
//     timer: Timer,
// }