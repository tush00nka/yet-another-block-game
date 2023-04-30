// use std::time::Duration;

use bevy::prelude::*;
use noise::Perlin;

use crate::{RENDER_DISTANCE, CHUNK_WIDTH, plugins::player::components::Player};

use super::{chunk::{systems::{generate_chunk_data, build_chunk}, components::ChunkComponent}, WorldMap, SeededPerlin, ChunkQueue};

pub fn generate_world_system(
    mut commands: Commands,
) {
    let perlin = Perlin::new(123);

    commands.insert_resource(SeededPerlin { noise: perlin });
}

pub fn generate_chunks_from_player_movement(
    player_query: Query<&Transform, With<Player>>,
    mut world_map: ResMut<WorldMap>,
    perlin: Res<SeededPerlin>,
    mut chunk_queue: ResMut<ChunkQueue>,
) {
    let player_transform = player_query.single();
    let (chunk_x, chunk_z) = ((player_transform.translation.x / CHUNK_WIDTH as f32).round() as i32, (player_transform.translation.z / CHUNK_WIDTH as f32).round() as i32);

    for x in -(RENDER_DISTANCE + 1)..(RENDER_DISTANCE + 1) {
        for z in -(RENDER_DISTANCE + 1)..(RENDER_DISTANCE + 1) {
            if !world_map.chunks.contains_key(&(chunk_x + x, chunk_z + z)) {
                generate_chunk_data(perlin.noise, (chunk_x + x, chunk_z + z), &mut world_map);
            }
        }
    }

    for x in -RENDER_DISTANCE..RENDER_DISTANCE {
        for z in -RENDER_DISTANCE..RENDER_DISTANCE {

            if !chunk_queue.queue.contains(&(chunk_x + x, chunk_z + z)) && !world_map.chunk_entities.contains_key(&(chunk_x + x, chunk_z + z)) {
                enque_chunk(&mut chunk_queue, (chunk_x + x, chunk_z + z));
            }
        }
    }
}

pub fn unload_far_chunks(
    mut commands: Commands,
    chunk_query: Query<(&ChunkComponent, Entity)>,
    player_query: Query<&Transform, With<Player>>,
    mut world_map: ResMut<WorldMap>,
) {
    let player_transform = player_query.single();
    let (chunk_x, chunk_z) = ((player_transform.translation.x / CHUNK_WIDTH as f32).round() as i32, (player_transform.translation.z / CHUNK_WIDTH as f32).round() as i32);
    
    for (component, chunk) in chunk_query.iter() {
        if (chunk_x - component.position.0).abs() > RENDER_DISTANCE ||  (chunk_z - component.position.1).abs() > RENDER_DISTANCE {
            world_map.chunk_entities.remove(&component.position);
            commands.entity(chunk).despawn();
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
    mut chunk_query: Query<&Handle<Mesh>, With<ChunkComponent>>,
    asset_server: Res<AssetServer>,
) {
    if chunk_queue.queue.len() > 0 {
        build_chunk(&mut commands, &mut world_map, &mut chunk_query, &mut meshes, &mut materials,  asset_server, chunk_queue.queue[0]);
        chunk_queue.queue.remove(0);
    }
}