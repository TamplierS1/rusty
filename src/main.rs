mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const TILE_SIZE: i32 = 16;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    res: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();

        let mut res = Resources::default();
        let map_builder = MapBuilder::new();

        spawn_player(&mut ecs, map_builder.player_pos);
        map_builder
            .rooms
            .iter()
            .for_each(|room| {
                // Skip the player room.
                if room.center() == map_builder.player_pos {
                    return;
                }
                spawn_monster(&mut ecs, room.center())
            });

        res.insert(map_builder.map);
        res.insert(Camera::new(
            map_builder.player_pos,
            Point {
                x: SCREEN_WIDTH,
                y: SCREEN_HEIGHT,
            },
        ));

        Self {
            ecs,
            res,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.res.insert(ctx.key);
        self.systems
            .execute(&mut self.ecs, &mut self.res);

        render_draw_buffer(ctx).expect("Error: render error.");
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
