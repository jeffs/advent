#!/usr/bin/env python3

from functools import partial


EXTENT = 300


def cell_power(x, y, serial):
    rack_id = x + 10
    power = rack_id * y
    power += serial
    power *= rack_id
    digit = power // 100 % 10
    return digit - 5

assert cell_power(3, 5, 8) == 4
assert cell_power(122,  79, 57) == -5
assert cell_power(217, 196, 39) ==  0
assert cell_power(101, 153, 71) ==  4


def square_power(top_left, serial):
    left, top = top_left
    return sum(
        cell_power(x, y, serial)
        for x in range(left, left + 3)
        for y in range(top, top + 3))


def find_square(serial):
    return max(
            ((x, y)
                for x in range(1, EXTENT - 1)
                for y in range(1, EXTENT - 1)),
            key=partial(square_power, serial=serial))


def main():
    with open('input') as line:
        serial = int(line.read())
    x, y = find_square(serial)
    print(square_power((x, y), serial))
    print('{},{}'.format(x, y))


if __name__ == '__main__':
    main()

