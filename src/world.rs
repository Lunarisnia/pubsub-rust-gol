use crate::cell::{Cell, Position};

pub struct World {
    pub height: u32,
    pub width: u32,
    pub cells: Vec<Cell>,
}

pub enum Direction {
    Up,
    UpRight,
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    LeftUp,
}

impl World {
    pub fn init(width: u32, height: u32) -> World {
        let mut new_world = World {
            width,
            height,
            cells: Vec::new(),
        };
        for i in 0..(width * height) {
            // This translate 1D coordinate to 2D coordinate
            let x = i % width;
            let y = i / width;

            new_world.cells.push(Cell {
                status: false,
                position: Position(y, x),
            })
        }

        new_world
    }

    fn translate_2d_to_1d(coordinate: Position, width: u32) -> u32 {
        coordinate.1 + width * coordinate.0
    }

    pub fn get_cell_neighbour(&self, cell: &Cell, direction: Direction) -> Option<&Cell> {
        match direction {
            Direction::Up => {
                if cell.position.0 == 0 {
                    return None;
                }
                let neighbour = Self::translate_2d_to_1d(Position(cell.position.0 - 1, cell.position.1), self.width);

                match self.cells.get(neighbour as usize) {
                    Some(c) => Some(c),
                    None => None
                }
            }
            Direction::UpRight => {
                if cell.position.0 == 0 || cell.position.1 + 1 >= self.width {
                    return None;
                }
                let neighbour = Self::translate_2d_to_1d(Position(cell.position.0 - 1, cell.position.1 + 1), self.width);

                match self.cells.get(neighbour as usize) {
                    Some(c) => Some(c),
                    None => None
                }
            }
            Direction::Right => {
                if cell.position.1 + 1 >= self.width {
                    return None;
                }

                let neighbour = Self::translate_2d_to_1d(Position(cell.position.0, cell.position.1 + 1), self.width);

                match self.cells.get(neighbour as usize) {
                    Some(c) => Some(c),
                    None => None,
                }
            }
            Direction::RightDown => {
                if cell.position.1 + 1 >= self.width || cell.position.0 + 1 >= self.height {
                    return None;
                }

                let neighbour = Self::translate_2d_to_1d(Position(cell.position.0 + 1, cell.position.1 + 1), self.width);

                match self.cells.get(neighbour as usize) {
                    Some(c) => Some(c),
                    None => None,
                }
            }
            Direction::Down => {
                if cell.position.0 + 1 >= self.height {
                    return None;
                }

                let neighbour = Self::translate_2d_to_1d(Position(cell.position.0 + 1, cell.position.1), self.width);

                match self.cells.get(neighbour as usize) {
                    Some(c) => Some(c),
                    None => None,
                }
            }
            Direction::DownLeft => {
                if cell.position.0 + 1 >= self.height || cell.position.1 == 0 {
                    return None;
                }

                let neighbour = Self::translate_2d_to_1d(Position(cell.position.0 + 1, cell.position.1 - 1), self.width);

                match self.cells.get(neighbour as usize) {
                    Some(c) => Some(c),
                    None => None,
                }
            }
            Direction::Left => {
                if cell.position.1 == 0 {
                    return None;
                }

                let neighbour = Self::translate_2d_to_1d(Position(cell.position.0, cell.position.1 - 1), self.width);

                match self.cells.get(neighbour as usize) {
                    Some(c) => Some(c),
                    None => None,
                }
            }
            Direction::LeftUp => {
                if cell.position.0 == 0 || cell.position.1 == 0 {
                    return None;
                }

                let neighbour = Self::translate_2d_to_1d(Position(cell.position.0 - 1, cell.position.1 - 1), self.width);

                match self.cells.get(neighbour as usize) {
                    Some(c) => Some(c),
                    None => None,
                }
            }
        }
    }
}