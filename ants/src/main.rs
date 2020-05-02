mod environment;
mod ext;

use environment::Environment;

// How many cells in a row and in a column.
const GRID_SIZE: usize = 1500;

// How many warring ant dynasties.
const DYNASTIES: u8 = 2;

fn main() {
    let mut environment = Environment::new(GRID_SIZE, DYNASTIES);

    environment.step();
}
