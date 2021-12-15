from day14 import part1, part2, puzzle, sample
from day14.part2 import update, count_pairs, dumps

from sys import exit, stderr


def assert_eq(want, got):
    if want != got:
        raise AssertionError(f"want {want}; got {got}")


def test_iterations():
    """Checks a few functions from part 2 against part 1."""
    chain = sample.chain
    counts = count_pairs(sample.chain)
    for i in range(1, 5):
        chain = part1.insert(chain, sample.rules)
        counts = update(counts, sample.rules)
        want = dumps(count_pairs(chain))
        got = dumps(counts)
        assert_eq(f"after step {i}: {want}", f"after step {i}: {got}")


def print_table():
    """Prints output like the Part 1 sample."""
    chain = sample.chain
    print("Template:    ", chain)
    for i in range(1, 5):
        chain = part1.insert(chain, sample.rules)
        print(f"After step {i}:", chain)
    print()


if __name__ == "__main__":
    test_iterations()
    assert_eq(1588, part1.solve(sample.chain, sample.rules, 10))
    assert_eq(1588, part2.solve(sample.chain, sample.rules, 10))

    print_table()
    print(part1.solve(puzzle.chain, puzzle.rules, 10))
    print(part2.solve(puzzle.chain, puzzle.rules, 40))
