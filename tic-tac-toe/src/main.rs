mod num_ext;
mod policies;

use num_ext::*;
use rand::prelude::*;
use std::fmt;

/// How many games should the agent play against a random policy to train its
/// value vector.
const TRAINING_GAMES: usize = 1000;

/// Dictates how often an exploration move happens. Exploration move means that
/// given a list of allowed actions, one is selected at random rather than one
/// with the highest reward.
const EXPLORATION_PROBABILITY: f32 = 0.01;

/// Akin to learning rate. Step size is a fraction which will bound the temporal
/// difference in value between state `s` and `s'`.
const STEP_SIZE: f32 = 0.2;

/// Tic-tac-toe is played on 3x3 grid. Since there are 9 fields and each field
/// can be in 3 states, there are 3^9 = 19683 distinct grids.
///
/// Grid is sometimes referred to more generally as state.
#[derive(Clone, Copy, Debug)]
struct Grid {
    pub fields: [Field; 9],
}

/// Each tic-tac-toe grid field can either have an X, an O or be empty.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Field {
    Empty,
    O,
    X,
}

/// The two players.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    X,
    O,
}

/// Value vector holds a ranking (how favorable it is) for each grid. Since
/// there are 19683 distinct states that the grid can be in, this is also the
/// length of the vector. The position of each grid in the vector is given by
/// converting it from radix 3 to radix 10.
type Values = Vec<f32>;

impl Grid {
    /// Creates a new state where each field is set to empty.
    fn new() -> Self {
        Self {
            fields: [Field::Empty; 9],
        }
    }

    /// Puts given player's mark on given field.
    /// ```text
    ///  0 | 1 | 2
    /// ---+---+---
    ///  3 | 4 | 5
    /// ---+---+---
    ///  6 | 7 | 8
    /// ```
    fn put(mut self, field_index: usize, player: Player) -> Self {
        debug_assert!(field_index < 9);
        self.fields[field_index] = player.into();
        self
    }

    /// Prints the grid into console.
    fn print(self) {
        let s = self.fields;
        println!(" {} | {} | {} ", s[0], s[1], s[2]);
        println!("---+---+---");
        println!(" {} | {} | {} ", s[3], s[4], s[5]);
        println!("---+---+---");
        println!(" {} | {} | {} ", s[6], s[7], s[8]);
    }

    /// Returns `None` if the game is not over, returns `Some(true)` if the
    /// provided player has won and `Some(false)` if the provided player hasn't
    /// won.
    ///
    /// Winning is of course determined by having 3 in a row (either vertically,
    /// horizontally or diagonally).
    fn has_won(self, player: Player) -> Option<bool> {
        // If we assign each field a numerical value as follows, we have to
        // check the middle field (4) in both diagonals, the row and the column.
        // Then we have to check the row and the column at the first field (0)
        // and at the last field (8).
        //
        // ```text
        //  0 | 1 | 2
        // ---+---+---
        //  3 | 4 | 5
        // ---+---+---
        //  6 | 7 | 8
        // ```
        let s = self.fields;

        // Does the player have 3 in a row which involves the middle field?
        let connected_via_middle = || {
            (s[4] == s[1] && s[4] == s[7])
                || (s[4] == s[3] && s[4] == s[5])
                || (s[4] == s[0] && s[4] == s[8])
                || (s[4] == s[6] && s[4] == s[2])
        };

        // That can only be the case if the middle field holds player's mark.
        if s[4] == player && connected_via_middle() {
            return Some(true);
        }

        // Does the player have 3 in a row in the first column or row?
        let connected_via_first =
            || (s[0] == s[1] && s[0] == s[2]) || (s[0] == s[3] && s[0] == s[6]);

        // If they have a stone, maybe.
        if s[0] == player && connected_via_first() {
            return Some(true);
        }

        // Does the player have 3 in a row in the last column or row?
        let connected_via_last =
            || (s[8] == s[5] && s[8] == s[2]) || (s[8] == s[7] && s[8] == s[6]);

        // If they have a stone, maybe.
        if s[8] == player && connected_via_last() {
            return Some(true);
        }

        // If there is no empty field where a mark can be put, and the player
        // didn't win, then we return false.
        // Otherwise the game is still on!
        if s.iter().filter(|field| **field == Field::Empty).count() == 0 {
            Some(false)
        } else {
            None
        }
    }

    /// Calculates the position of the grid in the vector of values. It amounts
    /// to treating each of the 9 fields of the grid as a numeral in ternary
    /// system (base 3). Then each field state is arbitrarily assigned a value
    /// 0, 1 or 2.
    fn to_base_10(self) -> usize {
        self.fields
            .iter()
            .zip(0..9)
            .fold(0usize, |ordinal, (field, i)| {
                ordinal + field.as_usize() * 3usize.pow(i)
            })
    }

    /// Converts a decimal number into ternary system (base 3). Then from the
    /// system creates a grid thanks to an arbitrary conversion between 0, 1, 2
    /// and X, O, empty field.
    fn from_base_10(ordinal: usize) -> Self {
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
        Self {
            fields: [
                Field::from_usize(first),
                Field::from_usize(second),
                Field::from_usize(third),
                Field::from_usize(forth),
                Field::from_usize(fifth),
                Field::from_usize(sixth),
                Field::from_usize(seventh),
                Field::from_usize(eighth),
                Field::from_usize(ninth),
            ],
        }
    }
}

impl PartialEq<Player> for Field {
    /// Does the player own the field's mark?
    fn eq(&self, player: &Player) -> bool {
        match self {
            Self::O => *player == Player::O,
            Self::X => *player == Player::X,
            Self::Empty => false,
        }
    }
}

impl From<Player> for Field {
    fn from(player: Player) -> Self {
        match player {
            Player::X => Self::X,
            Player::O => Self::O,
        }
    }
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

impl Field {
    /// Arbitrary conversion of `Field` into `usize`. Must match the
    /// `Field::from_usize` method.
    fn as_usize(self) -> usize {
        match self {
            Self::Empty => 0,
            Self::O => 1,
            Self::X => 2,
        }
    }

    /// Arbitrary conversion of `usize` into `Field`. Must match the
    /// `Field::as_usize` method.
    fn from_usize(u: usize) -> Self {
        match u {
            0 => Self::Empty,
            1 => Self::O,
            2 => Self::X,
            _ => panic!("Field can only be created from 0, 1 or 2."),
        }
    }
}

/// Creates a vector with initial values for each state. Each state which at
/// least one empty field is rated 0.5. Each state without a winner or where
/// an opponent has won is set to 0.0. Each state where given player won is
/// set to 1.0.
fn initial_values(player: Player) -> Values {
    // There are 3^9 possible states in the game of tic tac toe because each of
    // the 9 fields can be in 3 states.
    let possible_states = 3usize.pow(9);
    (0..possible_states)
        .map(|ordinal| {
            Grid::from_base_10(ordinal)
                .has_won(player)
                .map(|has_player_won| if has_player_won { 1.0 } else { 0.0 })
                .unwrap_or(0.5)
        })
        .collect()
}

/// Plays one game against given policy. Updates the value vector during the
/// game. It'd be nicer if the policy was trait and it could be combined in a
/// way where we could have two actors with their own set of state values play
/// against each other. Although this is easy to do, tic tac toe is not
/// interesting enough.
fn play_game(
    rng: &mut ThreadRng,
    values: &mut Values,
    mut policy: impl FnMut(&'_ mut ThreadRng, Grid, &mut Vec<usize>) -> usize,
) -> Grid {
    let mut grid = Grid::new();

    // This is a bit awkward but. It associates each action (vector index) with
    // a field on the grid (the value).
    let mut possible_actions: Vec<_> = (0..9).collect();
    loop {
        // --- Actor's move. Actor plays Xs. ---

        // Rolls a dice whether it should do an exploratory move.
        let should_explore = rng.gen_range(0.0, 1.0);
        if should_explore <= EXPLORATION_PROBABILITY {
            // Pick a random action from the set of possible actions.
            let action = rng.gen_range(0, possible_actions.len());
            // Gets the field which the action represents and marks it as X.
            let field_to_mark = possible_actions.swap_remove(action);
            grid.fields[field_to_mark] = Field::X;
        } else {
            let (action, next_state_value) = possible_actions
                .iter()
                .enumerate()
                .map(|(action, field_to_mark)| {
                    let grid_after_action = grid.put(*field_to_mark, Player::X);
                    let state_value = values[grid_after_action.to_base_10()];
                    (action, state_value)
                })
                .max_by(|(_, value), (_, another_value)| {
                    (*value).partial_ord(*another_value)
                })
                .expect("There must be at least one action to take");

            // Updates the value of the state to be closed to the next state by
            // using the temporal difference.
            let state_ordinal = grid.to_base_10();
            values[state_ordinal] +=
                STEP_SIZE * (next_state_value - values[state_ordinal]);

            // Field that should be marked as X.
            let field_to_mark = possible_actions.swap_remove(action);
            debug_assert_eq!(Field::Empty, grid.fields[field_to_mark]);
            grid.fields[field_to_mark] = Field::X;
        }

        if let Some(true) = grid.has_won(Player::X) {
            // TODO: Would it make sense to set the previous state value to 1?
            break;
        }

        if possible_actions.is_empty() {
            // TODO: Would it make sense to set the previous state value to 0?
            break;
        }

        // --- Policy move. For example human. ---
        let state_ordinal = grid.to_base_10();
        let field_to_mark = policy(rng, grid, &mut possible_actions);
        debug_assert_eq!(Field::Empty, grid.fields[field_to_mark]);
        grid.fields[field_to_mark] = Field::O;
        if let Some(true) = grid.has_won(Player::O) {
            values[state_ordinal] = 0.0;
            break;
        }
    }

    grid
}

fn main() {
    let mut values = initial_values(Player::X);
    let mut rng = thread_rng();

    // Trains the actor against a random policy.
    for _ in 0..TRAINING_GAMES {
        play_game(&mut rng, &mut values, policies::random);
    }

    loop {
        println!("\nNew game!");
        let end_state = play_game(&mut rng, &mut values, policies::human);
        println!();
        end_state.print();
        println!("\nGame finished.");
    }
}
