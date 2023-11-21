use std::collections::HashMap;
use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh}};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, Friction, CoefficientCombineRule};
use noise::{Perlin, NoiseFn};

use crate::{CHUNK_WIDTH, CHUNK_HEIGHT, plugins::world::WorldMap};

use super::components::{ChunkComponent, BlockType};

pub fn generate_chunk_data(
    perlin: Perlin,
    position: (i32, i32),
    world_map: &mut ResMut<WorldMap>,
) {
    let mut blocks = vec![vec![vec![BlockType::Air; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                let mut block_to_assign = BlockType::Air;

                // let octave0 = perlin.get([(x as f64 + position.0 as f64 * 16.0) * 0.05, (z as f64 + position.1 as f64 * 16.0) * 0.05]) as f32 * 4.0;
                // let octave1 = perlin.get([(y as f64 + position.0 as f64 * 16.0) * 0.05, (z as f64 + position.1 as f64 * 16.0) * 0.05]) as f32 * 4.0;
                // let octave2 = perlin.get([(x as f64 + position.0 as f64 * 16.0) * 0.05, (y as f64 + position.1 as f64 * 16.0) * 0.05]) as f32 * 4.0;

                // let density = (octave0 + octave1 + octave2).floor();

                // if density < 1.0 {
                //     block_to_assign = BlockType::Stone;
                // }

                let octave0 = perlin.get([(x as f64 + position.0 as f64 * 16.0) * 0.01, (z as f64 + position.1 as f64 * 16.0) * 0.01]) as f32 * 20.0;
                let octave1 = perlin.get([(x as f64 + position.0 as f64 * 16.0) * 0.05, (z as f64 + position.1 as f64 * 16.0) * 0.05]) as f32 * 4.0;
                let octave2 = perlin.get([(x as f64 + position.0 as f64 * 16.0) * 0.1, (z as f64 + position.1 as f64 * 16.0) * 0.1]) as f32;
                let height = (octave0 + octave1 + octave2 + 64.0).floor();

                if (y as f32) < height && (y as f32) > height/2.0 {
                    block_to_assign = BlockType::Dirt;
                }
                else if (y as f32) <= height / 2.0
                {
                    block_to_assign = BlockType::Stone;
                }
                else if y == height as usize {
                    block_to_assign = BlockType::Grass;
                }

                if y as f32 == height && height <= 50.0 {
                    block_to_assign = BlockType::Sand;
                }

                if y as f32 == 49.0 {
                    block_to_assign = BlockType::Water;
                }

                blocks[x][y][z] = block_to_assign;
            }
        }
    }

    world_map.chunks.insert((position.0, position.1), blocks.clone());
}

pub fn generate_chunk_mesh(
    world_map: &mut ResMut<WorldMap>,
    position: (i32, i32),
) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut verticies: Vec<[f32; 3]> = vec![];
    let mut indices: Vec<u32> = vec![];
    let mut normals: Vec<Vec3> = vec![];
    let mut uvs: Vec<Vec2> = vec![];

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT{
            for z in 0..CHUNK_WIDTH {
                generate_block(&mut verticies, &mut indices, &mut normals, &mut uvs, &world_map.chunks, &(x as i32,y as i32,z as i32), &position);
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verticies);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}

pub fn build_chunk(
    commands: &mut Commands,
    world_map: &mut ResMut<WorldMap>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    position: (i32, i32),
) {
    if world_map.chunk_entities.contains_key(&position) { // if there's a spawned chunk, we remove it
        commands.entity(world_map.chunk_entities[&position]).despawn();
        world_map.chunk_entities.remove(&position);
    }

    let texture_handle = asset_server.load("blocks.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        unlit: true,
        ..default()
    });

    let mesh = generate_chunk_mesh(world_map, position);

    let chunk = commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh.clone()),
        material: material_handle,
        transform: Transform::from_translation(Vec3::new(position.0 as f32 * CHUNK_WIDTH as f32, 0.0, position.1 as f32  * CHUNK_WIDTH as f32)),
        ..default()
    }).insert(ChunkComponent {
        position,
        blocks: world_map.chunks[&position].clone(),
    }).id();

    // adding a collider, so the world is actually walkable
    commands.entity(chunk)
        .insert(Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert(Friction {
        coefficient: 0.0,
        combine_rule: CoefficientCombineRule::Min,
    });

    world_map.chunk_entities.insert(position, chunk);
}

fn generate_block(
    verticies: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    normals: &mut Vec<Vec3>,
    uvs: &mut Vec<Vec2>,
    chunks: &HashMap<(i32,i32), Vec<Vec<Vec<BlockType>>>>,
    block_position: &(i32,i32,i32),
    chunk_position: &(i32, i32),
) {
    let block = block_at_position(chunks, *block_position, *chunk_position);

    let (x,y,z) = (block_position.0 as f32, block_position.1 as f32, block_position.2 as f32);
    if block == BlockType::Air {
        return;
    }

    // right side
    if block_at_position(chunks, (x as i32 +1, y as i32, z as i32), *chunk_position) == BlockType::Air {

        verticies.extend([
            [x + 1.0, y + 1.0, z + 0.0],
            [x + 1.0, y + 1.0, z + 1.0],
            [x + 1.0, y + 0.0, z + 1.0],
            [x + 1.0, y + 0.0, z + 0.0],
        ]);
    
        normals.extend([Vec3::NEG_X, Vec3::NEG_X, Vec3::NEG_X, Vec3::NEG_X]);
    
        add_indices(indices, (verticies.len() - 4) as u32);
        uvs.extend(block.uvs().right);
    }

    //left side
    if block_at_position(chunks, (x as i32 -1, y as i32, z as i32), *chunk_position) == BlockType::Air {

        verticies.extend([
            [x + 0.0, y + 1.0, z + 1.0],
            [x + 0.0, y + 1.0, z + 0.0],
            [x + 0.0, y + 0.0, z + 0.0],
            [x + 0.0, y + 0.0, z + 1.0]
        ]);

        normals.extend([Vec3::X, Vec3::X, Vec3::X, Vec3::X]);

        add_indices(indices, (verticies.len() - 4) as u32);
        uvs.extend(block.uvs().left);
    }

    //back side
    if block_at_position(chunks, (x as i32, y as i32, z as i32 -1), *chunk_position) == BlockType::Air {

        verticies.extend([
            [x + 0.0, y + 1.0, z + 0.0],
            [x + 1.0, y + 1.0, z + 0.0],
            [x + 1.0, y + 0.0, z + 0.0],
            [x + 0.0, y + 0.0, z + 0.0],
        ]);

        normals.extend([Vec3::NEG_Z, Vec3::NEG_Z, Vec3::NEG_Z, Vec3::NEG_Z]);

        add_indices(indices, (verticies.len() - 4) as u32);
        uvs.extend(block.uvs().back);
    }

    //front side
    if block_at_position(chunks, (x as i32, y as i32, z as i32 +1), *chunk_position) == BlockType::Air {

        verticies.extend([
            [x + 1.0, y + 1.0, z + 1.0],
            [x + 0.0, y + 1.0, z + 1.0],
            [x + 0.0, y + 0.0, z + 1.0],
            [x + 1.0, y + 0.0, z + 1.0],
        ]);

        normals.extend([Vec3::Z, Vec3::Z, Vec3::Z, Vec3::Z]);

        add_indices(indices, (verticies.len() - 4) as u32);
        uvs.extend(block.uvs().front);
    }

    //bottom side
    if block_at_position(chunks, (x as i32, y as i32 -1, z as i32), *chunk_position) == BlockType::Air {

        verticies.extend([
            [x + 0.0, y + 0.0, z + 1.0],
            [x + 0.0, y + 0.0, z + 0.0],
            [x + 1.0, y + 0.0, z + 0.0],
            [x + 1.0, y + 0.0, z + 1.0]
        ]);

        normals.extend([Vec3::NEG_Y, Vec3::NEG_Y, Vec3::NEG_Y, Vec3::NEG_Y]);

        add_indices(indices, (verticies.len() - 4) as u32);
        uvs.extend(block.uvs().bottom);
    }

    //top side
    if block_at_position(chunks, (x as i32, y as i32 +1, z as i32), *chunk_position) == BlockType::Air {

        verticies.extend([
            [x + 1.0, y + 1.0, z + 1.0],
            [x + 1.0, y + 1.0, z + 0.0],
            [x + 0.0, y + 1.0, z + 0.0],
            [x + 0.0, y + 1.0, z + 1.0]
        ]);

        normals.extend([Vec3::Y, Vec3::Y, Vec3::Y, Vec3::Y]);

        add_indices(indices, (verticies.len() - 4) as u32);
        uvs.extend(block.uvs().top);
    }
}

fn add_indices(
    indices: &mut Vec<u32>,
    base_index: u32,
) {
    indices.extend([base_index, base_index + 1, base_index + 2]);
    indices.extend([base_index, base_index + 2, base_index + 3]);
}

fn block_at_position(
    chunks: &HashMap<(i32,i32), Vec<Vec<Vec<BlockType>>>>,
    block_position: (i32, i32, i32),
    chunk_position: (i32, i32),
) -> BlockType {

    let mut new_position: (i32,i32,i32) = block_position;
    let mut new_chunk_position: (i32,i32) = chunk_position;

    if block_position.1 < 0 || block_position.1 > CHUNK_HEIGHT as i32 {
        return BlockType::Dirt;
    }

    if block_position.0 > CHUNK_WIDTH as i32 - 1 {
        new_position.0 = 0;
        new_chunk_position.0 += 1;
    } else if block_position.0 < 0 {
        new_position.0 = CHUNK_WIDTH as i32 - 1;
        new_chunk_position.0 -= 1;
    }

    if block_position.2 > CHUNK_WIDTH as i32 - 1 {
        new_position.2 = 0;
        new_chunk_position.1 += 1;
    } else if block_position.2 < 0 {
        new_position.2 = CHUNK_WIDTH as i32 - 1;
        new_chunk_position.1 -= 1;
    }

    if chunks.contains_key(&new_chunk_position) {

        //let index = new_position.0 + new_position.1 * CHUNK_HEIGHT as i32 + new_position.2 * CHUNK_WIDTH as i32;
        return chunks[&new_chunk_position][new_position.0 as usize][new_position.1 as usize][new_position.2 as usize].clone();
    }

    return BlockType::Dirt;
}