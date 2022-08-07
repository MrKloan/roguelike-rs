use rltk::console;
use specs::prelude::*;
use specs_derive::Component;

use super::{Position, Viewshed};

#[derive(Component, Debug)]
pub struct Monster {}

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (monsters, viewsheds, positions) = data;

        for (_monster, _viewshed, _position) in (&monsters, &viewsheds, &positions).join() {
            console::log("Monster considers their own existence");
        }
    }
}