use crate::prelude::*;

#[system]
#[read_component(MovingRandomly)]
#[write_component(Point)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    <&mut Point>::query()
        .filter(component::<MovingRandomly>())
        .iter_mut(ecs)
        .for_each(|pos| {
            let direction = match RandomNumberGenerator::new().range(0, 4) {
                0 => Point { x: -1, y: 0 },
                1 => Point { x: 0, y: -1 },
                2 => Point { x: 1, y: 0 },
                _ => Point { x: 0, y: 1 },
            };

            if map.can_enter_tile(direction + *pos) {
                *pos += direction;
            }
        });
}
