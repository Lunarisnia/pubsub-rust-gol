use std::{io, time};
use crate::world::{Direction, World};

mod cell;
mod world;

fn main() {
    let now = time::Instant::now();
    let mut world: World = World::init(4, 4);

    // Glider
    // world.cells[1].status = true;
    // world.cells[6].status = true;
    // world.cells[8].status = true;
    // world.cells[9].status = true;
    // world.cells[10].status = true;

    for _ in 0..3000 {
        print!("{}[2J", 27 as char);
        world.render();
        std::thread::sleep(time::Duration::from_millis(250));
        world.step();
    }
    println!("It take: {} seconds to simulate", now.elapsed().as_secs());
}
