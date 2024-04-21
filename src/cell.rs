use std::fmt::{Display, Formatter};

pub struct Position(pub u32, pub u32);

impl Position {
    pub fn set(&mut self, x : u32, y : u32) {
        self.0 = y;
        self.1 = x;
    }
}

pub struct Cell {
    pub status: bool,
    pub position: Position,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Position: {}, {}; Status: {})", self.position.0, self.position.1, self.status)
    }
}

impl Cell {
    pub fn is_active(&self) -> bool {
        self.status
    }

    pub fn get_position(&self) -> Position {
        Position(self.position.0, self.position.1)
    }
}