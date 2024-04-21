use crate::cell::{Cell, Position};
use crate::world::{Direction, World};

mod cell;
mod world;

fn main() {
    let mut world: World = World::init(3, 3);
    world.render()

    // for c in world.cells {
    //     println!("{}", c);
    // }
    // println!("Hello, world!");
}
