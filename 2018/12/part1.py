#!/usr/bin/env python3

NUM_GEN = 20

def main(file_name, debug=False):
    with open(file_name) as stream:
        lines = stream.readlines()

    lines = tuple(line.strip() for line in lines)
    lines = tuple(line.split() for line in lines if line)
    goods = set(line[0] for line in lines[1:] if line[2] == '#')
    state = lines[0][2]

    def get(index):
        return state[index] if 0 <= index < len(state) else '.'

    if debug:
        print(' 0:', state)

    for gen in range(20):
        pots = []
        for i in range(-2, len(state) + 2):
            key = ''.join(get(j) for j in range(i - 2, i + 3))
            pots.append('#' if key in goods else '.')
        state = ''.join(pots)
        if debug:
            print('{:2d}: {}'.format(gen + 1, state))

    def adjust(index):
        return index - NUM_GEN * 2

    return sum(adjust(i) for i in range(len(state)) if state[i] == '#')


assert main('short') == 325

print(main('input'))
