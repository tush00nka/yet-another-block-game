use bevy::prelude::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BlockType {
    Air,
    Dirt,
    Grass,
    Stone,
    Sand,
    Water,
    WoodLog,
    Leaves,
    Cactus,
}

impl BlockType {
    pub fn is_transparent(&self) -> bool {
        match self {
            BlockType::Air => true,
            BlockType::Water => true,
            _ => false,
        }
    }

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
            BlockType::WoodLog => BlockFaces {
                left: vec![Vec2::new(0.8, 0.0), Vec2::new(0.7, 0.0), Vec2::new(0.7, 0.1), Vec2::new(0.8, 0.1)],
                right: vec![Vec2::new(0.8, 0.0), Vec2::new(0.7, 0.0), Vec2::new(0.7, 0.1), Vec2::new(0.8, 0.1)],
                front: vec![Vec2::new(0.8, 0.0), Vec2::new(0.7, 0.0), Vec2::new(0.7, 0.1), Vec2::new(0.8, 0.1)],
                back: vec![Vec2::new(0.8, 0.0), Vec2::new(0.7, 0.0), Vec2::new(0.7, 0.1), Vec2::new(0.8, 0.1)],
                top: vec![Vec2::new(0.7, 0.0), Vec2::new(0.6, 0.0), Vec2::new(0.6, 0.1), Vec2::new(0.7, 0.1)],
                bottom: vec![Vec2::new(0.7, 0.0), Vec2::new(0.6, 0.0), Vec2::new(0.6, 0.1), Vec2::new(0.7, 0.1)],
            },
            BlockType::Leaves => BlockFaces {
                left: vec![Vec2::new(0.9, 0.0), Vec2::new(0.8, 0.0), Vec2::new(0.8, 0.1), Vec2::new(0.9, 0.1)],
                right: vec![Vec2::new(0.9, 0.0), Vec2::new(0.8, 0.0), Vec2::new(0.8, 0.1), Vec2::new(0.9, 0.1)],
                front: vec![Vec2::new(0.9, 0.0), Vec2::new(0.8, 0.0), Vec2::new(0.8, 0.1), Vec2::new(0.9, 0.1)],
                back: vec![Vec2::new(0.9, 0.0), Vec2::new(0.8, 0.0), Vec2::new(0.8, 0.1), Vec2::new(0.9, 0.1)],
                top: vec![Vec2::new(0.9, 0.0), Vec2::new(0.8, 0.0), Vec2::new(0.8, 0.1), Vec2::new(0.9, 0.1)],
                bottom: vec![Vec2::new(0.9, 0.0), Vec2::new(0.8, 0.0), Vec2::new(0.8, 0.1), Vec2::new(0.9, 0.1)],
            },
            BlockType::Cactus => BlockFaces {
                left: vec![Vec2::new(0.8, 0.1), Vec2::new(0.7, 0.1), Vec2::new(0.7, 0.2), Vec2::new(0.8, 0.2)],
                right: vec![Vec2::new(0.8, 0.1), Vec2::new(0.7, 0.1), Vec2::new(0.7, 0.2), Vec2::new(0.8, 0.2)],
                front: vec![Vec2::new(0.8, 0.1), Vec2::new(0.7, 0.1), Vec2::new(0.7, 0.2), Vec2::new(0.8, 0.2)],
                back: vec![Vec2::new(0.8, 0.1), Vec2::new(0.7, 0.1), Vec2::new(0.7, 0.2), Vec2::new(0.8, 0.2)],
                top: vec![Vec2::new(0.7, 0.1), Vec2::new(0.6, 0.1), Vec2::new(0.6, 0.2), Vec2::new(0.7, 0.2)],
                bottom: vec![Vec2::new(0.7, 0.1), Vec2::new(0.6, 0.1), Vec2::new(0.6, 0.2), Vec2::new(0.7, 0.2)],
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