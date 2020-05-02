//! Includes the RL inspired logic.
use crate::environment::{Cell, Direction, DynastyId};
use rand::prelude::*;
use std::collections::HashMap;

type State = (u8, u8, u8);

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

    pub fn pick_action(&mut self, cells: &[Vec<Cell>]) -> Direction {
        Direction::rand(&mut self.rng)
    }

    pub fn reward(&mut self, signal: f32) {
        unimplemented!()
    }
}
