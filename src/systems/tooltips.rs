use crate::prelude::*;

#[system]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(Point)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let map_pos = *mouse_pos
        + Point {
            x: camera.left_x,
            y: camera.top_y,
        };

    <(Entity, &Name, &Point)>::query()
        .iter(ecs)
        .filter(|(_, _, pos)| **pos == map_pos)
        .for_each(|(entity, name, _)| {
            if let Ok(health) = ecs
                .entry_ref(*entity)
                .unwrap()
                .get_component::<Health>()
            {
                draw_tooltip(*mouse_pos, Some(health), &name.0);
            } else {
                draw_tooltip(*mouse_pos, None, &name.0);
            }
        });
}

fn draw_tooltip(box_pos: Point, health: Option<&Health>, name: &String) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let box_size = Point { x: 20, y: 4 };

    draw_batch.draw_double_box(
        Rect {
            x1: box_pos.x,
            y1: box_pos.y,
            x2: box_pos.x + box_size.x,
            y2: box_pos.y + box_size.y,
        },
        ColorPair::new(WHITE, BLACK),
    );

    draw_batch.print_centered_at(
        Point {
            x: box_pos.x + box_size.x / 2,
            y: box_pos.y,
        },
        name,
    );

    if let Some(health) = health {
        draw_batch.printer(
            Point {
                x: box_pos.x,
                y: box_pos.y,
            } + Point { x: 2, y: 2 },
            format!(
                "#[white]Health: #[red]{}#[]/#[red]{}#[]",
                health.current, health.max
            ),
            TextAlign::Left,
            None,
        );
    }
    draw_batch
        .submit(10100)
        .expect("Error: monster hud render error.");
}
