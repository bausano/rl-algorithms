mod dynasty;
mod environment;
mod ext;

use dynasty::DynastyAgent;
use environment::Environment;
use std::time::Instant;

// How many cells in a row and in a column.
const GRID_SIZE: usize = 256;

// How many warring ant dynasties.
const DYNASTIES: u8 = 5;

fn main() {
    let mut dynasty_agents: Vec<_> =
        (0..DYNASTIES).map(DynastyAgent::new).collect();

    // Places the game of life 100 times.
    for env_n in 0..10000 {
        let mut elapsed = 0;
        let mut environment = Environment::new(GRID_SIZE, DYNASTIES);
        loop {
            let now = Instant::now();
            environment.step(&mut dynasty_agents);
            elapsed += now.elapsed().as_micros();

            if environment.steps % 4000 == 0 {
                render(env_n, &environment);
            }

            if environment.is_finished() {
                if environment.steps > 5000 {
                    render(env_n, &environment);
                }
                break;
            }
        }

        println!("Env #{}", env_n);
        println!("Steps: {}", environment.steps);
        println!("Avg step uqs {}", elapsed / environment.steps as u128);
        println!();
    }
}

// TODO: For debug now.
// https://color.adobe.com/create
// https://www.rapidtables.com/convert/color/hex-to-rgb.html
fn render(env: usize, environment: &Environment) {
    use environment::Cell;
    const DYN_NEST_COLOURS: &[[u8; 3]] = &[
        [139, 0, 0],
        [87, 128, 94],
        [0, 69, 133],
        [109, 0, 133],
        [133, 115, 0],
    ];
    const DYN_ANT_COLOURS: &[[u8; 3]] = &[
        [255, 30, 0],
        [87, 255, 117],
        [87, 174, 255],
        [224, 87, 255],
        [255, 233, 87],
    ];
    const DYN_TRAIL_COLOURS: &[[u8; 3]] = &[
        [254, 181, 172],
        [188, 255, 202],
        [189, 223, 255],
        [243, 189, 255],
        [255, 246, 189],
    ];

    let size = environment.size as u32;
    let mut image = image::DynamicImage::new_rgb8(size, size);
    let image_view = image.as_mut_rgb8().unwrap();

    for (y, row) in environment.cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let pixel = image_view.get_pixel_mut(x as u32, y as u32);
            match cell {
                Cell::Wall => pixel.0 = [0, 0, 0],
                Cell::Grass => pixel.0 = [255, 255, 255],
                Cell::Trail { dynasty_id, .. } => {
                    pixel.0 = DYN_TRAIL_COLOURS[*dynasty_id as usize]
                }
                Cell::Food(_) => pixel.0 = [200, 200, 200],
                Cell::Nest(dynasty_id) => {
                    pixel.0 = DYN_NEST_COLOURS[*dynasty_id as usize]
                }
                Cell::Ant { dynasty_id, .. } => {
                    pixel.0 = DYN_ANT_COLOURS[*dynasty_id as usize]
                }
            }
        }
    }

    image
        .save(format!("debug/e{}-t{}.png", env, environment.steps))
        .expect("Cannot save image");
}
