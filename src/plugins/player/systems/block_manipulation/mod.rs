use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::plugins::player::components::{Player, PlayerCamera};
use crate::plugins::world::{ChunkQueue, chunk::components::BlockType};
use crate::plugins::world::systems::enque_chunk;
use crate::{CHUNK_WIDTH, plugins::world::WorldMap};


pub fn block_breaking_system(
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    rapier_context: Res<RapierContext>,
    mut world_map: ResMut<WorldMap>,
    buttons: Res<Input<MouseButton>>,
    mut chunk_queue: ResMut<ChunkQueue>,
) {
    let camera_transform = camera_query.single();

    if buttons.just_pressed(MouseButton::Left) {
        let origin = camera_transform.translation;
        let direction: Vec3 = camera_transform.forward();
    
        if let Some((_, intersection)) = rapier_context.cast_ray_and_get_normal(origin, direction, 10.0, true, QueryFilter::exclude_dynamic()) {
            let hit = (intersection.point - intersection.normal * 0.5).floor();
            let chunk_pos = ((hit.x / CHUNK_WIDTH as f32).floor() as i32, (hit.z / CHUNK_WIDTH as f32).floor() as i32);
            let (x, y, z) = ((hit.x  - (chunk_pos.0 as f32 * 16.0)) as usize,
                                                (hit.y) as usize,
                                                (hit.z - (chunk_pos.1 as f32 * 16.0)) as usize);

            if world_map.chunks[&chunk_pos][x][y][z] != BlockType::Air{
                world_map.chunks.get_mut(&chunk_pos).unwrap()[x][y][z] = BlockType::Air;
            }

            enque_chunk(&mut chunk_queue, chunk_pos);
            // this is cringe, TODO: rework
            enque_chunk(&mut chunk_queue, (chunk_pos.0-1, chunk_pos.1));
            enque_chunk(&mut chunk_queue, (chunk_pos.0+1, chunk_pos.1));
            enque_chunk(&mut chunk_queue, (chunk_pos.0, chunk_pos.1-1));
            enque_chunk(&mut chunk_queue, (chunk_pos.0, chunk_pos.1+1));
        }
    }
}

pub fn block_placing_system(
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<Player>)>,
    rapier_context: Res<RapierContext>,
    mut world_map: ResMut<WorldMap>,
    buttons: Res<Input<MouseButton>>,
    mut chunk_queue: ResMut<ChunkQueue>,
) {
    let camera_transform = camera_query.single();

    if buttons.just_pressed(MouseButton::Right) {
        let origin = camera_transform.translation;
        let direction: Vec3 = camera_transform.forward();
    
        if let Some((_, intersection)) = rapier_context.cast_ray_and_get_normal(origin, direction, 10.0, true, QueryFilter::exclude_dynamic()) {
            let hit = (intersection.point + intersection.normal * 0.5).floor();
            let chunk_pos = ((hit.x / CHUNK_WIDTH as f32).floor() as i32, (hit.z / CHUNK_WIDTH as f32).floor() as i32);
            let (x, y, z) = ((hit.x  - (chunk_pos.0 as f32 * 16.0)) as usize,
                                                (hit.y) as usize,
                                                (hit.z - (chunk_pos.1 as f32 * 16.0)) as usize);

            if world_map.chunks[&chunk_pos][x][y][z] == BlockType::Air {
                world_map.chunks.get_mut(&chunk_pos).unwrap()[x][y][z] = BlockType::Stone;
            }

            enque_chunk(&mut chunk_queue, chunk_pos);
            // this is cringe, TODO: rework
            enque_chunk(&mut chunk_queue, (chunk_pos.0-1, chunk_pos.1));
            enque_chunk(&mut chunk_queue, (chunk_pos.0+1, chunk_pos.1));
            enque_chunk(&mut chunk_queue, (chunk_pos.0, chunk_pos.1-1));
            enque_chunk(&mut chunk_queue, (chunk_pos.0, chunk_pos.1+1));
        }
    }
}

