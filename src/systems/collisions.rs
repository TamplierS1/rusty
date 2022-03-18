use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    // Find the player position.
    <&Point>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .for_each(|pos| player_pos = *pos);

    // Remove all entities that have the same pos as the player.
    <(Entity, &Point)>::query()
        .filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, pos)| player_pos == **pos)
        .for_each(|(entity, _)| commands.remove(*entity));
}
