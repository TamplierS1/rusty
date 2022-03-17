use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();

    let offset = Point {
        x: camera.left_x,
        y: camera.top_y,
    };
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            if !out_of_bounds(*pos) {
                draw_batch.set(*pos - offset, render.color, render.glyph);
            }
        });

    draw_batch
        .submit(5000)
        .expect("Error: batch error.");
}
