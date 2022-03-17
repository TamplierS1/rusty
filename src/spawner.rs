use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(DARK_GREY, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
