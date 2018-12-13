#!/usr/bin/env python3

def load_rules(file_name):
    """
    Returns the initial sequence of live/dead plants (as a string), and the set
    of patterns resulting in live plants in each subsequent generation,
    recorded in the file having the specified `file_name`.
    """
    with open(file_name) as stream:
        lines = stream.readlines()
    lines = tuple(line.strip() for line in lines)
    lines = tuple(line.split() for line in lines if line)
    state = lines[0][2]
    goods = set(line[0] for line in lines[1:] if line[2] == '#')
    return state, goods


def brute(state, goods, num_gen):

    def get(index):
        return state[index] if 0 <= index < len(state) else '.'

    for gen in range(num_gen):
        pots = []
        for i in range(-2, len(state) + 2):
            key = ''.join(get(j) for j in range(i - 2, i + 3))
            pots.append('#' if key in goods else '.')
        state = ''.join(pots)

    def adjust(index):
        return index - num_gen * 2

    return sum(adjust(i) for i in range(len(state)) if state[i] == '#')


def find_inflection(state, goods):
    n0, n1, n2 = 0, 1, 2
    b0, b1, b2 = (
            brute(state, goods, n0),
            brute(state, goods, n1),
            brute(state, goods, n2))
    d0, d1 = b1 - b0, b2 - b1

    while d0 != d1:
        n0, n1, n2 = n1, n2, n2 + 1
        b0, b1, b2 = b1, b2, brute(state, goods, n2)
        d0, d1 = b1 - b0, b2 - b1

    return n0, b0, d0


def main(file_name, num_gen):
    state, goods = load_rules(file_name)
    base_gen, base_val, diff = find_inflection(state, goods)
    print(base_gen, base_val, diff)
    print(base_val + (num_gen - base_gen) * diff)


if __name__ == '__main__':
    main('input', 50000000000)
