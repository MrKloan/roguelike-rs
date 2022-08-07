use std::collections::HashMap;

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
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, monsters, players, mut viewsheds, mut positions) = data;

        let players: HashMap<String, Position> = (&players, &positions).join()
            .map(|player| (player.0.name.clone(), player.1.clone()))
            .collect();

        for (monster, monster_viewshed, monster_position) in (&monsters, &mut viewsheds, &mut positions).join() {
            for (player_name, player_position) in players.iter() {
                let player_index = map.index_of(player_position.x, player_position.y);

                if monster_viewshed.visible_tiles[player_index] {
                    console::log(format!("{} shouts insults at {}!", monster.name, player_name));

                    let path = rltk::a_star_search(
                        map.index_of(monster_position.x, monster_position.y) as i32,
                        player_index as i32,
                        &*map,
                    );
                    if path.success && path.steps.len() > 1 {
                        monster_position.x = path.steps[1] as i32 % map.width;
                        monster_position.y = path.steps[1] as i32 / map.width;
                        monster_viewshed.should_update = true;
                    }
                }
            }
        }
    }
}