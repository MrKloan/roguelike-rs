use specs::prelude::*;
use specs_derive::Component;

use crate::components::Position;

#[derive(Component)]
pub struct LeftWalker {}

pub struct WalkLeft {}

impl<'a> System<'a> for WalkLeft {
    type SystemData = (ReadStorage<'a, LeftWalker>, WriteStorage<'a, Position>);

    fn run(&mut self, (left_walkers, mut positions): Self::SystemData) {
        for (_left_walker, position) in (&left_walkers, &mut positions).join() {
            position.x -= 1;
            if position.x < 0 {
                position.x = 80 - 1;
            }
        }
    }
}