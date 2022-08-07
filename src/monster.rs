use rltk::console;
use specs::prelude::*;
use specs_derive::Component;

use crate::{Map, Player};
use crate::components::Position;
use crate::visibility::Viewshed;

#[derive(Component, Debug)]
pub struct Monster {
    pub name: String,
}

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, monsters, players, viewsheds, positions) = data;

        for (monster, monster_viewshed) in (&monsters, &viewsheds).join() {
            for (player, player_position) in (&players, &positions).join() {
                let player_index = map.index_of(player_position.x, player_position.y);
                if monster_viewshed.visible_tiles[player_index] {
                    console::log(format!("{} shouts insults at {}!", monster.name, player.name));
                }
            }
        }
    }
}