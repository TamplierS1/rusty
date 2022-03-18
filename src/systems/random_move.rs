use crate::prelude::*;

#[system]
#[read_component(MovingRandomly)]
#[read_component(Point)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &Point)>::query()
        .filter(component::<MovingRandomly>())
        .iter(ecs)
        .for_each(|(entity, pos)| {
            let direction = match RandomNumberGenerator::new().range(0, 4) {
                0 => Point { x: -1, y: 0 },
                1 => Point { x: 0, y: -1 },
                2 => Point { x: 1, y: 0 },
                _ => Point { x: 0, y: 1 },
            };

            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination: *pos + direction,
                },
            ));
        });
}
