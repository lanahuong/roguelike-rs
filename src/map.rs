use super::rect::Rect;
use bracket_lib::prelude::*;
use std::cmp::{max, min};

/// The types of tiles
#[derive(Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

/// Linearize the coordinates in the map
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

/// Makes a map with solid boundaries and 400 randomly placed walls. No guarantees that it won't
/// look awful.
pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // Add walls around the map
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Add random walls
    let mut rng = RandomNumberGenerator::new();

    for _ in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

/// Add a rectangle room on a map
fn apply_room_to_map(map: &mut [TileType], room: &Rect) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

/// Add an horizontal corridor on a map
fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

/// Add an vertical corridor on a map
fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

/// Create a map with random rooms connected by corridors
/// Return the map and the rooms
pub fn new_map_rooms_and_corridors(max_rooms: usize) -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80 * 50];
    let mut rng = RandomNumberGenerator::new();
    let mut rooms: Vec<Rect> = Vec::new();
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
        if !rooms.iter().any(|e| new_room.intersect(e)) {
            // Add the room to the map
            apply_room_to_map(&mut map, &new_room);

            // If it is not the first room link it to the previous one
            if !rooms.is_empty() {
                let new_center = new_room.center();
                let prev_center = rooms.last().unwrap().center();

                // Alternate between linking horizontally then vertically and the other way around
                if rng.roll_dice(1, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_center.0, new_center.0, prev_center.1);
                    apply_vertical_tunnel(&mut map, prev_center.1, new_center.1, new_center.0);
                } else {
                    apply_vertical_tunnel(&mut map, prev_center.1, new_center.1, prev_center.0);
                    apply_horizontal_tunnel(&mut map, prev_center.0, new_center.0, new_center.1);
                }
            }

            // Add the new room to the list of rooms
            rooms.push(new_room);
        }
    }

    (rooms, map)
}

/// Draw a map in a console
pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => ctx.set(
                x,
                y,
                RGB::from_f32(0.5, 0.5, 0.5),
                RGB::from_f32(0., 0., 0.),
                to_cp437(' '),
            ),
            TileType::Wall => ctx.set(x, y, RGB::named(YELLOW3), RGB::named(BLACK), to_cp437('█')),
        };

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
