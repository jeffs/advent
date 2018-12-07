#!/usr/bin/env python3

from functools import partial


def distance(point0, point1):
    x0, y0 = point0
    x1, y1 = point1
    return abs(x1 - x0) + abs(y1 - y0)


def parse_pair(line):
    x, y = line.strip().split(', ')
    return int(x), int(y)


def read_pairs():
    with open('input') as lines:
        return tuple(map(parse_pair, lines))


coordinates = read_pairs()
xs, ys = zip(*coordinates)
left, right = min(xs), max(xs) + 1
top, bottom = min(ys), max(ys) + 1
width, height = right - left, bottom - top

size = 0
for y in range(height):
    for x in range(width):
        point = (left + x, top + y)
        total = sum(map(partial(distance, point), coordinates))
        if total < 10000:
            size += 1

print(size)
