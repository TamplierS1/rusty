## rusty

`rusty` is a dungeon crawler roguelike written in Rust.

## Short Description

A dungeon crawler with procedurally generated levels, monsters of increasing difficulty and turn-based movement.


## MVP

1. ~~Create a basic dungeon map~~.
2. ~~Place the player and let them walk around~~. 
3. Spawn monsters and let the player kill them by walking into them.
4. Add health and a combat system that uses it.
5. Add healing potions.
6. Display a game over screen when the player dies.
7. Add the Amulet to the level and let the player win by reaching it.

## Stretch goals.

1. Add Field-Of-View.
2. Add a defence stat.
3. Move to a data-driven approach for enemies.
4. Add new kinds of monsters.
   1. Add bosses.
5. Add items: weapons, armor, potions.
6. Add multiple floors and place the Amulet on the last one.

## Dependencies

- [bracket-lib](https://github.com/amethyst/bracket-lib) - The Roguelike Toolkit (RLTK), implemented for Rust.
- [legion](https://github.com/amethyst/legion) - High performance Rust ECS library.