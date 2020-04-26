mod num_ext;
mod policies;

use num_ext::*;
use rand::prelude::*;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Field {
    X,
    O,
    Empty,
}

impl Default for Field {
    fn default() -> Self {
        Self::Empty
    }
}

impl Field {
    fn as_usize(self) -> usize {
        match self {
            Self::Empty => 0,
            Self::O => 1,
            Self::X => 2,
        }
    }

    fn from_usize(u: usize) -> Self {
        match u {
            0 => Self::Empty,
            1 => Self::O,
            2 => Self::X,
            _ => panic!("Field can only be created from 0, 1 or 2."),
        }
    }
}

type State = [Field; 9];

type Values = Vec<f32>;

fn empty_state() -> State {
    [Field::default(); 9]
}

fn ordinal(state: State) -> usize {
    state.iter().zip(0..9).fold(0usize, |ordinal, (field, i)| {
        ordinal + field.as_usize() * 3usize.pow(i)
    })
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
            Self::Empty => write!(f, " "),
        }
    }
}

fn draw(state: State) {
    println!("Game state {}: ", ordinal(state));
    println!(" {} | {} | {} ", state[0], state[1], state[2]);
    println!("---+---+---");
    println!(" {} | {} | {} ", state[3], state[4], state[5]);
    println!("---+---+---");
    println!(" {} | {} | {} ", state[6], state[7], state[8]);
}

/// Returns `None` if the game is not over, returns Some(true) if the "X" player
/// won, returns Some(false) if the "O" player won.
///  0 | 1 | 2
/// ---+---+---
///  3 | 4 | 5
/// ---+---+---
///  6 | 7 | 8
fn has_won(state: State, player: Field) -> Option<bool> {
    let connected_via_middle = || {
        (state[4] == state[1] && state[4] == state[7])
            || (state[4] == state[3] && state[4] == state[5])
            || (state[4] == state[0] && state[4] == state[8])
            || (state[4] == state[6] && state[4] == state[2])
    };

    if state[4] != Field::Empty && connected_via_middle() {
        return Some(state[4] == player);
    }

    let connected_via_first = || {
        (state[0] == state[1] && state[0] == state[2])
            || (state[0] == state[3] && state[0] == state[6])
    };

    if state[0] != Field::Empty && connected_via_first() {
        return Some(state[0] == player);
    }

    let connected_via_last = || {
        (state[8] == state[5] && state[8] == state[2])
            || (state[8] == state[7] && state[8] == state[6])
    };

    if state[8] != Field::Empty && connected_via_last() {
        return Some(state[8] == player);
    }

    // We rate draw the same as loss.
    if state.iter().filter(|field| **field == Field::Empty).count() == 0 {
        Some(false)
    } else {
        None
    }
}

fn state(ordinal: usize) -> State {
    let ninth = ordinal / 3usize.pow(8);
    debug_assert!(ninth <= 2);
    let ordinal = ordinal - ninth * 3usize.pow(8);
    let eighth = ordinal / 3usize.pow(7);
    debug_assert!(eighth <= 2);
    let ordinal = ordinal - eighth * 3usize.pow(7);
    let seventh = ordinal / 3usize.pow(6);
    debug_assert!(seventh <= 2);
    let ordinal = ordinal - seventh * 3usize.pow(6);
    let sixth = ordinal / 3usize.pow(5);
    debug_assert!(sixth <= 2);
    let ordinal = ordinal - sixth * 3usize.pow(5);
    let fifth = ordinal / 3usize.pow(4);
    debug_assert!(fifth <= 2);
    let ordinal = ordinal - fifth * 3usize.pow(4);
    let forth = ordinal / 3usize.pow(3);
    debug_assert!(forth <= 2);
    let ordinal = ordinal - forth * 3usize.pow(3);
    let third = ordinal / 3usize.pow(2);
    debug_assert!(third <= 2);
    let ordinal = ordinal - third * 3usize.pow(2);
    let second = ordinal / 3usize;
    debug_assert!(second <= 2);
    let ordinal = ordinal - second * 3usize;
    let first = ordinal;
    debug_assert!(first <= 2);
    [
        Field::from_usize(first),
        Field::from_usize(second),
        Field::from_usize(third),
        Field::from_usize(forth),
        Field::from_usize(fifth),
        Field::from_usize(sixth),
        Field::from_usize(seventh),
        Field::from_usize(eighth),
        Field::from_usize(ninth),
    ]
}

fn initial_values() -> Values {
    // There are 3^9 possible states in the game of tic tac toe because each of
    // the 9 fields can be in 3 states.
    let possible_states = 3usize.pow(9);
    (0..possible_states)
        .map(|ordinal| {
            has_won(state(ordinal), Field::X)
                .map(|has_x_won| if has_x_won { 1.0 } else { 0.0 })
                .unwrap_or(0.5)
        })
        .collect()
}

// fn get_possible_actions(state: State) -> Vec<usize> {
//     state
//         .iter()
//         .enumerate()
//         .filter(|(_, field)| **field == Field::Empty)
//         .map(|(i, _)| i)
//         .collect()
// }

fn main() {
    const GAMES: usize = 1000;
    const EXPLORATION: f32 = 0.01;
    const STEP_SIZE: f32 = 0.1;

    let mut values = initial_values();
    let mut rng = thread_rng();
    let mut draws = 0;
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut explorations = 0;
    let mut exploitations = 0;

    for _ in 0..GAMES {
        let mut state = empty_state();
        let mut possible_actions: Vec<_> = (0..9).collect();
        loop {
            // Move of the computer.
            let should_explore = rng.gen_range(0.0, 1.0);
            if should_explore <= EXPLORATION {
                explorations += 1;
                let i = rng.gen_range(0, possible_actions.len());
                state[possible_actions[i]] = Field::X;
                possible_actions.swap_remove(i);
            } else {
                let (i, action, next_state_value) = possible_actions
                    .iter()
                    .enumerate()
                    .map(|(pos, action)| {
                        let mut state_after_action = state;
                        state_after_action[*action] = Field::X;
                        let state_value = values[ordinal(state_after_action)];
                        (pos, *action, state_value)
                    })
                    .max_by(|(_, _, value), (_, _, another_value)| {
                        (*value).partial_ord(*another_value)
                    })
                    .expect("There must be at least one action to take");
                exploitations += 1;
                let state_ordinal = ordinal(state);
                values[state_ordinal] +=
                    STEP_SIZE * (next_state_value - values[state_ordinal]);
                debug_assert_eq!(Field::Empty, state[action]);
                state[action] = Field::X;
                possible_actions.swap_remove(i);
            }

            if let Some(true) = has_won(state, Field::X) {
                draw(state);
                x_wins += 1;
                break;
            }

            if possible_actions.is_empty() {
                draws += 1;
                break;
            }

            // Policy move.
            let action = policies::random(&mut rng, &mut possible_actions);
            debug_assert_eq!(Field::Empty, state[action]);
            state[action] = Field::O;
            if let Some(true) = has_won(state, Field::O) {
                o_wins += 1;
                break;
            }
        }
    }

    println!(
        "During training X won {} times, \
        O won {} times and there were {} draws. \
        X win probability is {}. There were {} exploration and \
        {} exploitation moves.",
        x_wins,
        o_wins,
        draws,
        x_wins as f32 / (x_wins as f32 + o_wins as f32 + draws as f32),
        explorations,
        exploitations
    );

    // TODO: Plays one game against me.
}
