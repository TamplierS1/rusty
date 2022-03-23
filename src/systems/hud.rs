use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn hud(ecs: &SubWorld) {
    render_player_hud(ecs);
}

fn render_player_hud(ecs: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    let player_box = Rect {
        x1: 0,
        y1: 0,
        x2: 20,
        y2: 10,
    };
    draw_batch.draw_double_box(player_box, ColorPair::new(WHITE, BLACK));
    draw_batch.print_centered_at(
        Point {
            x: player_box.x1 + player_box.x2 / 2,
            y: 0,
        },
        "You",
    );
    draw_batch.printer(
        Point {
            x: player_box.x1,
            y: player_box.y1,
        } + Point { x: 2, y: 2 },
        format!(
            "#[white]Health: #[red]{}#[]/#[red]{}#[]",
            player_health.current, player_health.max
        ),
        TextAlign::Left,
        None,
    );

    draw_batch
        .submit(10000)
        .expect("Error: player hud render error.");
}
