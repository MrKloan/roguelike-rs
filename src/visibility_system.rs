use rltk::{field_of_view, Point};
use specs::prelude::*;

use crate::components::{Position, Viewshed};
use crate::map::Map;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut viewsheds, positions) = data;

        for (viewshed, position) in (&mut viewsheds, &positions).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(position.x, position.y), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|point| point.x >= 0 && point.x < map.width && point.y >= 0 && point.y < map.height);
        }
    }
}