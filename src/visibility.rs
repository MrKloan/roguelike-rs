use rltk::{field_of_view, Point};
use specs::prelude::*;
use specs_derive::Component;

use crate::components::Position;
use crate::map::Map;

#[derive(Component)]
pub struct Viewshed {
    pub range: i32,
    pub visible_tiles: Vec<bool>,
    pub revealed_tiles: Vec<bool>,
    pub should_update: bool,
}

impl Viewshed {
    pub fn new(range: i32, map: &Map) -> Viewshed {
        Viewshed {
            range,
            visible_tiles: vec![false; (map.width * map.height) as usize],
            revealed_tiles: vec![false; (map.width * map.height) as usize],
            should_update: true,
        }
    }
}

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut viewsheds, positions) = data;

        for (viewshed, position) in (&mut viewsheds, &positions).join() {
            if !viewshed.should_update {
                continue;
            }

            viewshed.should_update = false;
            viewshed.visible_tiles.fill(false);

            field_of_view(Point::new(position.x, position.y), viewshed.range, &*map)
                .iter()
                .filter(|point| map.is_in_bound(point.x, point.y))
                .for_each(|point| {
                    let index = map.index_of(point.x, point.y);
                    viewshed.visible_tiles[index] = true;
                    viewshed.revealed_tiles[index] = true;
                });
        }
    }
}