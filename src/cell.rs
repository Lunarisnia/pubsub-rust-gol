pub struct Position(pub usize, pub usize);

pub struct Cell {
    pub status: bool,
    pub position: Position,
}

impl Cell {
    pub fn is_active(&self) -> bool {
        self.status
    }

    pub fn get_position(&self) {
        return
    }
}