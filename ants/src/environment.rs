use crate::dynasty::DynastyAgent;
use crate::ext::*;
use rand::prelude::*;

//--------------------------------- Constants --------------------------------//

// How long before trail goes cold.
const TRAIL_TTL: u16 = 16;

// How likely is it that a new nutrients resource is spawned in a cell. Some
// cells might have probability based on their position.
const FOOD_SPAWN_P: f32 = 0.000004;

// How much food is spawned (+-).
const BASE_FOOD_AMOUNT: FoodUnit = 1000;

// How much per step does the nutrients lose their value. Also affects the nest.
const FOOD_DECAY_RATE: FoodUnit = 3;

// How much food does it cost to spawn a new ant. New ant is spawned whenever
// the nest has enough food to support it. One ant is spawned per step.
const ANT_SPAWN_COST: FoodUnit = 100;

// How many steps does a single ant live. This directly affects how much can a
// dynasty spread around. If the size of the environment is large, the ants
// might not have enough time to travel around to get food and come back.
const ANT_TTL: u16 = 3000;

// Initially, how much extra food does a dynasty get. By default it gets at
// least `ANT_SPAWN_COST`, otherwise it's a foobar.
const INITIAL_DYNASTY_EXTRA_FOOD: FoodUnit = 250;

// They say ant can cary more than its weight, right?
const MAX_FOOD_ANT_CAN_CARRY: FoodUnit = 300;

// How many ants at most can a dynasty have.
const MAX_ANTS_PER_DYNASTY: usize = 200;

// How much food can be stored in a dynasty at most.
const MAX_DYNASTY_FOOD_STOCK: usize = 5000;

//----------------------------------- Types ----------------------------------//

/// There can be multiple different warring dynasties in a game.
pub type DynastyId = u8;

/// Nutrients are measured as a counter.
pub type FoodUnit = usize;

#[derive(Debug)]
pub struct Environment {
    /// The size of the environment, which is a square. Each vector in `cells`
    /// has length of `size`.
    pub size: usize,
    /// The environment is represented by a grid where the outer vector
    /// represents rows and the inner represents cells, i.e the position in the
    /// outer one is `y` and position in the inner one is `x`.
    pub cells: Vec<Vec<Cell>>,
    /// Dynasty can died off if their ants are wiped out. The index is equal to
    /// the dynasty id.
    pub dynasties: Vec<Dynasty>,
    // Cache randomness generator.
    rng: ThreadRng,
    // Caching memory so that it can be reused between steps.
    ant_moves: Vec<AntMove>,
}

#[derive(Clone, Debug)]
pub struct Dynasty {
    /// The same as the index in the array.
    pub id: DynastyId,
    /// Counter for how much food has the dynasty gathered.
    pub food: FoodUnit,
    /// How much ants does a dynasty have alive. If this gets to 0, dynasty is
    /// dead.
    pub ants: usize,
}

/// An action which an ant can take and current rotation of the ant. In another
/// words: up, right, bottom, left. Direction is the inverse of ant's last
/// action. It determines which cells the ant can "sense".
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

// This is the same data as in `Cell::Ant`. Ideally, we would have
// `Cell::Ant(Ant)`, but that yields borrow checker issues.
// This sucks but it's the way I try to fight borrow checker.
#[derive(Clone, Copy, Debug)]
struct Ant {
    dynasty_id: DynastyId,
    carries_food: FoodUnit,
    direction: Direction,
    ttl: u16,
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Grass,
    // TODO: Figure out a way to use `Ant` struct.
    Ant {
        dynasty_id: DynastyId,
        carries_food: FoodUnit,
        direction: Direction,
        ttl: u16,
    },
    /// Where ants have to return with their food. Non related nest is
    /// considered source of food and enemy ants can steal food.
    Nest(DynastyId),
    /// This seems like a bug.
    #[allow(dead_code)]
    Trail {
        dynasty_id: DynastyId,
        /// How long until the trail gets cold.
        ttl: u16,
    },
    /// A counter which describes quantity of nutrition left.
    Food(FoodUnit),
}

#[derive(Clone, Copy, Debug)]
struct AntMove {
    from: (usize, usize),
    ant: Ant,
}

//------------------------------- Support impl -------------------------------//

impl Default for Cell {
    fn default() -> Self {
        Self::Grass
    }
}

impl Environment {
    pub fn new(size: usize, dynasties: u8) -> Self {
        assert!(dynasties > 1);
        assert!(size > dynasties as usize * 5);

        let mut rng = ThreadRng::default();
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
            size,
            rng,
            cells,
            dynasties,
            ant_moves: Vec::new(),
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

impl Cell {
    fn is_grass(&self) -> bool {
        match self {
            Self::Grass => true,
            _ => false,
        }
    }

    fn ant(dynasty_id: DynastyId) -> Self {
        Self::Ant {
            dynasty_id,
            carries_food: 0,
            ttl: ANT_TTL,
            // Arbitrary choice of direction.
            direction: Direction::South,
        }
    }

    fn trail(dynasty_id: DynastyId) -> Self {
        Self::Trail {
            dynasty_id,
            ttl: TRAIL_TTL,
        }
    }
}

impl From<Ant> for Cell {
    fn from(ant: Ant) -> Self {
        Cell::Ant {
            carries_food: ant.carries_food,
            dynasty_id: ant.dynasty_id,
            direction: ant.direction,
            ttl: ant.ttl,
        }
    }
}

impl Direction {
    pub fn rand(rng: &mut ThreadRng) -> Self {
        match rng.gen_range(0, 4) {
            0 => Self::North,
            1 => Self::East,
            2 => Self::West,
            3 => Self::South,
            _ => panic!("Cannot go out of range <0; 5>."),
        }
    }

    // If an ant moves in a given direction from current position in given size
    // if the environment, what's going to be its new position.
    // Returns `None` if the ant cannot move in the direction.
    fn new_coords(
        self,
        x: usize,
        y: usize,
        size: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Self::North if y > 0 => Some((x, y - 1)),
            Self::South if y < size - 1 => Some((x, y + 1)),
            Self::West if x > 0 => Some((x - 1, y)),
            Self::East if x < size - 1 => Some((x + 1, y)),
            _ => None,
        }
    }

    // West is inverse of east, south is inverse of north.
    fn is_inverse(self, another: Self) -> bool {
        match self {
            Self::North => another == Self::South,
            Self::South => another == Self::North,
            Self::West => another == Self::East,
            Self::East => another == Self::West,
        }
    }
}

//------------------------------- World ticking ------------------------------//

impl Environment {
    pub fn step(&mut self, dynasty_agents: &mut [DynastyAgent]) {
        // Updates environment.
        for y in 0..self.size {
            for x in 0..self.size {
                if let Some(update) = self.single_cell_step(x, y) {
                    self.ant_moves.push(update);
                }
            }
        }

        // Moves ants.
        while let Some(ant_move) = self.ant_moves.pop() {
            let dynasty_agent =
                &mut dynasty_agents[ant_move.ant.dynasty_id as usize];
            self.move_ant(dynasty_agent, ant_move);
        }
    }

    // Process for updating environment. It doesn't simulate ant moves because
    // borrow checker wouldn't be happy. All bow to borrow checker.
    fn single_cell_step(&mut self, x: usize, y: usize) -> Option<AntMove> {
        match &mut self.cells[y][x] {
            Cell::Grass => {
                let should_spawn_food = self.rng.roll_dice(FOOD_SPAWN_P);
                if should_spawn_food {
                    // TODO: Make this a distribution.
                    // TODO: Some parts of the environment are more
                    // fruitful (king of the hill).
                    self.cells[y][x] = Cell::Food(BASE_FOOD_AMOUNT);
                }
                None
            }
            Cell::Trail { ttl, .. } => {
                if *ttl == 0 {
                    self.cells[y][x] = Cell::Grass
                } else {
                    *ttl -= 1;
                }
                None
            }
            Cell::Nest(dynasty_id) => {
                let dynasty_id = *dynasty_id;
                let d_i = dynasty_id as usize;
                if self.dynasties[d_i].food >= ANT_SPAWN_COST
                    && self.dynasties[d_i].ants <= MAX_ANTS_PER_DYNASTY
                {
                    // If there's enough food to spawn a new ant, try to
                    // find an empty cell where to put it.
                    // Because we spawned nests randomly, but made sure
                    // not on the edges of the environment, this
                    // shouldn't overflow.
                    for inc in 0..=3 {
                        let cell = &mut self.cells[y + 1][x - 1 + inc];
                        if cell.is_grass() {
                            *cell = Cell::ant(dynasty_id);
                            self.dynasties[d_i].food -= ANT_SPAWN_COST;
                            self.dynasties[d_i].ants += 1;
                            break;
                        }
                    }
                }

                // We want to decay food in the nest, so that we can
                // kill off a dynasty which has low food and no ants.
                self.dynasties[d_i].food =
                    self.dynasties[d_i].food.saturating_sub(FOOD_DECAY_RATE);
                None
            }
            Cell::Food(amount) => {
                if *amount < FOOD_DECAY_RATE {
                    self.cells[y][x] = Cell::Grass;
                } else {
                    *amount -= FOOD_DECAY_RATE;
                }
                None
            }
            Cell::Ant {
                ttl,
                direction,
                dynasty_id,
                carries_food,
            } => {
                if *ttl == 0 {
                    self.dynasties[*dynasty_id as usize].ants -= 1;
                    self.cells[y][x] =
                        Cell::Food(*carries_food + ANT_SPAWN_COST / 2);
                    None
                } else {
                    *ttl -= 1;
                    Some(AntMove {
                        from: (x, y),
                        ant: Ant {
                            ttl: *ttl,
                            direction: *direction,
                            dynasty_id: *dynasty_id,
                            carries_food: *carries_food,
                        },
                    })
                }
            }
        }
    }

    fn move_ant(
        &mut self,
        dynasty_agent: &mut DynastyAgent,
        ant_move: AntMove,
    ) {
        let AntMove {
            from: (x, y),
            mut ant,
        } = ant_move;
        // Let the agent do its magic and spit out an action.
        ant.direction = dynasty_agent.pick_action(&self.cells);

        // By default the ant gets a negative reward, also known as penalty for
        // breathing.
        let mut reward = -1.0;
        if let Some((new_x, new_y)) = ant.direction.new_coords(x, y, self.size)
        {
            match self.cells[new_y][new_x] {
                // Move ant.
                Cell::Grass | Cell::Trail { .. } => {
                    self.cells[new_y][new_x] = ant.into();
                    self.cells[y][x] = Cell::trail(ant.dynasty_id);
                }
                Cell::Ant {
                    direction,
                    dynasty_id,
                    carries_food,
                    ..
                } => {
                    if dynasty_id != ant.dynasty_id {
                        // If both ants point against each other, they fight.
                        // Otherwise our ant feeds on the one in the target cell.
                        if direction.is_inverse(ant.direction) {
                            // todo!("Fight!");
                        } else {
                            let can_carry =
                                MAX_FOOD_ANT_CAN_CARRY - ant.carries_food;
                            if can_carry >= carries_food {
                                ant.carries_food += carries_food;
                            } else {
                                ant.carries_food = MAX_FOOD_ANT_CAN_CARRY;
                            }
                            reward = 1.0;
                            self.cells[y][x] = Cell::trail(ant.dynasty_id);
                            self.cells[new_y][new_x] = ant.into();
                        }
                    }
                }
                Cell::Food(amount) => {
                    // Do we want to reward based on amount of food
                    // picked up?
                    reward = 1.0;
                    let can_carry = MAX_FOOD_ANT_CAN_CARRY - ant.carries_food;

                    // Takes as much food as the little guy can.
                    if can_carry >= amount {
                        ant.carries_food += amount;
                        self.cells[new_y][new_x] = Cell::Grass;
                    } else {
                        ant.carries_food = MAX_FOOD_ANT_CAN_CARRY;
                        self.cells[new_y][new_x] =
                            Cell::Food(amount - can_carry);
                    }

                    self.cells[y][x] = ant.into();
                }
                Cell::Nest(dynasty_id) => {
                    let d_i = dynasty_id as usize;
                    let ant_dynasty_id = ant.dynasty_id;
                    if ant_dynasty_id == dynasty_id {
                        // Disposes of food if any and there's space in the nest.
                        if ant.carries_food > 0
                            && self.dynasties[d_i].food
                                < MAX_DYNASTY_FOOD_STOCK - FOOD_DECAY_RATE
                        {
                            self.dynasties[d_i].food += self.dynasties[d_i]
                                .food
                                .saturating_add(ant.carries_food);
                            reward = 1.0;
                            self.cells[y][x] = ant.into();
                        }
                    } else if self.dynasties[d_i].food > 0 {
                        let can_carry =
                            MAX_FOOD_ANT_CAN_CARRY - ant.carries_food;

                        // Loot enemy nest as much as the ant can.
                        if can_carry >= self.dynasties[d_i].food {
                            ant.carries_food += self.dynasties[d_i].food;
                            self.dynasties[d_i].food = 0;
                        } else {
                            ant.carries_food = MAX_FOOD_ANT_CAN_CARRY;
                            self.dynasties[d_i].food -= can_carry;
                        }
                        reward = 1.0;
                        self.cells[y][x] = ant.into();
                    }
                }
            }
        }

        dynasty_agent.reward(reward);
    }
}
