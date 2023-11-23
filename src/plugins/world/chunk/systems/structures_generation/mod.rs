use bevy::prelude::*;

use crate::{plugins::world::{WorldMap, chunk::components::BlockType}, CHUNK_WIDTH, CHUNK_HEIGHT};

pub fn add_cactus (
    x: usize, y: usize, z: usize,
    mut blocks: Vec<Vec<Vec<BlockType>>>,
) -> Vec<Vec<Vec<BlockType>>> {
    for i in 1..4 {
        if y+i < CHUNK_HEIGHT-1 {
            blocks[x][y+i][z] = BlockType::Cactus;
        }
    }

    blocks
}

pub fn add_tree(
    chunk_pos: (i32, i32),
    x: usize, y: usize, z: usize,
    world_map: &mut ResMut<WorldMap>,
    mut blocks: Vec<Vec<Vec<BlockType>>>,
) -> Vec<Vec<Vec<BlockType>>> {
    blocks[x][y+5][z] = BlockType::Leaves;

    let mut reserved_blocks_x = vec![vec![vec![BlockType::Air; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];
    let mut reserved_blocks_neg_x = vec![vec![vec![BlockType::Air; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];
    let mut reserved_blocks_z = vec![vec![vec![BlockType::Air; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];
    let mut reserved_blocks_neg_z = vec![vec![vec![BlockType::Air; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

    if world_map.reserved_chunk_data.contains_key(&(chunk_pos.0 + 1, chunk_pos.1)) {
        reserved_blocks_x = world_map.reserved_chunk_data[&(chunk_pos.0 + 1, chunk_pos.1)].clone();
    }
    if world_map.reserved_chunk_data.contains_key(&(chunk_pos.0 - 1, chunk_pos.1)) {
        reserved_blocks_neg_x = world_map.reserved_chunk_data[&(chunk_pos.0 - 1, chunk_pos.1)].clone();
    }
    if world_map.reserved_chunk_data.contains_key(&(chunk_pos.0, chunk_pos.1 + 1)) {
        reserved_blocks_z = world_map.reserved_chunk_data[&(chunk_pos.0, chunk_pos.1 + 1)].clone();
    }
    if world_map.reserved_chunk_data.contains_key(&(chunk_pos.0, chunk_pos.1 - 1)) {
        reserved_blocks_neg_z = world_map.reserved_chunk_data[&(chunk_pos.0, chunk_pos.1 - 1)].clone();
    }

    let mut need_x = false;
    let mut need_neg_x = false;
    let mut need_z = false;
    let mut need_neg_z = false;

    for i in 1..5 {
        if y+i < CHUNK_HEIGHT-1 {
            blocks[x][y+i][z] = BlockType::WoodLog;
        }

        for j in 1..5-i-1 {

            if x+j >= CHUNK_WIDTH {
                reserved_blocks_x[x+j-CHUNK_WIDTH][y+i+2][z] = BlockType::Leaves;
                need_x = true;
            }
            else {
                blocks[x+j][y+i+2][z] = BlockType::Leaves;
            }

            if (x as i32 - j as i32) < 0 {
                reserved_blocks_neg_x[(x as i32 - j as i32 + CHUNK_WIDTH as i32) as usize][y+i+2][z] = BlockType::Leaves;
                need_neg_x = true;
            }
            else {
                blocks[x-j][y+i+2][z] = BlockType::Leaves;
            }

            if z+j >= CHUNK_WIDTH {
                reserved_blocks_z[x][y+i+2][z+j-CHUNK_WIDTH] = BlockType::Leaves;
                need_z = true;
            }
            else {
                blocks[x][y+i+2][z+j] = BlockType::Leaves;
            }

            if (z as i32 - j as i32) < 0 {
                reserved_blocks_neg_z[x][y+i+2][(z as i32 - j as i32 + CHUNK_WIDTH as i32) as usize] = BlockType::Leaves;
                need_neg_z = true;
            }
            else {
                blocks[x][y+i+2][z-j] = BlockType::Leaves;
            }
        }
    }

    if need_x {world_map.reserved_chunk_data.insert((chunk_pos.0 + 1, chunk_pos.1), reserved_blocks_x);}
    if need_neg_x {world_map.reserved_chunk_data.insert((chunk_pos.0 - 1, chunk_pos.1), reserved_blocks_neg_x);}
    if need_z {world_map.reserved_chunk_data.insert((chunk_pos.0, chunk_pos.1 + 1), reserved_blocks_z);}
    if need_neg_z {world_map.reserved_chunk_data.insert((chunk_pos.0, chunk_pos.1 - 1), reserved_blocks_neg_z);}

    blocks
}