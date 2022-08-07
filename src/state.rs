use rltk::{GameState, Rltk};
use specs::prelude::*;

use crate::{Player, Viewshed};
use crate::components::{Position, Renderable};
use crate::map::Map;
use crate::player::player_input;
use crate::visibility_system::VisibilitySystem;

pub struct State {
    pub world: World,
}

impl State {
    fn run_systems(&mut self) {
        VisibilitySystem {}.run_now(&self.world);
        self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();

        player_input(self, context);
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
    }
}