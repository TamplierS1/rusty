use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
    pub size: Point,
}

impl Camera {
    pub fn new(player_pos: Point, size: Point) -> Self {
        Self {
            left_x: player_pos.x - size.x / 2,
            right_x: player_pos.x + size.x / 2,
            top_y: player_pos.y - size.y / 2,
            bottom_y: player_pos.y + size.y / 2,
            size,
        }
    }

    pub fn update(&mut self, player_pos: Point) {
        self.left_x = player_pos.x - self.size.x / 2;
        self.right_x = player_pos.x + self.size.x / 2;
        self.top_y = player_pos.y - self.size.y / 2;
        self.bottom_y = player_pos.y + self.size.y / 2;
    }
}
