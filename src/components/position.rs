use rltk::Point;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<&Position> for Point {
    fn from(position: &Position) -> Self {
        Point::new(position.x, position.y)
    }
}