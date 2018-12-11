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

def test_cell_power():
    assert cell_power(3, 5, 8) == 4
    assert cell_power(122,  79, 57) == -5
    assert cell_power(217, 196, 39) ==  0
    assert cell_power(101, 153, 71) ==  4

test_cell_power()


class RowPowerCache:

    def __init__(self, y, serial):
        self.sums = [0] * EXTENT
        self.sums[0] = cell_power(1, y, serial)
        for x in range(2, EXTENT + 1):
            power = cell_power(x, y, serial)
            self.sums[x - 1] = self.sums[x - 2] + power

    def get(self, left, size):
        assert 1 <= left <= EXTENT
        assert 1 <= size <= EXTENT
        assert 1 <= left + size - 1 <= EXTENT
        passed = self.sums[left - 2] if 1 < left else 0
        return self.sums[left + size - 2] - passed

def test_row_power_cache():
    SERIAL = 5235
    cache = RowPowerCache(1, SERIAL)
    assert cache.get(1, 1) == cell_power(1, 1, SERIAL)
    assert cache.get(2, 3) == ( cell_power(2, 1, SERIAL)
                              + cell_power(3, 1, SERIAL)
                              + cell_power(4, 1, SERIAL) )

test_row_power_cache()


class PowerCache:

    max_size_seen = 0

    def __init__(self, serial):
        self.row_caches = tuple(
                RowPowerCache(y, serial)
                for y in range(1, EXTENT + 1))

    def get(self, left, top, size):
        if size != PowerCache.max_size_seen:
            print('\rPowerCache.get: size={}    '.format(size), end='')
            PowerCache.max_size_seen = size
        assert 1 <= top <= EXTENT
        assert 1 <= size <= EXTENT
        assert 1 <= top + size - 1 <= EXTENT
        return sum(self.row_caches[y - 1].get(left, size)
                for y in range(top, top + size))

def test_power_cache():
    assert PowerCache(5235).get(33, 54, 3) == 28
    assert PowerCache(18).get(90, 269, 16) == 113
    assert PowerCache(42).get(232, 251, 12) == 119

test_power_cache()


def find_square(serial):
    cache = PowerCache(serial)
    def get_power(square):
        return cache.get(*square)
    return max(
            ((x, y, size)
                for size in range(1, EXTENT)
                for x in range(1, EXTENT - size + 2)
                for y in range(1, EXTENT - size + 2)),
            key=get_power)

def test_find_square():
    assert find_square(18) == (90, 269, 16)
    assert find_square(42) == (232, 251, 12)

# test_find_square()    # Takes a long time.


def main():
    with open('input') as line:
        serial = int(line.read())
    x, y, size = find_square(serial)
    print()
    print('{},{},{}'.format(x, y, size))

if __name__ == '__main__':
    main()

