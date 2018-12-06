#!/usr/bin/env python3

with open('input') as stream:
    polymer = tuple(c for c in stream.read() if c.isalpha())

i = 0
while i < len(polymer) - 1:
    if polymer[i] == polymer[i + 1].swapcase():
        polymer = polymer[:i] + polymer[i + 2:]
        if 0 < i:
            i -= 1
    else:
        i += 1

print(len(polymer))
