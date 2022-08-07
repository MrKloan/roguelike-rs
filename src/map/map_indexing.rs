use specs::prelude::*;

use crate::components::{BlocksTile, Position};
use crate::map::Map;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blockers, entities) = data;

        map.update_blocked_tiles();
        map.clear_entities();

        (&entities, &positions).join().for_each(|(entity, position)| {
            let index = map.index_of(position.x, position.y);
            map.entities[index].push(entity);

            if blockers.get(entity).is_some() {
                map.blocked_tiles[index] = true;
            }
        });
    }
}