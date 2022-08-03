#[forbid(warnings)]
use rltk::{FontCharType, GameState, Rltk, RltkBuilder, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;

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
                position.x = WIDTH - 1;
            }
        }
    }
}

#[derive(Component)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let players = world.read_storage::<Player>();
    let mut positions = world.write_storage::<Position>();
    let map = world.fetch::<Vec<TileType>>();

    for (_player, position) in (&players, &mut positions).join() {
        let destination_idx = xy_idx(position.x + delta_x, position.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            position.x = min(WIDTH - 1, max(0, position.x + delta_x));
            position.y = min(HEIGHT - 1, max(0, position.y + delta_y));
        }
    }
}

fn player_input(state: &mut State, context: &mut Rltk) {
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

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH as usize) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (WIDTH * HEIGHT) as usize];

    // Make the boundaries walls
    for x in 0..WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, HEIGHT - 1)] = TileType::Wall;
    }
    for y in 0..HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(WIDTH - 1, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, WIDTH - 1);
        let y = rng.roll_dice(1, HEIGHT - 1);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

fn draw_map(map: &[TileType], context: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                context.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                context.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        // Move the coordinates
        x += 1;
        if x > WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}

struct State {
    world: World,
}

impl State {
    fn run_systems(&mut self) {
        WalkLeft {}.run_now(&self.world);
        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();

        player_input(self, context);
        self.run_systems();

        let map = self.world.fetch::<Vec<TileType>>();
        draw_map(&map, context);

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
    let context = RltkBuilder::simple(WIDTH, HEIGHT)?
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut state = State {
        world: World::new(),
    };

    state.world.insert(new_map());

    state.world.register::<Player>();
    state.world.register::<Position>();
    state.world.register::<Renderable>();
    state.world.register::<LeftWalker>();

    state
        .world
        .create_entity()
        .with(Player {})
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .build();

    /*for i in 0..10 {
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
    }*/

    rltk::main_loop(context, state)
}
