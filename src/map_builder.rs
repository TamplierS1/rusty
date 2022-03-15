use bracket_lib::prelude::Rect;

use crate::prelude::*;

const ROOM_ATTEMPTS: i32 = 30;
const MIN_ROOM_SIZE: i32 = 4;
const MAX_ROOM_SIZE: i32 = 10;

pub struct MapBuilder {
    pub map: Map,
    pub player_pos: Point,
    rooms: Vec<Rect>,
}

impl MapBuilder {
    pub fn new() -> Self {
        let mut mb = Self {
            map: Map::new(),
            rooms: Vec::new(),
            player_pos: Point { x: 0, y: 0 },
        };

        mb.fill();
        mb.carve_rooms();
        mb.carve_corridors();

        let room = mb.rooms.last().unwrap();
        mb.player_pos = Point {
            x: room.x1 + (room.x2 - room.x1) / 2,
            y: room.y1 + (room.y2 - room.y1) / 2,
        };
        mb
    }

    fn fill(&mut self) {
        self.map.tiles = vec![TileType::Wall; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
    }

    fn carve_rooms(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..ROOM_ATTEMPTS {
            let pos = Point {
                x: rng.range(1, SCREEN_WIDTH - 1),
                y: rng.range(1, SCREEN_HEIGHT - 1),
            };
            let size = Point {
                x: rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
                y: rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE),
            };

            let new_room = Rect {
                x1: pos.x,
                x2: pos.x + size.x,
                y1: pos.y,
                y2: pos.y + size.y,
            };

            if out_of_bounds(Point {
                x: new_room.x2,
                y: new_room.y2,
            }) {
                continue;
            }

            let mut intersects = false;
            for room in &self.rooms {
                if room.intersect(&new_room) {
                    intersects = true;
                    break;
                }
            }

            if !intersects {
                self.carve_room(pos, size);
                self.rooms.push(new_room);
            }
        }
    }

    fn carve_corridors(&mut self) {
        let rooms = self.rooms.clone();
        for (i, room) in rooms.iter().enumerate() {
            // TODO: this can be replaced with skip(len-1)
            if i == rooms.len() - 1 {
                return;
            }
            self.connect_rooms(*room, self.rooms[i + 1]);
        }
    }

    fn connect_rooms(&mut self, room1: Rect, room2: Rect) {
        // TODO: replace all manual center calculations with Rect::center().
        let center1 = Point {
            x: room1.x1 + (room1.x2 - room1.x1) / 2,
            y: room1.y1 + (room1.y2 - room1.y1) / 2,
        };
        let center2 = Point {
            x: room2.x1 + (room2.x2 - room2.x1) / 2,
            y: room2.y1 + (room2.y2 - room2.y1) / 2,
        };
        let mut temp = Point {
            x: center1.x,
            y: center1.y,
        };

        loop {
            if ((temp.x - 1) - center2.x).abs() < (temp.x - center2.x).abs() {
                temp.x -= 1;
            } else if ((temp.x + 1) - center2.x).abs() < (temp.x - center2.x).abs() {
                temp.x += 1
            } else if ((temp.y + 1) - center2.y).abs() < (temp.y - center2.y).abs() {
                temp.y += 1
            } else if ((temp.y - 1) - center2.y).abs() < (temp.y - center2.y).abs() {
                temp.y -= 1
            } else {
                break;
            }

            if !out_of_bounds(temp) {
                self.carve(temp);
            }
        }
    }

    fn carve_room(&mut self, pos: Point, size: Point) {
        for y in pos.y..pos.y + size.y {
            for x in pos.x..pos.x + size.x {
                self.carve_xy(x, y)
            }
        }
    }

    fn carve(&mut self, pos: Point) {
        self.map.tiles[map_idx(pos)] = TileType::Floor;
    }

    fn carve_xy(&mut self, x: i32, y: i32) {
        self.map.tiles[map_idx(Point { x, y })] = TileType::Floor;
    }
}
