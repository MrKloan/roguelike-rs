use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Statistics {
    pub max_health: i32,
    pub health: i32,
    pub defense: i32,
    pub attack: i32,
}