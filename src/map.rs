use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn can_enter_tile(&self, pos: Point) -> bool {
        !out_of_bounds(pos) && self.tiles[map_idx(pos)] == TileType::Floor
    }
}

pub fn map_idx(pos: Point) -> usize {
    (pos.x + SCREEN_WIDTH * pos.y) as usize
}

pub fn out_of_bounds(pos: Point) -> bool {
    pos.x >= SCREEN_WIDTH || pos.y >= SCREEN_HEIGHT || pos.x <= 0 || pos.y <= 0
}

pub fn try_idx(pos: Point) -> Option<usize> {
    if out_of_bounds(pos) {
        None
    } else {
        Some(map_idx(pos))
    }
}
