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

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                match self.tiles[map_idx(Point { x, y })] {
                    TileType::Wall => ctx.set(x, y, DARKGREY, BLACK, to_cp437('#')),
                    TileType::Floor => ctx.set(x, y, BLACK, BLACK, to_cp437(' ')),
                }
            }
        }
    }
}

pub fn map_idx(pos: Point) -> usize {
    (pos.x + SCREEN_WIDTH * pos.y) as usize
}
