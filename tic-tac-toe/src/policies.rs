use super::{draw, Field, State};
use rand::prelude::*;
use std::io;
use std::io::prelude::*;

/// Picks a random action from the set of possible moves. Panics if the state
/// has no action for the policy to pick.
pub(super) fn random(
    rng: &mut ThreadRng,
    _state: State,
    actions: &mut Vec<usize>,
) -> usize {
    debug_assert_ne!(0, actions.len());
    let i = rng.gen_range(0, actions.len());
    actions.swap_remove(i)
}

/// Always picks the last action in the array.
pub(super) fn last_action(
    _rng: &mut ThreadRng,
    _state: State,
    actions: &mut Vec<usize>,
) -> usize {
    actions.pop().expect("There has to be at least one action")
}

/// Always picks the first action in the array.
pub(super) fn first_action(
    _rng: &mut ThreadRng,
    _state: State,
    actions: &mut Vec<usize>,
) -> usize {
    actions.swap_remove(0)
}

/// Asks for human input.
pub(super) fn human(
    _rng: &mut ThreadRng,
    state: State,
    actions: &mut Vec<usize>,
) -> usize {
    let draw_field = |i: usize| match state[i] {
        Field::Empty => {
            actions.iter().position(|a| *a == i).unwrap().to_string()
        }
        field => field.to_string(),
    };

    println!(
        " {} | {} | {} ",
        draw_field(0),
        draw_field(1),
        draw_field(2)
    );
    println!("---+---+---");
    println!(
        " {} | {} | {} ",
        draw_field(3),
        draw_field(4),
        draw_field(5)
    );
    println!("---+---+---");
    println!(
        " {} | {} | {} ",
        draw_field(6),
        draw_field(7),
        draw_field(8)
    );

    let stdin = io::stdin();
    let handle = stdin.lock().lines().next().unwrap().unwrap();
    let i = handle.parse::<usize>().expect("Cannot parse to number");
    actions.swap_remove(i)
}
