use crate::components::Position;

pub struct Room {
    pub first: Position,
    pub second: Position,
}

impl Room {
    pub fn new(position: Position, width: i32, height: i32) -> Room {
        let second = Position { x: position.x + width, y: position.y + height };
        Room {
            first: position,
            second,
        }
    }

    pub fn intersect(&self, other: &Room) -> bool {
        self.first.x <= other.second.x
            && self.second.x >= other.first.x
            && self.first.y <= other.second.y
            && self.second.y >= other.first.y
    }

    pub fn center(&self) -> Position {
        Position {
            x: (self.first.x + self.second.x) / 2,
            y: (self.first.y + self.second.y) / 2,
        }
    }
}