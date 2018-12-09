#!/usr/bin/env python3
#
# 476 players; last marble is worth 71431 points

DEBUG = False

import sys

def find_high_score(num_players, num_marbles):
    assert num_players >= 1
    assert num_marbles >= 0
    scores = [0] * num_players
    circle = [0]
    index = 0                           # index of the "current marble"
    if DEBUG:
        print('[-] (0)', file=sys.stderr)
    for marble in range(1, num_marbles + 1):

        player = (marble - 1) % num_players

        if marble % 23 == 0:
            index = (index + (len(circle) - 1) * 7) % len(circle)
            scores[player] += marble + circle[index]
            circle.pop(index)
        else:
            index = (index + 2) % len(circle)
            circle.insert(index, marble)

        if DEBUG:
            print('[{}]'.format(player + 1),
                    ''.join(map(
                        lambda x: ('({})' if x == circle[index] else ' {} ').format(x),
                        [circle[-1]] + circle[:-1])),
                    file=sys.stderr)

    return max(scores)


if __name__ == '__main__':
    assert find_high_score( 9, 25) == 32

    assert find_high_score(10, 1618) == 8317
    assert find_high_score(13, 7999) == 146373
    assert find_high_score(17, 1104) == 2764
    assert find_high_score(21, 6111) == 54718
    assert find_high_score(30, 5807) == 37305

    print(find_high_score(476, 71_431))
