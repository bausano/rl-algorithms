mod environment;
mod ext;

use environment::Environment;
use std::time::{Duration, Instant};

// How many cells in a row and in a column.
const GRID_SIZE: usize = 100;

// How many warring ant dynasties.
const DYNASTIES: u8 = 2;

fn main() {
    let mut environment = Environment::new(GRID_SIZE, DYNASTIES);

    let mut elapsed = 0;
    let steps = 100;
    for _ in 0..steps {
        let now = Instant::now();
        environment.step();
        elapsed += now.elapsed().as_millis();
    }
    println!("avg ms {}", elapsed / steps);
}
