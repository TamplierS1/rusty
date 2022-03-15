mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mb = MapBuilder::new();
        Self {
            map: mb.map,
            player: Player::new(mb.player_pos),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.player.update(ctx, &self.map);

        ctx.cls();
        self.map.render(ctx);
        self.player.render(ctx);
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
        .build()?;

    main_loop(context, State::new())
}
