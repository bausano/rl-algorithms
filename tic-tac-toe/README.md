# Tic-Tac-Toe
> To illustrate the general idea of reinforcement learning and contrast it with other approaches, we next consider a single example in more detail.Consider the familiar child’s game of tic-tac-toe.  Two players take turns playing on a three-by-three board.  One player plays Xs and the other Os until one player wins by placing three marks in a row,  horizontally,  vertically,  or diagonally, as the X player has in this game:
```
 x | o | o
---+---+---
 o | x | x
---+---+---
   |   | x
```
> If the board fills up with neither player getting three in a row,  the game isa draw.  Because a skilled player can play so as never to lose, let us assumethat we are playing against an imperfect player, one whose play is sometimes incorrect and allows us to win.  For the moment, in fact, let us consider drawsand losses to be equally bad for us.  How might we construct a player that willfind the imperfections in its opponent’s play and learn to maximize its chancesof winning?
    \
    \
    _Sutton Bartol, Reinforcement Learning: Introduction, 2nd edition, p. 11_

This implementation has one value for each one of the 3^9 possible states. It uses temporal difference to update state values.

You can run it with `cargo run --release` and optionally you can provide how many training games should it play against a random policy before playing against a user (e.g. `cargo run --release 5000`).

## Exercises
> Many tic-tac-toe positions appear different but are really the same because of symmetries. How might we amend the reinforcement learning algorithm described above to take advantage of this? In what ways would this improve it? Now think again. Suppose the opponent did not take advantage of symmetries. In that case, should we? Is it true, then, that symmetrically equivalent positions should necessarily have the same value?

While symmetries would reduce computational complexity, if the opponent does not take advantages, in the example of tic-tac-toe we might fail to pick up some low hanging fruit. An option is to update states of symmetries by only a fraction of what we update the actual played state by. This way we learn from symmetrical positions and leverage symmetries, but at the same time we're aware that we can take advantage of our opponent's ignorance of symmetry.

> Greedy Play Suppose the reinforcement learning player was greedy, that is, it always played the move that brought it to the position that it rated the best. Would it learn to play better, or worse, than a non greedy player? What problems might occur?

We are stuck in a local minima. The actor wouldn't innovate. Solution that is ok to be played against one environment might fail with slight changes in the environment.
