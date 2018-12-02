#!/usr/bin/env python3

from collections import Counter

with open('input') as stream:
    counters = tuple(map(Counter, stream))

n2 = sum(1 for counter in counters if 2 in counter.values())
n3 = sum(1 for counter in counters if 3 in counter.values())

print(n2 * n3)
