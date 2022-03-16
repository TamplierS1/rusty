mod camera;
mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const TILE_SIZE: i32 = 16;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mb = MapBuilder::new();
        Self {
            map: mb.map,
            player: Player::new(mb.player_pos),
            camera: Camera::new(
                mb.player_pos,
                Point {
                    x: SCREEN_WIDTH,
                    y: SCREEN_HEIGHT,
                },
            ),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.player.update(ctx, &self.map);
        self.camera.update(self.player.pos);

        ctx.cls();
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect(
            format!(
                "Error: failed to construct {}x{} terminal.",
                SCREEN_WIDTH, SCREEN_HEIGHT
            )
            .as_str(),
        )
        .with_title("rusty")
        .with_fps_cap(60.0)
        .with_tile_dimensions(TILE_SIZE, TILE_SIZE)
        .build()?;

    main_loop(context, State::new())
}
