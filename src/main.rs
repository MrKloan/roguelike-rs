#![deny(warnings)]

use rltk::{RGB, RltkBuilder};
use specs::prelude::*;

use crate::components::{Position, Renderable, Viewshed};
use crate::map::Map;
use crate::player::Player;
use crate::state::State;

mod components;
mod map;
mod player;
mod state;
mod visibility_system;

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

    state.world.register::<Player>();
    state.world.register::<Position>();
    state.world.register::<Renderable>();
    state.world.register::<Viewshed>();

    state
        .world
        .create_entity()
        .with(Player {})
        .with(map.starting_position())
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Viewshed::new(8, &map))
        .build();

    state.world.insert(map);

    rltk::main_loop(context, state)
}
