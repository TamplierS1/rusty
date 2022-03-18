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
            <(Entity, &Point)>::query()
                .filter(component::<Player>())
                .iter(ecs)
                .for_each(|(entity, pos)| {
                    commands.push((
                        (),
                        WantsToMove {
                            entity: *entity,
                            destination: *pos + direction,
                        },
                    ));
                });

            *turn_state = TurnState::PlayerTurn;
        }
    }
}
