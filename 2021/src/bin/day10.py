from itertools import filterfalse
from sys import argv

CLOSERS = ")]}>"

PAIRS = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">",
}


def load_lines(path):
    with open(path) as lines:
        return tuple(line.strip() for line in lines)


# Part 1


def find_openers(line):
    line = tuple(c for c in line)
    openers = []
    for c in line:
        if c in CLOSERS:
            openers.pop()
        else:
            openers.append(c)
    return openers


def find_corrupt_closer(line):
    line = tuple(c for c in line)
    openers = []
    for c in line:
        if c in CLOSERS:
            if not openers or PAIRS[openers.pop()] != c:
                return c
        else:
            openers.append(c)


def solve1(lines):
    SCORES = {")": 3, "]": 57, "}": 1197, ">": 25137}
    closers = map(find_corrupt_closer, lines)
    return sum(SCORES[c] for c in closers if c)


def test_solve1():
    assert 26397 == solve1(load_lines("tests/day10/sample"))


# Part 2


def complete(openers):
    return reversed([PAIRS[c] for c in openers])


def score_closers(closers):
    score = 0
    for c in closers:
        score = score * 5 + CLOSERS.index(c) + 1
    return score


def test_score_closers():
    assert 288957 == score_closers("}}]])})]")


def score_line(line):
    return score_closers(complete(find_openers(line)))


def solve2(lines):
    lines = filterfalse(find_corrupt_closer, lines)
    scores = sorted(map(score_line, lines))
    return scores[len(scores) // 2]


def test_solve2():
    assert 288957 == solve2(load_lines("tests/day10/sample"))


def main():
    with open("tests/day10/input") as f:
        lines = [line.strip() for line in f]
    print(solve1(lines))
    print(solve2(lines))


if __name__ == "__main__":
    if len(argv) > 1 and argv[1] in ("-t", "--test"):
        test_solve1()
        test_score_closers()
        test_solve2()
    else:
        main()
