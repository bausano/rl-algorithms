#!/usr/bin/env python

import matplotlib.pyplot as plt

plt.plotfile(
    'debug/data.txt',
    delimiter=',',
    cols=(0, 1),
    names=('t', 's'),
    marker='o'
)
plt.show()
