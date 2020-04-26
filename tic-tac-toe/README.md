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
