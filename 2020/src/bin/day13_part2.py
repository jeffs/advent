#!/usr/bin/env python3
#
# This problem really threw me for a loop, so I explored the space a little in
# Python.  It's great to use a dynamic language with a REPL to figure stuff out
# before investing in a more robust and maintainable implementation.  Python is
# especially nice for working with large integers.
#
# We're looking for a timestamp (in minutes) such that each bus arrives at a
# particular delay after that timestamp.  Each bus arrives regularly at
# multiples of some number of minutes, called the "bus ID," all starting at
# time zero.
#
# If we were to divide the target timestamp (once we found it) by each bus ID
# and take the remainder, we would have the complement of each delay: i.e., the
# number of minutes by which our timestamp missed the previous arrival of each
# bus.  It turns out that if we compute the remainders first (by subtracting
# each bus's delay from its period), we can then work backwards to find the
# timestamp using the Chinese Remainder Theorem.
#
# Aside:  All the online materials I found about modulus arithmetic used
# conventions I found unfamiliar.  For example, instead of `f(x) % d = r` they
# would say `f(x) ≡ r (mod d)`, and they use 1-based indexes even when
# discussing remainders.  The only explanation I found accessible was this
# video from Randell Heyman: https://www.youtube.com/watch?v=ru7mWZJlRQg .
# Note, however, they he does not show how to find the _lowest_ solution, which
# is what we actually need for Advent.
#
# The theorem requires that all input divisors (in our case, the bus IDs) are
# coprime (i.e., don't share any prime factors).  This is true for all of the
# samples in the original problem statement, as well as for my input.  In fact,
# all the numbers "happen" to be prime, which is a pretty good hint we're going
# the right way.  Thanks, @topaz!
#
# It would be nice if we could consider the buses one at a time, then combine
# the results.  For example, if there were only one bus ID b having remainder
# r, we could find a timestamp t such that t % b = r.  (In that case, t = b + r
# would work.)  But how do we find a value t that works for all buses, instead
# of only one particular bus?
#
# If we were to find a suitable t, then add 0 to it, we would still have t.  In
# fact, we could add any multiple of b to t and still have a valid solution,
# because it would not change the value of t % b; in other words, (t + k * b) %
# b == t % b for any integer k, so if t is a solution, then so is t * k * b.
# (Remember that t being a "solution" for a given bus b means that b will
# arrive at t plus some delay, and our remainder r is the bus ID b minus that
# delay).
#
# So, we can define our target t as a series (sum) of per-bus solutions, as
# long as each per-bus solution is a multiple of all the other bus IDs.  This
# is the key insight that makes the Chinese Remainder Theorem work, and which I
# personally found a little hard to wrap my head around:  We're going to solve
# for each bus, but constrain each per-bus solution so that it can be added to
# all of the other per-bus solutions without invalidating them.  The sum of all
# these per-bus solutions will then, by definition, work for all of the buses,
# and be our final answer.
#
# The fact that our per-bus solutions have to be multiples of all the other bus
# IDs means the anwer will be a big number.  As the problem states:
#
#   With so many bus IDs in your list, surely the actual earliest timestamp
#   will be larger than 100000000000000!
#
# The size of that number (1e14) means we can't find the solution in time to
# earn a star on Advent by simply looping through all possible timestamps.
# Even if we check 1e9 timestamps per second (one every few clock cycles on a
# current generation CPU), it will take us over 24 hours to brute force the
# answer, which would be too long to complete this problem on the day it drops.

from functools import reduce
from operator import mul


def is_prime(n):
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    i = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i += 2
    return True


def find_multiplier(multiplicand, bus_id, remainder):
    """
    Returns an integer N such that multiplicand * N % bus_id == remainder.
    """
    for n in range(bus_id):
        if multiplicand * n % bus_id == remainder:
            return n
    raise Exception(
        "no solution: can't find multiplier for bus {}".format(bus_id))


def find_timestamp(bus_ids, remainders):
    product = reduce(mul, bus_ids)
    sum = 0
    for b, r in zip(bus_ids, remainders):
        multiplicand = product // b
        multiplier = find_multiplier(multiplicand, b, r)
        term = multiplicand * multiplier
        sum += term
    return int(sum) % product


def parse_line(line):
    entries = line.strip().split(',')
    delays = [i for i in range(len(entries)) if entries[i].isdigit()]
    bus_ids = [int(entries[i]) for i in delays]
    return delays, bus_ids


def load_input(path):
    with open(path) as lines:
        next(lines)
        return parse_line(next(lines))


def solve_part2(delays, bus_ids):
    assert all(map(is_prime, bus_ids))
    remainders = [(n - i) % n for i, n in zip(delays, bus_ids)]
    return find_timestamp(bus_ids, remainders)


def test_is_prime():
    assert (2, 3, 5, 7, 11, 13, 17, 19) == tuple(filter(is_prime, range(20)))


def test_find_timestamp():
    bus_ids = [3, 4, 5]
    remainders = [2, 2, 1]
    assert 26 == find_timestamp(bus_ids, remainders)


def test_solve_part2():
    assert 1068781 == solve_part2(*load_input('tests/day13/sample1'))
    for line, want in (
            ('17,x,13,19', 3417),
            ('67,7,59,61', 754018),
            ('67,x,7,59,61', 779210),
            ('67,7,x,59,61', 1261476),
            ('1789,37,47,1889', 1202161486)):
        assert want == solve_part2(*parse_line(line))


def test():
    test_is_prime()
    test_find_timestamp()
    test_solve_part2()


if __name__ == '__main__':
    test()
    print(solve_part2(*load_input('tests/day13/input')))
