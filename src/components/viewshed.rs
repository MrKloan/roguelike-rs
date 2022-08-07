use rltk::Point;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Viewshed {
    pub range: i32,
    pub visible_tiles: Vec<Point>,
    pub should_update: bool,
}

impl Viewshed {
    pub fn new(range: i32) -> Viewshed {
        Viewshed {
            range,
            visible_tiles: Vec::new(),
            should_update: true,
        }
    }
}