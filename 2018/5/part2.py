#!/usr/bin/env python3

def react(polymer):
    i = 0
    while i < len(polymer) - 1:
        if polymer[i] == polymer[i + 1].swapcase():
            polymer = polymer[:i] + polymer[i + 2:]
            if 0 < i:
                i -= 1
        else:
            i += 1
    return polymer


def remove(polymer, char):
    import sys
    print('removing', char, file=sys.stderr)
    chars = (char, char.swapcase())
    return tuple(c for c in polymer if c not in chars)


with open('input') as stream:
    polymer = tuple(c for c in stream.read() if c.isalpha())

chars = set(c.upper() for c in polymer)
n = min(len(react(remove(polymer, c))) for c in chars)
print(n)
