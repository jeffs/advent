#!/usr/bin/env python3


from collections import namedtuple
from inspect import isgenerator
import sys


Position = namedtuple('Position', ['x', 'y'])
Velocity = namedtuple('Velocity', ['dx', 'dy'])
Light = namedtuple('Light', ['position', 'velocity'])


class Grid:

    def __init__(self, lights):
        self.lights = normalize_lights(lights)
        positions = get_positions(self.lights)
        self.width = max(x for x, y in positions) + 1
        self.height = max(y for x, y in positions) + 1

    def next(self):
        return Grid(
                translate_light(light, light.velocity)
                for light in self.lights)

    def dimensions(self):
        return self.width, self.height

    def print(self, width, height):
        positions = get_positions(self.lights)
        print('\n' + '\n'.join(
            ''.join(
                '#' if Position(x, y) in positions else '.'
                for x in range(width))
            for y in range(height)))


def get_positions(lights):
    return set(light.position for light in lights)


def normalize_lights(lights):
    """
    Translate `lights` so the left-most x and top-most y coordinates are zero.
    This is purely an adjustment of the frame of reference:  Lights are not
    moved relative to each other, nor are their velocities altered.
    """
    if isgenerator(lights):
        lights = tuple(lights)
    positions = get_positions(lights)
    adjustment = Velocity(
            -min(position.x for position in positions),
            -min(position.y for position in positions))
    return translate_lights(lights, adjustment)


def parse_light(line):
    spaced = ''.join(c if c in '1234567890-' else ' ' for c in line)
    x, y, dx, dy = map(int, spaced.split())
    return Light(Position(x, y), Velocity(dx, dy))


def read_file(path):
    with open(path) as lines:
        return tuple(map(parse_light, lines))


def translate_light(light, velocity):
    """Add the dx and dy of `velocity` to the position of `light`."""
    position = Position(
            light.position.x + velocity.dx,
            light.position.y + velocity.dy)
    return Light(position, light.velocity)


def translate_lights(lights, velocity):
    return tuple(translate_light(light, velocity) for light in lights)


def main():
    DISPLAY_WIDTH = 80
    DISPLAY_HEIGHT = 24
    lights = read_file('input')
    prev = Grid(lights)
    grid = prev.next()
    time = 0
    while grid.dimensions() < prev.dimensions():
        time += 1
        print('\rtime =', time, end='')
        prev, grid = grid, grid.next()
    prev.print(DISPLAY_WIDTH, DISPLAY_HEIGHT)

if __name__ == '__main__':
    main()
