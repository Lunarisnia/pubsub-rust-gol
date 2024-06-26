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


impl Direction {
    const VALUES: [Self; 8] = [
        Self::Up,
        Self::UpRight,
        Self::Right,
        Self::RightDown,
        Self::Down,
        Self::DownLeft,
        Self::Left,
        Self::LeftUp,
    ];
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

    fn calculate(&self) -> Vec<(usize, bool)> {
        let mut update_list: Vec<(usize, bool)> = Vec::new();
        for (i, cell) in self.cells.iter().enumerate() {
            let mut active_neighbour_count = 0;

            for direction in Direction::VALUES {
                match self.get_cell_neighbour(&cell, direction) {
                    Some(c) => {
                        if c.status {
                            active_neighbour_count += 1
                        }
                    }
                    None => ()
                };
            }

            if cell.status {
                if active_neighbour_count < 2 || active_neighbour_count > 3 {
                    update_list.push((i, false));
                } else if active_neighbour_count == 2 || active_neighbour_count == 3 {
                    update_list.push((i, true));
                }
            } else {
                if active_neighbour_count == 3 {
                    update_list.push((i, true));
                }
            }
        }

        update_list
    }

    pub fn step(&mut self) {
        let update_list: Vec<(usize, bool)> = self.calculate();
        for (coordinate, new_status) in update_list {
            self.cells[coordinate].status = new_status;
        }
    }

    pub fn render(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            if i % self.width as usize == 0 {
                println!();
            }
            print!(" {} ", if cell.status { "0" } else { "." });
            // print!(" {} ", cell.status);
        }
        println!();
    }
}