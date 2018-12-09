#!/usr/bin/env python3

from collections import namedtuple
from time import time

DEBUG = False

class Node:

    def __init__(self, value, prev=None, next=None):
        self.value = value
        self.prev = prev or self
        self.next = next or self

class Circle:

    def __init__(self):
        self.current = Node(0)

    def insert(self, value):
        prev, next = self.current.next, self.current.next.next
        node = Node(value, prev, next)
        prev.next = node
        next.prev = node
        self.current = node

    def remove(self):
        node = self.current
        for _ in range(7):
            node = node.prev
        prev, next = node.prev, node.next
        prev.next = next
        next.prev = prev
        self.current = next
        return node.value
    
    def print(self):
        if not DEBUG:
            return
        zero = self.current
        while zero.value != 0:
            zero = zero.next
        print('(0)' if zero is self.current else ' 0 ', end='')
        node = zero.next
        while node is not zero:
            print(('({})' if node is self.current else ' {} ').format(node.value), end='')
            node = node.next
        print()

def find_high_score(num_players, num_marbles):
    assert num_players >= 1
    assert num_marbles >= 0
    scores = [0] * num_players
    circle = Circle()
    start = time()
    circle.print()
    for marble in range(1, num_marbles + 1):

        if marble % 100_000 == 0:
            print('{:,}: {:,} marbles per second'.format(
                marble,
                int(marble // (time() - start))))

        player = (marble - 1) % num_players
        
        if marble % 23 == 0:
            scores[player] += marble + circle.remove()
        else:
            circle.insert(marble)

        circle.print()

    return max(scores)


if __name__ == '__main__':
    assert find_high_score( 9, 25) == 32

    assert find_high_score(10, 1618) == 8317
    assert find_high_score(13, 7999) == 146373
    assert find_high_score(17, 1104) == 2764
    assert find_high_score(21, 6111) == 54718
    assert find_high_score(30, 5807) == 37305

    print(find_high_score(476, 71_431 * 100))
