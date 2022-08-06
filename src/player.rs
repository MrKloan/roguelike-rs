use std::cmp::{max, min};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;

use crate::components::Position;
use crate::map::{TileType, map_index};
use crate::State;

#[derive(Component)]
pub struct Player {}

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let players = world.read_storage::<Player>();
    let mut positions = world.write_storage::<Position>();
    let map = world.fetch::<Vec<TileType>>();

    for (_player, position) in (&players, &mut positions).join() {
        let destination_idx = map_index(position.x + delta_x, position.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            position.x = min(crate::WIDTH - 1, max(0, position.x + delta_x));
            position.y = min(crate::HEIGHT - 1, max(0, position.y + delta_y));
        }
    }
}

pub fn player_input(state: &mut State, context: &mut Rltk) {
    match context.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut state.world),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut state.world),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut state.world),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut state.world),
            _ => {}
        },
    }
}