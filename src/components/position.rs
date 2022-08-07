use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32
}