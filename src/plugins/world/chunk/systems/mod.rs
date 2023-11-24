use std::collections::HashMap;
use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh}};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, Friction, CoefficientCombineRule};
use noise::{Perlin, NoiseFn};
use rand::{rngs::StdRng, SeedableRng, Rng};

use crate::{CHUNK_WIDTH, CHUNK_HEIGHT, plugins::world::{WorldMap, SeededPerlin}};

use self::structures_generation::{add_tree, add_cactus};

use super::components::{ChunkComponent, BlockType};

mod structures_generation;

pub fn generate_chunk_data(
    perlin: &Res<SeededPerlin>,
    position: (i32, i32),
    world_map: &mut ResMut<WorldMap>,
) {
    let mut blocks = vec![vec![vec![BlockType::Air; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH];

    let mut tree_positions = vec![];

    let mut random = StdRng::seed_from_u64(perlin.seed as u64);

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                let mut block_to_assign = BlockType::Air;

                let height = height_by_coords(perlin.terrain_noise, x, z, position);
                let tree_value = perlin.tree_noise.get([(x as f64 + position.0 as f64 * 16.0) * 0.01, (z as f64 + position.1 as f64 * 16.0) * 0.01]) as f32;

                let temperature = perlin.temperature_noise.get([(x as f64 + position.0 as f64 * 16.0) * 0.001, (z as f64 + position.1 as f64 * 16.0) * 0.001]) as f32 * 10.;
                let moisture = perlin.moisture_noise.get([(x as f64 + position.0 as f64 * 16.0) * 0.001, (z as f64 + position.1 as f64 * 16.0) * 0.001]) as f32 * 10.;

                if y < height && y > height/2 {
                    if temperature > 0.5 && moisture < 0.5 {
                        block_to_assign = BlockType::Sand;
                    }
                    else {
                        block_to_assign = BlockType::Dirt;
                    }
                }
                else if y <= height / 2
                {
                    block_to_assign = BlockType::Stone;
                }
                else if y == height {
                    if temperature > 0.5 && moisture < 0.5 {
                        block_to_assign = BlockType::Sand;
                    }
                    else {
                        block_to_assign = BlockType::Grass;
                    }
                }

                if y == height && height <= 50 {
                    block_to_assign = BlockType::Sand;
                }

                if y < 50 && y > height {
                    block_to_assign = BlockType::Water;
                }

                let tree_chance = random.gen_range(-1.0..tree_value.abs());
                if tree_value > 0.5 && tree_chance > 0.8 && y == height && height >= 50 {
                    tree_positions.push((x,y,z));
                }

                blocks[x][y][z] = block_to_assign;   
            }
        }
    }

    for pos in tree_positions.iter() {
        let temperature = perlin.temperature_noise.get([(pos.0 as f64 + position.0 as f64 * 16.0) * 0.001, (pos.2 as f64 + position.1 as f64 * 16.0) * 0.001]) as f32 * 10.;
        let moisture = perlin.moisture_noise.get([(pos.0 as f64 + position.0 as f64 * 16.0) * 0.001, (pos.2 as f64 + position.1 as f64 * 16.0) * 0.001]) as f32 * 10.;
        
        if temperature > 0.5 && moisture < 0.5 {
            blocks = add_cactus(random.gen_range(2..5), pos.0, pos.1, pos.2, blocks);
        }
        else {
            blocks = add_tree(random.gen_range(3..6), position, pos.0, pos.1, pos.2, world_map, blocks);
        }
    }

    world_map.chunks.insert(position, blocks);
}

fn height_by_coords(
    perlin: Perlin,
    x: usize, z: usize,
    chunk_position: (i32, i32),
) -> usize{
    let octave0 = perlin.get([(x as f64 + chunk_position.0 as f64 * 16.0) * 0.01, (z as f64 + chunk_position.1 as f64 * 16.0) * 0.01]) as f32 * 20.0;
    let octave1 = perlin.get([(x as f64 + chunk_position.0 as f64 * 16.0) * 0.05, (z as f64 + chunk_position.1 as f64 * 16.0) * 0.05]) as f32 * 4.0;
    let octave2 = perlin.get([(x as f64 + chunk_position.0 as f64 * 16.0) * 0.1, (z as f64 + chunk_position.1 as f64 * 16.0) * 0.1]) as f32;
    let height = (octave0 + octave1 + octave2 + 64.0).floor();
    height as usize
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
    let mut colors: Vec<[f32; 4]> = vec![];

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT{
            for z in 0..CHUNK_WIDTH {
                generate_block(&mut verticies, &mut indices, &mut normals, &mut uvs, &world_map.chunks, &(x as i32,y as i32,z as i32), &position);
            }
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verticies.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    calculate_ao(&mut colors, position, &world_map.chunks);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    mesh
}

fn calculate_ao(
    colors: &mut Vec<[f32; 4]>,
    chunk_position: (i32, i32),
    chunks: &HashMap<(i32,i32), Vec<Vec<Vec<BlockType>>>>,
) {
    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                if chunks[&chunk_position][x][y][z] != BlockType::Air {
                    let x = x as i32;
                    let y = y as i32;
                    let z = z as i32;

                    if block_at_position(chunks, (x as i32 + 1, y as i32, z as i32), chunk_position).is_transparent()
                    && !block_at_position(chunks, (x as i32, y as i32, z as i32), chunk_position).is_transparent() {
                        let neighbors = [
                            block_at_position(chunks, (x + 1, y, z - 1), chunk_position),
                            block_at_position(chunks, (x + 1, y - 1, z - 1), chunk_position),
                            block_at_position(chunks, (x + 1, y - 1, z), chunk_position),
                            block_at_position(chunks, (x + 1, y - 1, z + 1), chunk_position),
                            block_at_position(chunks, (x + 1, y, z + 1), chunk_position),
                            block_at_position(chunks, (x + 1, y + 1, z + 1), chunk_position),
                            block_at_position(chunks, (x + 1, y + 1, z), chunk_position),
                            block_at_position(chunks, (x + 1, y + 1, z - 1), chunk_position),
                        ];
                        let darks = side_ao(neighbors);
                        colors.append(&mut vec![[darks[0], darks[0], darks[0], 1.],
                                                [darks[1], darks[1], darks[1], 1.],
                                                [darks[2], darks[2], darks[2], 1.],
                                                [darks[3], darks[3], darks[3], 1.]]);
                    }

                    if block_at_position(chunks, (x as i32 - 1, y as i32, z as i32), chunk_position).is_transparent()
                    && !block_at_position(chunks, (x as i32, y as i32, z as i32), chunk_position).is_transparent() {
                
                        let neighbors = [
                            block_at_position(chunks, (x - 1, y, z + 1), chunk_position),
                            block_at_position(chunks, (x - 1, y - 1, z + 1), chunk_position),
                            block_at_position(chunks, (x - 1, y - 1, z), chunk_position),
                            block_at_position(chunks, (x - 1, y - 1, z - 1), chunk_position),
                            block_at_position(chunks, (x - 1, y, z - 1), chunk_position),
                            block_at_position(chunks, (x - 1, y + 1, z - 1), chunk_position),
                            block_at_position(chunks, (x - 1, y + 1, z), chunk_position),
                            block_at_position(chunks, (x - 1, y + 1, z + 1), chunk_position),
                        ];
                        let darks = side_ao(neighbors);
                        colors.append(&mut vec![[darks[0], darks[0], darks[0], 1.],
                                                [darks[1], darks[1], darks[1], 1.],
                                                [darks[2], darks[2], darks[2], 1.],
                                                [darks[3], darks[3], darks[3], 1.]]);
                    }
                    if block_at_position(chunks, (x as i32, y as i32, z as i32 + 1), chunk_position).is_transparent()
                    && !block_at_position(chunks, (x as i32, y as i32, z as i32), chunk_position).is_transparent() {
                        let neighbors = [
                            block_at_position(chunks, (x + 1, y, z + 1), chunk_position),
                            block_at_position(chunks, (x + 1, y - 1, z + 1), chunk_position),
                            block_at_position(chunks, (x, y - 1, z + 1), chunk_position),
                            block_at_position(chunks, (x - 1, y - 1, z + 1), chunk_position),
                            block_at_position(chunks, (x - 1, y, z + 1), chunk_position),
                            block_at_position(chunks, (x - 1, y + 1, z + 1), chunk_position),
                            block_at_position(chunks, (x, y + 1, z + 1), chunk_position),
                            block_at_position(chunks, (x + 1, y + 1, z + 1), chunk_position),
                        ];
                        let darks = side_ao(neighbors);
                        colors.append(&mut vec![[darks[0], darks[0], darks[0], 1.],
                                                [darks[1], darks[1], darks[1], 1.],
                                                [darks[2], darks[2], darks[2], 1.],
                                                [darks[3], darks[3], darks[3], 1.]]);
                    }
                    if block_at_position(chunks, (x as i32, y as i32, z as i32 - 1), chunk_position).is_transparent()
                    && !block_at_position(chunks, (x as i32, y as i32, z as i32), chunk_position).is_transparent() {

                        let neighbors = [
                            block_at_position(chunks, (x - 1, y, z - 1), chunk_position),
                            block_at_position(chunks, (x - 1, y - 1, z - 1), chunk_position),
                            block_at_position(chunks, (x, y - 1, z - 1), chunk_position),
                            block_at_position(chunks, (x + 1, y - 1, z - 1), chunk_position),
                            block_at_position(chunks, (x + 1, y, z - 1), chunk_position),
                            block_at_position(chunks, (x + 1, y + 1, z - 1), chunk_position),
                            block_at_position(chunks, (x, y + 1, z - 1), chunk_position),
                            block_at_position(chunks, (x - 1, y + 1, z - 1), chunk_position),
                        ];
                        let darks = side_ao(neighbors);
                        colors.append(&mut vec![[darks[0], darks[0], darks[0], 1.],
                                                [darks[1], darks[1], darks[1], 1.],
                                                [darks[2], darks[2], darks[2], 1.],
                                                [darks[3], darks[3], darks[3], 1.]]);
                    }

                    if block_at_position(chunks, (x as i32, y as i32 + 1, z as i32), chunk_position).is_transparent()
                    && !block_at_position(chunks, (x as i32, y as i32, z as i32), chunk_position).is_transparent() {
                
                        let neighbors = [
                            block_at_position(chunks, (x, y + 1, z + 1), chunk_position),
                            block_at_position(chunks, (x - 1, y + 1, z + 1), chunk_position),
                            block_at_position(chunks, (x - 1, y + 1, z), chunk_position),
                            block_at_position(chunks, (x - 1, y + 1, z - 1), chunk_position),
                            block_at_position(chunks, (x, y + 1, z - 1), chunk_position),
                            block_at_position(chunks, (x + 1, y + 1, z - 1), chunk_position),
                            block_at_position(chunks, (x + 1, y + 1, z), chunk_position),
                            block_at_position(chunks, (x + 1, y + 1, z + 1), chunk_position),   
                        ];
                        let darks = side_ao(neighbors);
                        colors.append(&mut vec![
                            [darks[0], darks[0], darks[0], 1.],
                            [darks[1], darks[1], darks[1], 1.],
                            [darks[2], darks[2], darks[2], 1.],
                            [darks[3], darks[3], darks[3], 1.],
                        ]);
                    }

                    if block_at_position(chunks, (x as i32, y as i32 - 1, z as i32), chunk_position).is_transparent()
                    && !block_at_position(chunks, (x as i32, y as i32, z as i32), chunk_position).is_transparent() {
                
                        let neighbors = [
                            block_at_position(chunks, (x - 1, y - 1, z), chunk_position),
                            block_at_position(chunks, (x - 1, y - 1, z + 1), chunk_position),
                            block_at_position(chunks, (x, y - 1, z + 1), chunk_position),
                            block_at_position(chunks, (x + 1, y - 1, z + 1), chunk_position),
                            block_at_position(chunks, (x + 1, y - 1, z), chunk_position),
                            block_at_position(chunks, (x + 1, y - 1, z - 1), chunk_position),
                            block_at_position(chunks, (x, y - 1, z - 1), chunk_position),
                            block_at_position(chunks, (x - 1, y - 1, z - 1), chunk_position),
                        ];
                        let darks = side_ao(neighbors);
                        colors.append(&mut vec![[darks[0], darks[0], darks[0], 1.],
                                                [darks[1], darks[1], darks[1], 1.],
                                                [darks[2], darks[2], darks[2], 1.],
                                                [darks[3], darks[3], darks[3], 1.]]); 
                    }
            
                }
            }
        }
    }
}

fn ao_value(side1: bool, corner: bool, side2: bool) -> u32 {
    match (side1, corner, side2) {
        (true, _, true) => 0,
        (true, true, false) | (false, true, true) => 1,
        (false, false, false) => 3,
        _ => 2,
    }
}

fn side_ao(neighbors: [BlockType; 8]) -> [f32; 4] {
    let ns = [
        neighbors[0].is_transparent(),
        neighbors[1].is_transparent(),
        neighbors[2].is_transparent(),
        neighbors[3].is_transparent(),
        neighbors[4].is_transparent(),
        neighbors[5].is_transparent(),
        neighbors[6].is_transparent(),
        neighbors[7].is_transparent(),
    ];

    [
        1.0 - ao_value(ns[6], ns[7], ns[0]) as f32 * 0.25,
        1.0 - ao_value(ns[4], ns[5], ns[6]) as f32 * 0.25,
        1.0 - ao_value(ns[2], ns[3], ns[4]) as f32 * 0.25,
        1.0 - ao_value(ns[0], ns[1], ns[2]) as f32 * 0.25,

    ]
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

    if world_map.reserved_chunk_data.contains_key(&position) {
        let mut blocks = world_map.chunks[&position].clone();
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_WIDTH {
                    if world_map.reserved_chunk_data[&position][x][y][z] != BlockType::Air {
                        blocks[x][y][z] = world_map.reserved_chunk_data[&position][x][y][z];
                    }
                }
            }
        }
        world_map.chunks.insert(position, blocks);
        world_map.reserved_chunk_data.remove(&position);
    }

    let texture_handle = asset_server.load("blocks.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        unlit: false,
        perceptual_roughness: 0.0,
        reflectance: 0.0,
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
    if block_at_position(chunks, (x as i32 + 1, y as i32, z as i32), *chunk_position).is_transparent()
    && !block_at_position(chunks, (x as i32, y as i32, z as i32), *chunk_position).is_transparent() {

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
    if block_at_position(chunks, (x as i32 - 1, y as i32, z as i32), *chunk_position).is_transparent()
    && !block_at_position(chunks, (x as i32, y as i32, z as i32), *chunk_position).is_transparent() {

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
    if block_at_position(chunks, (x as i32, y as i32, z as i32 - 1), *chunk_position).is_transparent()
    && !block_at_position(chunks, (x as i32, y as i32, z as i32), *chunk_position).is_transparent() {

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
    if block_at_position(chunks, (x as i32, y as i32, z as i32 + 1), *chunk_position).is_transparent()
    && !block_at_position(chunks, (x as i32, y as i32, z as i32), *chunk_position).is_transparent() {

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
    if block_at_position(chunks, (x as i32, y as i32 - 1, z as i32), *chunk_position).is_transparent()
    && !block_at_position(chunks, (x as i32, y as i32, z as i32), *chunk_position).is_transparent() {

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
    if block_at_position(chunks, (x as i32, y as i32 + 1, z as i32), *chunk_position).is_transparent()
    && !block_at_position(chunks, (x as i32, y as i32, z as i32), *chunk_position).is_transparent() {

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