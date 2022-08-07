#![deny(warnings)]

use rltk::{RGB, RltkBuilder};
use specs::prelude::*;

use crate::components::{BlocksTile, Position, Renderable};
use crate::map::Map;
use crate::monster::Monster;
use crate::player::Player;
use crate::state::State;
use crate::visibility::Viewshed;

mod components;
mod map;
mod monster;
mod player;
mod state;
mod visibility;

fn main() -> rltk::BError {
    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 50;

    let context = RltkBuilder::simple(WIDTH, HEIGHT)?
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut state = State::new();

    let map = Map::new(WIDTH, HEIGHT);

    state.world.register::<BlocksTile>();
    state.world.register::<Monster>();
    state.world.register::<Player>();
    state.world.register::<Position>();
    state.world.register::<Renderable>();
    state.world.register::<Viewshed>();

    add_monsters(&mut state, &map);
    add_player(&mut state, &map);

    state.world.insert(map);

    rltk::main_loop(context, state)
}

fn add_player(state: &mut State, map: &Map) {
    state
        .world
        .create_entity()
        .with(Player {
            name: String::from("Player")
        })
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

    for (index, room) in map.rooms.iter().enumerate().skip(1) {
        let roll = rng.roll_dice(1, 2);
        let (name, glyph) = match roll {
            1 => ("Goblin", rltk::to_cp437('g')),
            _ => ("Orc", rltk::to_cp437('o'))
        };

        state.world.create_entity()
            .with(Monster {
                name: String::from(format!("{} #{}", name, index))
            })
            .with(room.center())
            .with(Renderable {
                glyph,
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(Viewshed::new(8, map))
            .with(BlocksTile {})
            .build();
    }
}
