use std::cmp::{max, min};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;

use crate::{State, Viewshed};
use crate::components::Position;
use crate::map::{Map, TileType};

#[derive(Component)]
pub struct Player {}

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let players = world.read_storage::<Player>();
    let mut positions = world.write_storage::<Position>();
    let mut viewsheds = world.write_storage::<Viewshed>();

    let map = world.fetch::<Map>();

    for (_player, position, viewshed) in (&players, &mut positions, &mut viewsheds).join() {
        let destination_index = map.index_of(position.x + delta_x, position.y + delta_y);
        if map.tiles[destination_index] != TileType::Wall {
            position.x = min(map.width - 1, max(0, position.x + delta_x));
            position.y = min(map.height - 1, max(0, position.y + delta_y));
            viewshed.should_update = true;
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