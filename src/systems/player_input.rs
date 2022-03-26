use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let direction = match key {
            VirtualKeyCode::W => Point { x: 0, y: -1 },
            VirtualKeyCode::S => Point { x: 0, y: 1 },
            VirtualKeyCode::A => Point { x: -1, y: 0 },
            VirtualKeyCode::D => Point { x: 1, y: 0 },
            _ => Point::zero(),
        };

        if direction != Point::zero() {
            let (player_entity, destination) = <(Entity, &Point)>::query()
                .filter(component::<Player>())
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + direction)))
                .unwrap();

            let mut attacked = false;
            <(Entity, &Point)>::query()
                .filter(component::<Enemy>())
                .iter(ecs)
                .filter(|(_, pos)| destination == **pos)
                .for_each(|(entity, _)| {
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                    attacked = true;
                });

            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
            *turn_state = TurnState::PlayerTurn;
        }
    }
}
