use crate::prelude::*;

pub struct Player {
    pub pos: Point,
}

impl Player {
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.pos.x, self.pos.y, WHITE, BLACK, to_cp437('@'));
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let direction = match key {
                VirtualKeyCode::W => Point { x: 0, y: -1 },
                VirtualKeyCode::S => Point { x: 0, y: 1 },
                VirtualKeyCode::A => Point { x: -1, y: 0 },
                VirtualKeyCode::D => Point { x: 1, y: 0 },
                _ => Point::zero(),
            };

            if map.can_enter_tile(self.pos + direction) {
                self.pos += direction;
            }
        }
    }
}
