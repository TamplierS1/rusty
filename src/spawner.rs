use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(DARK_GREY, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

pub fn spawn_monster(ecs: &mut World, pos: Point) {
    let enemy_type = RandomNumberGenerator::new().range::<i32>(0, 4);
    let glyph = match enemy_type {
        0 => to_cp437('E'),
        1 => to_cp437('O'),
        2 => to_cp437('o'),
        _ => to_cp437('g'),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(DARKKHAKI, BLACK),
            glyph,
        },
        MovingRandomly,
        match enemy_type {
            0 => Health {
                current: 30,
                max: 30,
            },
            1 => Health {
                current: 17,
                max: 17,
            },
            2 => Health {
                current: 10,
                max: 10,
            },
            _ => Health { current: 6, max: 6 },
        },
    ));
}
