use crate::prelude::*;

#[system]
#[read_component(MovingRandomly)]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(Player)]
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
            let destination = *pos + direction;

            let mut attacked = false;
            <(Entity, &Point, &Health)>::query()
                .iter(ecs)
                .filter(|(_, pos, _)| **pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    attacked = true;
                });

            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        });
}
