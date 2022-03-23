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
    let (health, name, glyph) = match RandomNumberGenerator::new().roll_dice(1, 20) {
        1..=12 => goblin(),
        13..=17 => orc(),
        18..=19 => ogre(),
        _ => ettin(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(DARKKHAKI, BLACK),
            glyph,
        },
        MovingRandomly,
        Health {
            current: health,
            max: health,
        },
        Name(name),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}

fn ogre() -> (i32, String, FontCharType) {
    (6, "Ogre".to_string(), to_cp437('O'))
}

fn ettin() -> (i32, String, FontCharType) {
    (13, "Ettin".to_string(), to_cp437('E'))
}
