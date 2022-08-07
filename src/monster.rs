use rltk::{a_star_search, NavigationPath, Point};
use rltk::DistanceAlg::Pythagoras;
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

        let players_positions: Vec<Position> = (&players, &positions).join()
            .map(|(_, position)| position.clone())
            .collect();

        for (_monster, monster_viewshed, monster_position) in (&monsters, &mut viewsheds, &mut positions).join() {
            let path_to_nearest_player = players_positions.iter()
                .filter(|player_position| self.is_far_enough(&*monster_position, player_position))
                .map(|player_position| map.index_of(player_position.x, player_position.y))
                .filter(|player_index| monster_viewshed.visible_tiles[*player_index])
                .map(|player_index| (map.index_of(monster_position.x, monster_position.y), player_index))
                .map(|(monster_index, player_index)| a_star_search(monster_index as i32, player_index as i32, &*map))
                .filter(|path| path.success && path.steps.len() > 1)
                .reduce(|first, second| self.shortest_path(first, second));

            if let Some(path) = path_to_nearest_player {
                monster_position.x = path.steps[1] as i32 % map.width;
                monster_position.y = path.steps[1] as i32 / map.width;
                monster_viewshed.should_update = true;
            }
        }
    }
}

impl MonsterAI {
    fn is_far_enough(&self, monster_position: &Position, player_position: &Position) -> bool {
        Pythagoras.distance2d(Point::from(monster_position), Point::from(player_position)) > 1.
    }

    fn shortest_path(&self, first: NavigationPath, second: NavigationPath) -> NavigationPath {
        if first.steps.len() < second.steps.len() {
            first
        } else {
            second
        }
    }
}