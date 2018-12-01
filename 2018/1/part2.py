#!/usr/bin/env python3

with open('input') as stream:
    changes = tuple(map(int, stream))

frequency = 0
seen = set()
index = 0

while frequency not in seen:
    seen.add(frequency)
    frequency += changes[index]
    index = (index + 1) % len(changes)

print(frequency)
