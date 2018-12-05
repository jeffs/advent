#!/usr/bin/env python3

def parse_ints(line):
    digit_or_space = lambda c: c if c in '0123456789' else ' '
    words = ''.join(map(digit_or_space, line)).split()
    return tuple(map(int, words))

def parse_rectangle(line):
    _, left, top, width, height = parse_ints(line)
    return left, top, width, height

with open('input') as stream:
    rectangles = tuple(map(parse_rectangle, stream))

counts = { }    # maps (column, row) pairs to rectange counts
for left, top, width, height in rectangles:
    for column in range(left, left + width):
        for row in range(top, top + height):
            counts[(column, row)] = counts.get((column, row), 0) + 1

overlap_count = sum(1 for v in counts.values() if v > 1)
print(overlap_count)
