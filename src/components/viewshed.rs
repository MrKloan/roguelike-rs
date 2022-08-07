use specs::prelude::*;
use specs_derive::Component;

use crate::Map;

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