#[forbid(warnings)]
use rltk::{FontCharType, GameState, Rltk, RltkBuilder, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    foreground: RGB,
    background: RGB,
}

#[derive(Component)]
struct LeftWalker {}

struct WalkLeft {}

impl<'a> System<'a> for WalkLeft {
    type SystemData = (ReadStorage<'a, LeftWalker>, WriteStorage<'a, Position>);

    fn run(&mut self, (left_walkers, mut positions): Self::SystemData) {
        for (_left_walker, position) in (&left_walkers, &mut positions).join() {
            position.x -= 1;
            if position.x < 0 {
                position.x = 79;
            }
        }
    }
}

struct State {
    world: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut walk_left = WalkLeft {};
        walk_left.run_now(&self.world);
        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();
        self.run_systems();

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (position, renderable) in (&positions, &renderables).join() {
            context.set(
                position.x,
                position.y,
                renderable.foreground,
                renderable.background,
                renderable.glyph,
            );
        }
    }
}

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut state = State {
        world: World::new(),
    };

    state.world.register::<Position>();
    state.world.register::<Renderable>();
    state.world.register::<LeftWalker>();

    state
        .world
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .build();

    for i in 0..10 {
        state
            .world
            .create_entity()
            .with(Position {
                x: i * 7,
                y: i + 20,
            })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(LeftWalker {})
            .build();
    }

    rltk::main_loop(context, state)
}
