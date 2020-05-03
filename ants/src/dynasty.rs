//! Includes the RL inspired logic.
use crate::environment::{Cell, Direction, DynastyId};
use crate::ext::*;
use rand::prelude::*;
use std::collections::HashMap;

// Chance to take a random action.
const EXPLORATION_P: f32 = 0.01;

// How fast do state values propagate.
const STEP_SIZE: f32 = 0.2;

type State = [u8; 9];

pub struct DynastyAgent {
    dynasty_id: DynastyId,
    /// Agent's estimation of states.
    state_values: HashMap<State, f32>,
    // Roll the dice for exploratory moves.
    rng: ThreadRng,
}

impl DynastyAgent {
    pub fn new(dynasty_id: DynastyId) -> Self {
        Self {
            dynasty_id,
            state_values: HashMap::default(),
            rng: ThreadRng::default(),
        }
    }

    /// Picks an action with the best value according to current value function
    /// or occasionally explores by picking the best value.
    /// The reward signal is for previous turn.
    pub fn pick_action(
        &mut self,
        ant_x: usize,
        ant_y: usize,
        reward: f32,
        cells: &[Vec<Cell>],
    ) -> Direction {
        let current_state = get_state_at(
            self.dynasty_id,
            cells,
            ant_x as isize,
            ant_y as isize,
        );
        let current_state_value = {
            // Get the value of the current state.
            let current_state_value =
                self.state_values.entry(current_state).or_insert(0.5);

            // Updates the current state value by the reward function.
            *current_state_value += STEP_SIZE * (reward - *current_state_value);
            *current_state_value
        };

        if self.rng.roll_dice(EXPLORATION_P) {
            return Direction::rand(&mut self.rng);
        }

        // Possible actions in the environment.
        let actions = &[
            (Direction::North, (ant_x as isize, ant_y as isize - 1)),
            (Direction::South, (ant_x as isize, ant_y as isize + 1)),
            (Direction::West, (ant_x as isize - 1, ant_y as isize)),
            (Direction::East, (ant_x as isize + 1, ant_y as isize)),
        ];

        let mut best_action_value = 0.0;
        // Arbitrary first action.
        let mut best_action = Direction::North;

        // Finds the best action to take.
        for (direction, (x, y)) in actions {
            let state = get_state_at(self.dynasty_id, cells, *x, *y);
            let action_value = *self.state_values.entry(state).or_insert(0.5);

            if action_value > best_action_value {
                best_action_value = action_value;
                best_action = *direction;
            }
        }

        // Temporal difference update.
        self.state_values.insert(
            current_state,
            STEP_SIZE * (best_action_value - current_state_value),
        );

        best_action
    }
}

impl Cell {
    // Each cell state is assigned some numerical value, so that it can be
    // stored easily in a hash map as a key.
    fn to_byte(&self, with_regards_to_dynasty: DynastyId) -> u8 {
        match self {
            Self::Wall => 0,
            Self::Grass => 1,
            Self::Food(_) => 2,
            Self::Nest(id) if id == &with_regards_to_dynasty => 3,
            Self::Nest(_) => 4,
            Self::Trail { dynasty_id, .. }
                if dynasty_id == &with_regards_to_dynasty =>
            {
                5
            }
            Self::Trail { .. } => 6,
            Self::Ant { dynasty_id, .. }
                if dynasty_id == &with_regards_to_dynasty =>
            {
                7
            }
            Self::Ant { .. } => 7,
        }
    }
}

// Returns 3x3 view into the environment with center at given coordinates.
fn get_state_at(
    dynasty_id: DynastyId,
    cells: &[Vec<Cell>],
    around_x: isize,
    around_y: isize,
) -> State {
    let cell_value = |x: isize, y: isize| -> u8 {
        if x < 0 || y < 0 {
            return Cell::Wall.to_byte(dynasty_id);
        }

        match cells.get(y as usize) {
            None => Cell::Wall.to_byte(dynasty_id),
            Some(row) => match row.get(x as usize) {
                None => Cell::Wall.to_byte(dynasty_id),
                Some(cell) => cell.to_byte(dynasty_id),
            },
        }
    };

    let mut state = State::default();
    for y in 0..3 {
        for x in 0..3 {
            state[(x + y * 3) as usize] =
                cell_value(around_x - 1 + x, around_y - 1 + y);
        }
    }

    state
}
