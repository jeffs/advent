#!/usr/bin/env python3

def parse_ints(line):
    digit_or_space = lambda c: c if c in '0123456789' else ' '
    words = ''.join(map(digit_or_space, line)).split()
    return tuple(map(int, words))

with open('input') as stream:
    rectangles = tuple(map(parse_ints, stream))

counts = { }    # maps (column, row) pairs to rectange counts
for _id, left, top, width, height in rectangles:
    for column in range(left, left + width):
        for row in range(top, top + height):
            counts[(column, row)] = counts.get((column, row), 0) + 1

def is_overlapped(rectangle):
    _id, left, top, width, height = rectangle
    for column in range(left, left + width):
        for row in range(top, top + height):
            if counts[(column, row)] > 1:
                return True
    return False

for rectangle in rectangles:
    if not is_overlapped(rectangle):
        print(rectangle[0])
