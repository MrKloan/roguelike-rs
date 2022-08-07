use std::cmp::{max, min};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;

use crate::{State, Statistics, Viewshed};
use crate::components::Position;
use crate::map::Map;
use crate::state::RunState;

#[derive(Component)]
pub struct Player {
    pub name: String,
}

pub fn player_input(state: &mut State, context: &mut Rltk) -> RunState {
    return match context.key {
        None => RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut state.world),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut state.world),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut state.world),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut state.world),
            VirtualKeyCode::Numpad7 => try_move_player(-1, -1, &mut state.world),
            VirtualKeyCode::Numpad9 => try_move_player(1, -1, &mut state.world),
            VirtualKeyCode::Numpad1 => try_move_player(-1, 1, &mut state.world),
            VirtualKeyCode::Numpad3 => try_move_player(1, 1, &mut state.world),
            _ => RunState::Paused
        },
    };
}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) -> RunState {
    let players = world.read_storage::<Player>();
    let statistics = world.read_storage::<Statistics>();
    let mut positions = world.write_storage::<Position>();
    let mut viewsheds = world.write_storage::<Viewshed>();

    let map = world.fetch::<Map>();

    for (_player, position, viewshed) in (&players, &mut positions, &mut viewsheds).join() {
        let destination_index = map.index_of(position.x + delta_x, position.y + delta_y);

        for potential_target in map.entities[destination_index].iter() {
            match statistics.get(*potential_target) {
                Some(_target) => {
                    rltk::console::log("Attack!");
                    return RunState::Running;
                }
                None => {}
            }
        }

        if !map.blocked_tiles[destination_index] {
            position.x = min(map.width - 1, max(0, position.x + delta_x));
            position.y = min(map.height - 1, max(0, position.y + delta_y));
            viewshed.should_update = true;
        }
    }

    RunState::Running
}