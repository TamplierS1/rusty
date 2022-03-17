use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point { x, y };
            let offset = Point {
                x: camera.left_x,
                y: camera.top_y,
            };

            if !out_of_bounds(pt) {
                let glyph = match map.tiles[map_idx(Point { x, y })] {
                    TileType::Floor => to_cp437(' '),
                    TileType::Wall => to_cp437('#'),
                };
                draw_batch.set(pt - offset, ColorPair::new(DARK_GREY, BLACK), glyph);
            }
        }
    }

    draw_batch
        .submit(0)
        .expect("Error: batch error.");
}
