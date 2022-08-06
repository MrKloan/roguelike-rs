use std::cmp::{max, min};

use rltk::{RandomNumberGenerator, RGB, Rltk};

use crate::map::rectangle::Rectangle;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_index(x: i32, y: i32) -> usize {
    (y as usize * crate::WIDTH as usize) + x as usize
}

pub fn new_map(width: i32, height: i32) -> (Vec<Rectangle>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; (width * height) as usize];

    let mut rooms: Vec<Rectangle> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let room_width = rng.range(MIN_SIZE, MAX_SIZE);
        let room_height = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, width - room_width - 1) - 1;
        let y = rng.roll_dice(1, height - room_height - 1) - 1;

        let new_room = Rectangle::new(x, y, room_width, room_height);

        if is_placement_valid(&rooms, &new_room) {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                link_rooms_with_tunnels(&mut rooms, &new_room, &mut map, &mut rng)
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}

fn is_placement_valid(rooms: &Vec<Rectangle>, new_room: &Rectangle) -> bool {
    for other_room in rooms.iter() {
        if new_room.intersect(other_room) {
            return false;
        }
    }

    true
}

fn apply_room_to_map(room: &Rectangle, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[map_index(x, y)] = TileType::Floor;
        }
    }
}

fn link_rooms_with_tunnels(rooms: &Vec<Rectangle>, new_room: &Rectangle, mut map: &mut Vec<TileType>, rng: &mut RandomNumberGenerator) {
    let new_room_center = new_room.center();
    let previous_room_center = rooms[rooms.len() - 1].center();

    if rng.range(0, 2) == 1 {
        apply_horizontal_tunnel(&mut map, previous_room_center.x, new_room_center.x, previous_room_center.y);
        apply_vertical_tunnel(&mut map, previous_room_center.y, new_room_center.y, new_room_center.x);
    } else {
        apply_vertical_tunnel(&mut map, previous_room_center.y, new_room_center.y, previous_room_center.x);
        apply_horizontal_tunnel(&mut map, previous_room_center.x, new_room_center.x, new_room_center.y);
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = map_index(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = map_index(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

pub fn draw_map(map: &[TileType], context: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
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

        // Move the coordinates
        x += 1;
        if x > crate::WIDTH - 1 {
            x = 0;
            y += 1;
        }
    }
}