use rltk::Point;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
}