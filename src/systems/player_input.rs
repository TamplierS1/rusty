use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
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
            <&mut Point>::query()
                .filter(component::<Player>())
                .iter_mut(ecs)
                .for_each(|pos| {
                    let destination = *pos + direction;
                    if map.can_enter_tile(destination) {
                        *pos = destination;
                        camera.update(destination);
                    }
                });
        }
    }
}
