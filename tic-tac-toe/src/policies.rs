use super::{Field, Grid};
use rand::prelude::*;
use std::io;
use std::io::prelude::*;

/// Picks a random action from the set of possible moves. Panics if the state
/// has no action for the policy to pick.
pub(super) fn random(
    rng: &mut ThreadRng,
    _: Grid,
    actions: &mut Vec<usize>,
) -> usize {
    debug_assert_ne!(0, actions.len());
    let i = rng.gen_range(0, actions.len());
    actions.swap_remove(i)
}

/// Asks for human input.
pub(super) fn human(
    _rng: &mut ThreadRng,
    grid: Grid,
    actions: &mut Vec<usize>,
) -> usize {
    let field = |i: usize| match grid.fields[i] {
        Field::Empty => {
            actions.iter().position(|a| *a == i).unwrap().to_string()
        }
        field => field.to_string(),
    };

    println!(" {} | {} | {} ", field(0), field(1), field(2));
    println!("---+---+---");
    println!(" {} | {} | {} ", field(3), field(4), field(5));
    println!("---+---+---");
    println!(" {} | {} | {} ", field(6), field(7), field(8));

    let stdin = io::stdin();
    let handle = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .expect("Expected a string stdin input");
    let i = handle
        .parse::<usize>()
        .expect("Cannot parse given string to a number");
    assert!(
        i > actions.len(),
        "You must provide a number less than {}.",
        actions.len()
    );
    actions.swap_remove(i)
}
