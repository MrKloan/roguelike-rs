use std::cmp::{max, min};

use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, RGB, Rltk};
use specs::prelude::*;

use crate::components::{Position, Viewshed};
use crate::map::room::Room;
use crate::player::Player;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub rooms: Vec<Room>,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        let mut map = Map {
            width,
            height,
            tiles: vec![TileType::Wall; (width * height) as usize],
            revealed_tiles: vec![false; (width * height) as usize],
            rooms: Vec::new(),
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let room_width = rng.range(MIN_SIZE, MAX_SIZE);
            let room_height = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, width - room_width - 1) - 1;
            let y = rng.roll_dice(1, height - room_height - 1) - 1;

            let new_room = Room::new(
                Position { x, y },
                room_width,
                room_height,
            );

            if map.is_placement_valid(&new_room) {
                map.add_room(&new_room);

                if !map.rooms.is_empty() {
                    map.link_rooms_with_tunnels(&new_room, &mut rng)
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    fn is_placement_valid(&self, room: &Room) -> bool {
        for other_room in self.rooms.iter() {
            if room.intersect(other_room) {
                return false;
            }
        }

        true
    }

    fn add_room(&mut self, room: &Room) {
        for y in room.first.y + 1..=room.second.y {
            for x in room.first.x + 1..=room.second.x {
                let index = self.index_of(x, y);
                self.tiles[index] = TileType::Floor;
            }
        }
    }

    fn link_rooms_with_tunnels(&mut self, new_room: &Room, rng: &mut RandomNumberGenerator) {
        let new_room_center = new_room.center();
        let previous_room_center = self.rooms[self.rooms.len() - 1].center();

        if rng.range(0, 2) == 1 {
            self.add_horizontal_tunnel(previous_room_center.x, new_room_center.x, previous_room_center.y);
            self.add_vertical_tunnel(previous_room_center.y, new_room_center.y, new_room_center.x);
        } else {
            self.add_vertical_tunnel(previous_room_center.y, new_room_center.y, previous_room_center.x);
            self.add_horizontal_tunnel(previous_room_center.x, new_room_center.x, new_room_center.y);
        }
    }

    fn add_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        let map_size = (self.width * self.height) as usize;

        for x in min(x1, x2)..=max(x1, x2) {
            let index = self.index_of(x, y);
            if index > 0 && index < map_size {
                self.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    fn add_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        let map_size = (self.width * self.height) as usize;

        for y in min(y1, y2)..=max(y1, y2) {
            let index = self.index_of(x, y);
            if index > 0 && index < map_size {
                self.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    pub fn index_of(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn starting_position(&self) -> Position {
        self.rooms[0].center()
    }

    pub fn draw(&self, world: &World, context: &mut Rltk) {
        let mut players = world.write_storage::<Player>();
        let mut viewsheds = world.write_storage::<Viewshed>();

        for (_player, _viewshed) in (&mut players, &mut viewsheds).join() {
            let mut y = 0;
            let mut x = 0;

            for (index, tile) in self.tiles.iter().enumerate() {
                // let tile_point = Point::new(x, y);
                // if viewshed.visible_tiles.contains(&tile_point) {
                if self.revealed_tiles[index] {
                    match tile {
                        TileType::Floor => {
                            context.set(
                                x,
                                y,
                                RGB::from_f32(0.5, 0.5, 0.5),
                                RGB::from_f32(0., 0., 0.),
                                rltk::to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            context.set(
                                x,
                                y,
                                RGB::from_f32(0.0, 1.0, 0.0),
                                RGB::from_f32(0., 0., 0.),
                                rltk::to_cp437('#'),
                            );
                        }
                    }
                }

                x += 1;
                if x > self.width - 1 {
                    x = 0;
                    y += 1;
                }
            }
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, index: usize) -> bool {
        self.tiles[index as usize] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}