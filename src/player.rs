use bracket_lib::prelude::*;
use std::cmp::{min, max};
use super::{Position, Player, TileType, xy_idx, State};
use specs::{prelude::*, World};

/// Move the player inside the console boundaries
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_, pos) in (&mut players, &mut positions).join() {
        let dest_x = pos.x + delta_x;
        let dest_y = pos.y + delta_y;
        if map[xy_idx(dest_x, dest_y)] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

/// Apply effects from player inputs
pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            _ => {}
        }
    }
}

