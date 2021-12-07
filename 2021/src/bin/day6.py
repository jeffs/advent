def load_fish(path):
    with open(path) as istream:
        return tuple(map(int, istream.read().split(",")))


def next_part1(fish):
    fish = [i - 1 for i in fish]
    noobs = fish.count(-1)
    return [6 if i < 0 else i for i in fish] + [8] * noobs


def solve_part1(fish):
    for _ in range(80):
        fish = next_part1(fish)
    return len(fish)


def test_part1():
    assert 5934 == solve_part1(load_fish("tests/day6/sample"))


def next_part2(counts):
    counts = { (k - 1): v for (k, v) in counts.items() }
    if -1 in counts:
        noobs = counts[8] = counts[-1]
        counts[6] = counts.get(6, 0) + noobs
        del counts[-1]
    return counts


def solve_part2(fish):
    counts = {}
    for f in fish:
        counts[f] = counts.get(f, 0) + 1
    for _ in range(256):
        counts = next_part2(counts)
    return sum(counts.values())


def test_part2():
    assert 26984457539 == solve_part2(load_fish("tests/day6/sample"))


def main():
    test_part1()
    test_part2()
    fish = load_fish("tests/day6/input")
    print(solve_part1(fish))
    print(solve_part2(fish))


if __name__ == "__main__":
    main()
