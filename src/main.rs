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

    add_player(&mut state, &map);
    add_monsters(&mut state, &map);

    state.world.insert(map);

    rltk::main_loop(context, state)
}

fn add_player(state: &mut State, map: &Map) {
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
}

fn add_monsters(state: &mut State, map: &Map) {
    let mut rng = rltk::RandomNumberGenerator::new();

    for room in map.rooms.iter().skip(1) {
        let roll = rng.roll_dice(1, 2);
        let glyph = match roll {
            1 => rltk::to_cp437('g'),
            _ => rltk::to_cp437('o')
        };

        state.world.create_entity()
            .with(room.center())
            .with(Renderable {
                glyph,
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(Viewshed::new(8, map))
            .build();
    }
}
