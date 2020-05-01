/// There can be multiple different warring dynasties in a game.
pub type DynastyId = u8;

/// Nutrients are measured as a counter;
pub type FoodUnit = usize;

// How long before trail goes cold.
const TRAIL_TTL: u16 = 5;

// How likely is it that a new nutrients resource is spawned in a cell. Some
// cells might have probability based on their position.
const FOOD_SPAWN_P: f32 = 0.001;

// How much food is spawned (+-).
const BASE_FOOD_AMOUNT: FoodUnit = 1000;

// How much per tick does the nutrients lose their value. Also affects the nest.
const FOOD_DECAY_RATE: FoodUnit = 3;

// How much food does it cost to spawn a new ant. New ant is spawned whenever
// the nest has enough food to support it. One ant is spawned per tick.
const ANT_SPAWN_COST: FoodUnit = 100;

// How many ticks does a single ant live.
const ANT_TTL: u16 = 1000;

// Initially, how much extra food does a dynasty get. By default it gets at
// least `ANT_SPAWN_COST`, otherwise it's a foobar.
const INITIAL_DYNASTY_EXTRA_FOOD: FoodUnit = 250;

// They say ant can cary more than its weight, right?
const MAX_FOOD_ANT_CAN_CARRY: FoodUnit = 300;

pub struct Environment {
    /// The environment is represented by a grid where the outer vector
    /// represents rows and the inner represents cells, i.e the position in the
    /// outer one is `y` and position in the inner one is `x`.
    pub cells: Vec<Vec<Cell>>,
    /// Dynasty can died off if their ants are wiped out.
    pub dynasties: Vec<Dynasty>,
}

pub struct Dynasty {
    pub id: DynastyId,
    /// Counter for how much food has the dynasty gathered.
    pub food: FoodUnit,
    /// How much ants does a dynasty have alive. If this gets to 0, dynasty is
    /// dead.
    pub ants: usize,
}

/// In another words: up, right, bottom, left. Direction is the inverse of ant's
/// last action. It determines which cells the ant can "sense".
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Ant {
    pub dynasty_id: DynastyId,
    pub carries_food: FoodUnit,
    pub direction: Direction,
    pub ttl: u16,
}

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

impl Default for Cell {
    fn default() -> Self {
        Self::Grass
    }
}

impl Environment {
    pub fn new(size: usize, dynasties: u8) -> Self {
        Self {
            cells: (0..size)
                .map(|_| (0..size).map(|_| Cell::default()).collect())
                .collect(),
            dynasties: (0..dynasties).map(Dynasty::new).collect(),
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
