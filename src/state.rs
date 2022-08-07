use rltk::{GameState, Rltk};
use specs::prelude::*;

use crate::{Player, Viewshed};
use crate::components::{Position, Renderable};
use crate::map::Map;
use crate::monster::MonsterAI;
use crate::player::player_input;
use crate::visibility_system::VisibilitySystem;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub world: World,
    pub run_state: RunState,
}

impl State {
    pub fn new() -> State {
        State {
            world: World::new(),
            run_state: RunState::Running,
        }
    }

    fn run_systems(&mut self) {
        VisibilitySystem {}.run_now(&self.world);
        MonsterAI {}.run_now(&self.world);
        self.world.maintain();
    }

    fn paused(&mut self, context: &mut Rltk) -> RunState {
        player_input(self, context)
    }

    fn running(&mut self, context: &mut Rltk) -> RunState {
        context.cls();

        self.run_systems();

        let map = self.world.fetch::<Map>();
        map.draw(&self.world, context);

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();
        let players = self.world.read_storage::<Player>();
        let viewsheds = self.world.read_storage::<Viewshed>();
        let map = self.world.fetch::<Map>();

        for (position, renderable) in (&positions, &renderables).join() {
            for (_player, viewshed) in (&players, &viewsheds).join() {
                let map_index = map.index_of(position.x, position.y);
                if viewshed.visible_tiles[map_index] {
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

        RunState::Paused
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        self.run_state = match self.run_state {
            RunState::Paused => self.paused(context),
            RunState::Running => self.running(context)
        }
    }
}