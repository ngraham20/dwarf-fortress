use std::cmp::{min, max};

use bracket_lib::prelude::*;
use specs::prelude::*;

use super::{Map, Position, Player, Tile, State};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut map = ecs.write_resource::<Map>();
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    for (_p, pos) in (&mut players, &mut positions).join() {
        let try_x = min(79, max(0, pos.x + dx));
        let try_y = min(49, max(0, pos.y + dy));
        let try_pos = map.xy_idx(try_x, try_y);
        match map.tiles[try_pos] {
            Tile::Floor => {
                pos.x = try_x;
                pos.y = try_y;
            }
            Tile::Minable(_ore) => {
                try_dig(try_x, try_y, &mut map);
            }
        }
    }
}

pub fn try_dig(x: i32, y: i32, map: &mut Map) {
    let target = map.xy_idx(x, y);
    match map.tiles[target] {
        Tile::Minable(_ore) => {
            map.tiles[target] = Tile::Floor;
        }
        _ => {}
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            // Player Movement
            VirtualKeyCode::W | VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S | VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::A | VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D | VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            _ => {}
        },
    }
}