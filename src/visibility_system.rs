use rltk::{field_of_view, Point};
use specs::prelude::*;

use crate::components::{Position, Viewshed};
use crate::map::Map;
use crate::player::Player;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewsheds, positions, players) = data;

        for (entity, viewshed, position) in (&entities, &mut viewsheds, &positions).join() {
            if !viewshed.should_update {
                continue;
            }

            viewshed.should_update = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(position.x, position.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|point| map.is_in_bound(&point));

            // If this is the player, reveal what they can see.
            let player: Option<&Player> = players.get(entity);
            if player.is_some() {
                for tile in map.visible_tiles.iter_mut() {
                    *tile = false;
                }

                for tile in viewshed.visible_tiles.iter() {
                    let index = map.index_of(tile.x, tile.y);
                    map.revealed_tiles[index] = true;
                    map.visible_tiles[index] = true;
                }
            }
        }
    }
}