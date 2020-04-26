use rand::prelude::*;

/// Picks a random action from the set of possible moves. Panics if the state
/// has no action for the policy to pick.
pub(super) fn random(rng: &mut ThreadRng, actions: &mut Vec<usize>) -> usize {
    debug_assert_ne!(0, actions.len());
    let i = rng.gen_range(0, actions.len());
    actions.swap_remove(i);
    actions[i]
}
