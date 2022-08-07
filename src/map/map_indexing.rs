use specs::prelude::*;

use crate::components::{BlocksTile, Position};
use crate::map::Map;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, positions, blockers) = data;

        map.update_blocked_tiles();
        (&positions, &blockers).join().for_each(|(position, _)| map.block_tile_at(position));
    }
}