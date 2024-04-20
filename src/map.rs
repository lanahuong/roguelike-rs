use super::rect::Rect;
use bracket_lib::prelude::*;
use std::cmp::{max, min};

/// The types of tiles
#[derive(Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    width: usize,
    height: usize,
}

impl Map {
    /// Linearize the coordinates in the map
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    /// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
    /// look awful.
    pub fn new_map_test() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Floor; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
        };

        // Add walls around the map
        for x in 0..80 {
            let idx = map.xy_idx(x, 0);
            map.tiles[idx] = TileType::Wall;
            let idx = map.xy_idx(x, 49);
            map.tiles[idx] = TileType::Wall;
        }
        for y in 0..50 {
            let idx = map.xy_idx(0, y);
            map.tiles[idx] = TileType::Wall;
            let idx = map.xy_idx(79, y);
            map.tiles[idx] = TileType::Wall;
        }

        // Add random walls
        let mut rng = RandomNumberGenerator::new();

        for _ in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = map.xy_idx(x, y);
            if idx != map.xy_idx(40, 25) {
                map.tiles[idx] = TileType::Wall;
            }
        }

        map
    }

    /// Add a rectangle room on a map
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// Add an horizontal corridor on a map
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width * self.height {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// Add an vertical corridor on a map
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width * self.height {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Create a map with random rooms connected by corridors
    /// Return the map and the rooms
    pub fn new_map_rooms_and_corridors(max_rooms: usize) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),
            width: 80,
            height: 50,
        };
        let mut rng = RandomNumberGenerator::new();
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        for _ in 0..max_rooms {
            // Generate a random room
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(0, 79 - w - 1);
            let y = rng.range(0, 49 - h - 1);
            let new_room = Rect::new(x, y, w, h);

            // If the room is valid add it to the map
            if !map.rooms.iter().any(|e| new_room.intersect(e)) {
                // Add the room to the map
                map.apply_room_to_map(&new_room);

                // If it is not the first room link it to the previous one
                if !map.rooms.is_empty() {
                    let new_center = new_room.center();
                    let prev_center = map.rooms.last().unwrap().center();

                    // Alternate between linking horizontally then vertically and the other way around
                    if rng.roll_dice(1, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_center.0, new_center.0, prev_center.1);
                        map.apply_vertical_tunnel(prev_center.1, new_center.1, new_center.0);
                    } else {
                        map.apply_vertical_tunnel(prev_center.1, new_center.1, prev_center.0);
                        map.apply_horizontal_tunnel(prev_center.0, new_center.0, new_center.1);
                    }
                }

                // Add the new room to the list of rooms
                map.rooms.push(new_room);
            }
        }

        map
    }

    /// Draw a map in a console
    pub fn draw_map(&self, ctx: &mut BTerm) {
        let mut y = 0;
        let mut x = 0;

        for tile in self.tiles.iter() {
            match tile {
                TileType::Floor => ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    to_cp437(' '),
                ),
                TileType::Wall => {
                    ctx.set(x, y, RGB::named(YELLOW3), RGB::named(BLACK), to_cp437('█'))
                }
            };

            x += 1;
            if x >= self.width {
                x = 0;
                y += 1;
            }
        }
    }
}
