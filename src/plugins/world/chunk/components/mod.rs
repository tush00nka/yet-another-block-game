use bevy::prelude::*;

#[derive(Component)]
pub struct ChunkComponent {
    pub position: (i32,i32),
    pub blocks: Vec<Vec<Vec<BlockType>>>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BlockType {
    Air,
    Dirt,
    Grass,
    Stone,
    Sand,
    Water,
}

impl BlockType {
    pub fn uvs(&self) -> BlockFaces {
        match self {
            BlockType::Air => BlockFaces::new(),
            BlockType::Dirt => BlockFaces {
                left: vec![Vec2::new(0.1, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.1), Vec2::new(0.1, 0.1)],
                right: vec![Vec2::new(0.1, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.1), Vec2::new(0.1, 0.1)],
                front: vec![Vec2::new(0.1, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.1), Vec2::new(0.1, 0.1)],
                back: vec![Vec2::new(0.1, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.1), Vec2::new(0.1, 0.1)],
                top: vec![Vec2::new(0.1, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.1), Vec2::new(0.1, 0.1)],
                bottom: vec![Vec2::new(0.1, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.1), Vec2::new(0.1, 0.1)],
            },
            BlockType::Grass => BlockFaces {
                left: vec![Vec2::new(0.2, 0.0), Vec2::new(0.1, 0.0), Vec2::new(0.1, 0.1), Vec2::new(0.2, 0.1)],
                right: vec![Vec2::new(0.2, 0.0), Vec2::new(0.1, 0.0), Vec2::new(0.1, 0.1), Vec2::new(0.2, 0.1)],
                front: vec![Vec2::new(0.2, 0.0), Vec2::new(0.1, 0.0), Vec2::new(0.1, 0.1), Vec2::new(0.2, 0.1)],
                back: vec![Vec2::new(0.2, 0.0), Vec2::new(0.1, 0.0), Vec2::new(0.1, 0.1), Vec2::new(0.2, 0.1)],
                top: vec![Vec2::new(0.3, 0.0), Vec2::new(0.2, 0.0), Vec2::new(0.2, 0.1), Vec2::new(0.3, 0.1)],
                bottom: vec![Vec2::new(0.1, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.1), Vec2::new(0.1, 0.1)],
            },
            BlockType::Stone => BlockFaces {
                left: vec![Vec2::new(0.4, 0.0), Vec2::new(0.3, 0.0), Vec2::new(0.3, 0.1), Vec2::new(0.4, 0.1)],
                right: vec![Vec2::new(0.4, 0.0), Vec2::new(0.3, 0.0), Vec2::new(0.3, 0.1), Vec2::new(0.4, 0.1)],
                front: vec![Vec2::new(0.4, 0.0), Vec2::new(0.3, 0.0), Vec2::new(0.3, 0.1), Vec2::new(0.4, 0.1)],
                back: vec![Vec2::new(0.4, 0.0), Vec2::new(0.3, 0.0), Vec2::new(0.3, 0.1), Vec2::new(0.4, 0.1)],
                top: vec![Vec2::new(0.4, 0.0), Vec2::new(0.3, 0.0), Vec2::new(0.3, 0.1), Vec2::new(0.4, 0.1)],
                bottom: vec![Vec2::new(0.4, 0.0), Vec2::new(0.3, 0.0), Vec2::new(0.3, 0.1), Vec2::new(0.4, 0.1)],
            },
            BlockType::Sand => BlockFaces {
                left: vec![Vec2::new(0.5, 0.0), Vec2::new(0.4, 0.0), Vec2::new(0.4, 0.1), Vec2::new(0.5, 0.1)],
                right: vec![Vec2::new(0.5, 0.0), Vec2::new(0.4, 0.0), Vec2::new(0.4, 0.1), Vec2::new(0.5, 0.1)],
                front: vec![Vec2::new(0.5, 0.0), Vec2::new(0.4, 0.0), Vec2::new(0.4, 0.1), Vec2::new(0.5, 0.1)],
                back: vec![Vec2::new(0.5, 0.0), Vec2::new(0.4, 0.0), Vec2::new(0.4, 0.1), Vec2::new(0.5, 0.1)],
                top: vec![Vec2::new(0.5, 0.0), Vec2::new(0.4, 0.0), Vec2::new(0.4, 0.1), Vec2::new(0.5, 0.1)],
                bottom: vec![Vec2::new(0.5, 0.0), Vec2::new(0.4, 0.0), Vec2::new(0.4, 0.1), Vec2::new(0.5, 0.1)],
            },
            BlockType::Water => BlockFaces {
                left: vec![Vec2::new(0.6, 0.0), Vec2::new(0.5, 0.0), Vec2::new(0.5, 0.1), Vec2::new(0.6, 0.1)],
                right: vec![Vec2::new(0.6, 0.0), Vec2::new(0.5, 0.0), Vec2::new(0.5, 0.1), Vec2::new(0.6, 0.1)],
                front: vec![Vec2::new(0.6, 0.0), Vec2::new(0.5, 0.0), Vec2::new(0.5, 0.1), Vec2::new(0.6, 0.1)],
                back: vec![Vec2::new(0.6, 0.0), Vec2::new(0.5, 0.0), Vec2::new(0.5, 0.1), Vec2::new(0.6, 0.1)],
                top: vec![Vec2::new(0.6, 0.0), Vec2::new(0.5, 0.0), Vec2::new(0.5, 0.1), Vec2::new(0.6, 0.1)],
                bottom: vec![Vec2::new(0.6, 0.0), Vec2::new(0.5, 0.0), Vec2::new(0.5, 0.1), Vec2::new(0.6, 0.1)],
            },
        }
    }
}

pub struct BlockFaces {
    pub left: Vec<Vec2>,
    pub right: Vec<Vec2>,
    pub front: Vec<Vec2>,
    pub back: Vec<Vec2>,
    pub top: Vec<Vec2>,
    pub bottom: Vec<Vec2>,
}

impl BlockFaces {
    fn new() -> Self {
        BlockFaces { left: vec![], right: vec![], front: vec![], back: vec![], top: vec![], bottom: vec![] }
    }
}