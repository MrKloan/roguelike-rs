#![deny(warnings)]

use rltk::{RGB, RltkBuilder};
use specs::prelude::*;

use crate::components::{Position, Renderable};
use crate::ia::LeftWalker;
use crate::map::Map;
use crate::player::Player;
use crate::state::State;

mod components;
mod ia;
mod map;
mod player;
mod state;

fn main() -> rltk::BError {
    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 50;

    let context = RltkBuilder::simple(WIDTH, HEIGHT)?
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut state = State {
        world: World::new(),
    };

    let map = Map::new(WIDTH, HEIGHT);
    let starting_position = map.starting_position();
    state.world.insert(map);

    state.world.register::<Player>();
    state.world.register::<Position>();
    state.world.register::<Renderable>();
    state.world.register::<LeftWalker>();

    state
        .world
        .create_entity()
        .with(Player {})
        .with(starting_position)
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .build();

    rltk::main_loop(context, state)
}
