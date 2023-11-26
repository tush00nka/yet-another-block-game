use std::time::SystemTime;

use bevy::prelude::*;
use noise::Perlin;

use crate::{RENDER_DISTANCE, CHUNK_WIDTH, plugins::player::components::Player};

use super::{chunk::systems::{generate_chunk_data, build_chunk}, WorldMap, SeededPerlin, ChunkQueue};

pub fn generate_world_system(
    mut commands: Commands,
) {
    let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("[E] SystemTime before UNIX EPOCH!").as_secs() as u32;
    let terrain_perlin = Perlin::new(seed);
    let tree_perlin = Perlin::new(seed*2);
    let temperature_perlin = Perlin::new(seed+20);
    let moisture_perlin = Perlin::new(seed+30);

    commands.insert_resource(SeededPerlin { seed: seed, terrain_noise: terrain_perlin, tree_noise: tree_perlin, temperature_noise: temperature_perlin, moisture_noise: moisture_perlin});
}

pub fn generate_chunks_from_player_movement(
    player_query: Query<&Transform, With<Player>>,
    mut world_map: ResMut<WorldMap>,
    perlin: Res<SeededPerlin>,
    mut chunk_queue: ResMut<ChunkQueue>,
) {
    let player_transform = player_query.single();
    let (chunk_x, chunk_z) = ((player_transform.translation.x / CHUNK_WIDTH as f32).round() as i32, (player_transform.translation.z / CHUNK_WIDTH as f32).round() as i32);

    let render_distance = RENDER_DISTANCE;

    for x in -(render_distance + 1)..(render_distance + 1) {
        for z in -(render_distance + 1)..(render_distance + 1) {
            if !world_map.chunks.contains_key(&(chunk_x + x, chunk_z + z)) {
                generate_chunk_data(&perlin, (chunk_x + x, chunk_z + z), &mut world_map);
            }
        }
    }

    for x in -render_distance..render_distance {
        for z in -render_distance..render_distance {
            if !chunk_queue.queue.contains(&(chunk_x + x, chunk_z + z)) && !world_map.chunk_entities.contains_key(&(chunk_x + x, chunk_z + z)) {
                enque_chunk(&mut chunk_queue, (chunk_x + x, chunk_z + z));
            }
        }
    }
}

pub fn unload_far_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut world_map: ResMut<WorldMap>,
) {
    let player_transform = player_query.single();
    let (chunk_x, chunk_z) = ((player_transform.translation.x / CHUNK_WIDTH as f32).round() as i32, (player_transform.translation.z / CHUNK_WIDTH as f32).round() as i32);
    
    for chunk in world_map.chunk_entities.clone().iter() {
        let chunk_position = chunk.0.clone();

        if (chunk_x - chunk_position.0).abs() > RENDER_DISTANCE ||  (chunk_z - chunk_position.1).abs() > RENDER_DISTANCE {
            commands.entity(*chunk.1).despawn();
            world_map.chunk_entities.remove(&chunk_position);
        }
    }

    for chunk in world_map.water_chunk_entities.clone().iter() {
        let chunk_position = chunk.0.clone();

        if (chunk_x - chunk_position.0).abs() > RENDER_DISTANCE ||  (chunk_z - chunk_position.1).abs() > RENDER_DISTANCE {
            commands.entity(*chunk.1).despawn();
            world_map.water_chunk_entities.remove(&chunk_position);
        }
    }
}

pub fn enque_chunk(
    chunk_queue: &mut ResMut<ChunkQueue>,
    position: (i32,i32),
) {
    chunk_queue.queue.push(position);
}

pub fn deque_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world_map: ResMut<WorldMap>,
    mut chunk_queue: ResMut<ChunkQueue>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
) {
    if chunk_queue.queue.len() > 0 && chunk_queue.is_next_ready { 
        chunk_queue.is_next_ready = false;
        if let Ok(player_transform) = player_query.get_single() {
            let position = ((player_transform.translation.x / CHUNK_WIDTH as f32).floor() as i32, (player_transform.translation.z / CHUNK_WIDTH as f32).floor() as i32);
            let closest_index = get_closest_chunk_from_queue(&chunk_queue.queue, position);
            let chunk = chunk_queue.queue[closest_index];

            if world_map.chunks.contains_key(&chunk) {
                chunk_queue.queue.remove(closest_index);
                chunk_queue.is_next_ready = build_chunk(&mut commands, &mut world_map, &mut meshes, &mut materials, asset_server, chunk);
            }
        }
    }
}

fn get_closest_chunk_from_queue(
    queue: &Vec<(i32, i32)>,
    position: (i32,i32),
) -> usize {
    let mut closest = (100, 100);
    let mut index = 0;

    for i in 0..queue.len() {
        let distance_chunk = (((queue[i].0 - position.0).pow(2) + (queue[i].1 - position.1).pow(2)) as f32).sqrt();
        let distance_closest = (((closest.0 - position.0).pow(2) + (closest.1 - position.1).pow(2)) as f32).sqrt();
        if distance_chunk < distance_closest {
            closest = queue[i];
            index = i;
        }
    }

    index
}