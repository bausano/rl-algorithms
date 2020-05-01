# Ants
"Ants" are an extension of an example given by David Silver in his
[video lecture][david-silver-videolecture-3]. Two ant dynasties are fighting
over randomly distributed resources in an environment. Ants get positive reward
signal when they kill non related ants, pick up food, and deliver food to nest.
They get negative reward for being killed and per move. Ant is characterized
its `x` and `y` coordinate in a grid and by its direction. The direction
dictates which three adjacent cells to `x:y` the ant can "sense".

In the following scenario, the ant's direction is "left" as it can "sense" the
cells to the left.
```text
 x-1  x  x+1
+---+---+---+
| o |   |   |  y-1
+---+---+---+
| o | A |   |  y
+---+---+---+
| o |   |   |  y+1
+---+---+---+
```

Each dynasty has got N ants. When ants are killed they cost food to re-spawn.
Therefore a dynasty which is proficient at gathering food will, besides reward
signal, be benefiting from more ants. Each ant has a limited life span, which is
why each move ants get negative reward signal in default case.

All dynasty ants share the same policy and value function.

Each ant leaves for M moves a trail. Related ants are able to distinguish
between `n in N`. For enemy trail, ant only "senses" whether it exists or not.
I am hoping this could lead to specializations of ants (i.e. ant #3, #25 and
#40 are explorers while other are exploiters).

<!-- Invisible List of References -->
[david-silver-videolecture-3]: https://youtu.be/Nd1-UUMVfz4?t=1771