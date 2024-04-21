use crate::cell::{Cell, Position};
use crate::world::{Direction, World};

mod cell;
mod world;

fn main() {
    let mut world: World = World::init(3, 3);

    match world.get_cell_neighbour(&world.cells[0], Direction::Right) {
        Some(cell) => println!("Neighbour: {}", cell),
        None => println!("This neighbour don't exist"),
    }

    // for c in world.cells {
    //     println!("{}", c);
    // }
    println!("Hello, world!");
}
