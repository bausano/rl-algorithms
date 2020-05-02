use crate::ext::*;
use rand::prelude::*;

// -------------------------------- Constants --------------------------------//

// How long before trail goes cold.
const TRAIL_TTL: u16 = 5;

// How likely is it that a new nutrients resource is spawned in a cell. Some
// cells might have probability based on their position.
const FOOD_SPAWN_P: f32 = 0.001;

// How much food is spawned (+-).
const BASE_FOOD_AMOUNT: FoodUnit = 1000;

// How much per step does the nutrients lose their value. Also affects the nest.
const FOOD_DECAY_RATE: FoodUnit = 3;

// How much food does it cost to spawn a new ant. New ant is spawned whenever
// the nest has enough food to support it. One ant is spawned per step.
const ANT_SPAWN_COST: FoodUnit = 100;

// How many steps does a single ant live.
const ANT_TTL: u16 = 2500;

// Initially, how much extra food does a dynasty get. By default it gets at
// least `ANT_SPAWN_COST`, otherwise it's a foobar.
const INITIAL_DYNASTY_EXTRA_FOOD: FoodUnit = 250;

// They say ant can cary more than its weight, right?
const MAX_FOOD_ANT_CAN_CARRY: FoodUnit = 300;

// ---------------------------------- Types ----------------------------------//

/// There can be multiple different warring dynasties in a game.
pub type DynastyId = u8;

/// Nutrients are measured as a counter;
pub type FoodUnit = usize;

pub struct Environment {
    /// The environment is represented by a grid where the outer vector
    /// represents rows and the inner represents cells, i.e the position in the
    /// outer one is `y` and position in the inner one is `x`.
    pub cells: Vec<Vec<Cell>>,
    /// Dynasty can died off if their ants are wiped out.
    pub dynasties: Vec<Dynasty>,
    // Some goodies.
    rng: ThreadRng,
}

pub struct Dynasty {
    pub id: DynastyId,
    /// Counter for how much food has the dynasty gathered.
    pub food: FoodUnit,
    /// How much ants does a dynasty have alive. If this gets to 0, dynasty is
    /// dead.
    pub ants: usize,
}

/// An action which an ant can take. In another words: up, right, bottom, left.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Clone, Copy, Debug)]
pub struct Ant {
    pub dynasty_id: DynastyId,
    pub carries_food: FoodUnit,
    pub ttl: u16,
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Grass,
    Ant(Ant),
    /// Where ants have to return with their food. Non related nest is
    /// considered source of food and enemy ants can steal food.
    Nest(DynastyId),
    Trail {
        dynasty_id: DynastyId,
        /// How long until the trail gets cold.
        ttl: u16,
    },
    /// A counter which describes quantity of nutrition left.
    Food(FoodUnit),
}

// ------------------------------ Support impl -------------------------------//

impl Default for Cell {
    fn default() -> Self {
        Self::Grass
    }
}

impl Environment {
    pub fn new(size: usize, dynasties: u8) -> Self {
        assert!(dynasties > 1);
        assert!(size > dynasties as usize * 5);
        let mut rng = thread_rng();
        let dynasties: Vec<Dynasty> =
            (0..dynasties).map(Dynasty::new).collect();
        let mut cells: Vec<Vec<Cell>> = (0..size)
            .map(|_| (0..size).map(|_| Cell::default()).collect())
            .collect();

        // Each dynasty nest is randomly positioned. We avoid edges for
        // simplification of new ant placing logic.
        for dynasty in &dynasties {
            let nest_x = rng.gen_range(1, size - 1);
            let nest_y = rng.gen_range(1, size - 1);
            cells[nest_y][nest_x] = Cell::Nest(dynasty.id);
        }

        Self {
            rng,
            cells,
            dynasties,
        }
    }
}

impl Dynasty {
    pub fn new(id: DynastyId) -> Self {
        Self {
            id,
            food: ANT_SPAWN_COST + INITIAL_DYNASTY_EXTRA_FOOD,
            ants: 0,
        }
    }
}

// ------------------------------ XXX -----------------------------//

impl Environment {
    pub fn step(&mut self) {
        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                match cell {
                    Cell::Grass => {
                        let should_spawn_food =
                            self.rng.roll_dice(FOOD_SPAWN_P);
                        if should_spawn_food {
                            // TODO: Make this a distribution.
                            *cell = Cell::Food(BASE_FOOD_AMOUNT);
                        }
                    }
                    Cell::Trail { ttl, .. } => {
                        if *ttl == 0 {
                            *cell = Cell::Grass
                        } else {
                            *ttl -= 1;
                        }
                    }
                    Cell::Nest(dynasty_id) => {
                        let i = *dynasty_id as usize;
                        self.dynasties[i].food = self.dynasties[i]
                            .food
                            .saturating_sub(FOOD_DECAY_RATE);
                        // TODO: Find empty field around to spawn new ant.
                    }
                    Cell::Food(amount) => {
                        if *amount < FOOD_DECAY_RATE {
                            *cell = Cell::Grass
                        } else {
                            *amount -= FOOD_DECAY_RATE;
                        }
                    }
                    Cell::Ant(ant) => {
                        //
                    }
                }
            }
        }
    }
}
