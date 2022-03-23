mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

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
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    res: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
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
        res.insert(TurnState::AwaitingInput);

        Self {
            ecs,
            res,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(0);

        self.res.insert(ctx.key);

        let current_state = self
            .res
            .get::<TurnState>()
            .unwrap()
            .clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.res),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.res),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.res),
        }

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
        .with_font("terminal8x8.png", 8, 8)
        .with_sparse_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .build()?;

    register_palette_color("red", RGB::named(RED));
    register_palette_color("white", RGB::named(WHITE));

    main_loop(context, State::new())
}
