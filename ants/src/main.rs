mod dynasty;
mod environment;
mod ext;

use dynasty::DynastyAgent;
use environment::Environment;
use std::fs::File;
use std::io::Write;

// How many cells in a row and in a column.
const GRID_SIZE: usize = 200;

// How many warring ant dynasties.
const DYNASTIES: u8 = 5;

const SIMULATED_ENVS: usize = 11;

fn main() {
    let mut stats = File::create("debug/data.txt").unwrap();
    let mut dynasty_agents: Vec<_> = (0..DYNASTIES)
        .map(|id| DynastyAgent::new(id, GRID_SIZE))
        .collect();

    for env_n in 0..SIMULATED_ENVS {
        let mut environment = Environment::new(GRID_SIZE, DYNASTIES);
        loop {
            environment.step(&mut dynasty_agents);

            if env_n % 10 == 0 && environment.steps % 100 == 0 {
                render_value_function(&environment, &dynasty_agents[0]);
            }

            if environment.is_finished() {
                // debug
                if environment.steps > 10000 {
                    render(env_n, &environment);
                }
                break;
            }
        }
        println!("#{} steps: {}", env_n, environment.steps);
        write!(stats, "{},", environment.steps).unwrap();
        environment.reward_winner(&mut dynasty_agents);
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

fn render_value_function(
    environment: &Environment,
    dynasty_agent: &DynastyAgent,
) {
    let size = environment.size as u32;
    let mut image = image::DynamicImage::new_rgb8(size, size);
    let image_view = image.as_mut_rgb8().unwrap();

    for (y, row) in dynasty_agent.state_values_with_food.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let pixel = image_view.get_pixel_mut(x as u32, y as u32);
            let c = ((value + 10.0).max(0.0) * 20.0).max(255.0) as u8;
            pixel.0 = [c, c, c];
        }
    }
    image
        .save(format!(
            "debug/{}-f-{}-w.png",
            environment.steps, dynasty_agent.dynasty_id
        ))
        .expect("Cannot save image");

    let mut image = image::DynamicImage::new_rgb8(size, size);
    let image_view = image.as_mut_rgb8().unwrap();
    for (y, row) in dynasty_agent.state_values_without_food.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            let pixel = image_view.get_pixel_mut(x as u32, y as u32);
            let c = ((value + 10.0).max(0.0) * 20.0).min(255.0) as u8;
            pixel.0 = [c, c, c];
        }
    }
    image
        .save(format!(
            "debug/{}-f-{}-n.png",
            environment.steps, dynasty_agent.dynasty_id
        ))
        .expect("Cannot save image");
}
