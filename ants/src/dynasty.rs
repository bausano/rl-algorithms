//! Includes the RL inspired logic.
use crate::environment::{Ant, Direction, DynastyId, MAX_FOOD_ANT_CAN_CARRY};
use crate::ext::*;
use rand::prelude::*;

// Chance to take a random action.
const EXPLORATION_P: f32 = 0.05;

// How fast do state values propagate.
const STEP_SIZE: f32 = 0.2;

// The status or the 3x3 cells and the bool which says whether the ant cannot
// carry more food.
type State = Vec<Vec<f32>>;

pub struct DynastyAgent {
    dynasty_id: DynastyId,
    /// Agent's estimation of how good is it being in certain cell if the ant
    /// carries maximum food.
    state_values_with_food: State,
    /// Agent's estimation of how good is it being in a certain cell if the
    // ant still can carry more food.
    state_values_without_food: State,
    // Roll the dice for exploratory moves.
    rng: ThreadRng,
}

impl DynastyAgent {
    pub fn new(dynasty_id: DynastyId, size: usize) -> Self {
        let mut rng = ThreadRng::default();
        let mut empty_state_values = || {
            (0..size)
                .map(|_| (0..size).map(|_| rng.gen_range(0.4, 0.6)).collect())
                .collect()
        };

        Self {
            dynasty_id,
            state_values_with_food: empty_state_values(),
            state_values_without_food: empty_state_values(),
            rng,
        }
    }

    /// Picks an action with the best value according to current value function
    /// or occasionally explores by picking the best value.
    /// The reward signal is for previous turn.
    pub fn pick_action(
        &mut self,
        ant_x: usize,
        ant_y: usize,
        ant: Ant,
    ) -> Direction {
        let state_values = if ant.carries_food == MAX_FOOD_ANT_CAN_CARRY {
            &mut self.state_values_with_food
        } else {
            &mut self.state_values_without_food
        };

        state_values[ant_y][ant_x] =
            STEP_SIZE * (ant.reward - state_values[ant_y][ant_x]);

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
        let mut best_action = None;

        // Finds the best action to take.
        for (direction, (x, y)) in actions {
            if let Some(action_value) =
                get_state_value_at(*x, *y, &state_values)
            {
                if best_action.is_none() || action_value > best_action_value {
                    best_action_value = action_value;
                    best_action = Some(*direction);
                }
            }
        }

        // Temporal difference update.
        state_values[ant_y][ant_x] =
            STEP_SIZE * (best_action_value - state_values[ant_y][ant_x]);

        best_action.unwrap()
    }
}

// Returns 3x3 view into the environment with center at given coordinates.
fn get_state_value_at(x: isize, y: isize, state: &State) -> Option<f32> {
    if x < 0 || y < 0 {
        return None;
    }

    state
        .get(y as usize)
        .and_then(|row| row.get(x as usize))
        .map(|x| *x)
}
