#!/usr/bin/env python3

from functools import partial


def closest_coordinate(point, coordinates):
    min_dist = min(map(partial(distance, point), coordinates))
    closest = None
    for coord in coordinates:
        if distance(point, coord) == min_dist:
            if closest:
                return None
            closest = coord
    return closest


def distance(point0, point1):
    x0, y0 = point0
    x1, y1 = point1
    return abs(x1 - x0) + abs(y1 - y0)


def make_empty_row(size):
    return [None] * size


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

# Each spot in the grid will be assigned the coordinate to which it is closest.
rows = [make_empty_row(width) for _ in range(height)]

for y in range(height):
    row = rows[y]
    for x in range(width):
        point = (left + x, top + y)
        row[x] = closest_coordinate(point, coordinates)

# Coordinates whose zones extend to the top, left, right, or bottom edge are
# disqualified, because their areas are implicitly infinite.
infinites = set(rows[0]) \
          | set(row[0] for row in rows) \
          | set(row[-1] for row in rows) \
          | set(rows[-1])

counts = { }
for y in range(height):
    row = rows[y]
    for x in range(width):
        point = (left + x, top + y)
        coord = row[x]
        if coord not in infinites:
            counts[coord] = counts.get(coord, 0) + 1


max_value = max(counts.values())
print(max_value)
