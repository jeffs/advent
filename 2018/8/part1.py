#!/usr/bin/env python3


from collections import namedtuple


Node = namedtuple('Node', ['children', 'metadata'])


def build_tree(numbers):
    queue = list(numbers)
    queue.reverse()
    pop = queue.pop
    def imp():
        num_children = pop()
        num_metadata = pop()
        children = tuple(imp() for _ in range(num_children))
        metadata = tuple(pop() for _ in range(num_metadata))
        return Node(children, metadata)
    return imp()


def sum_metadata(root):
    assert root
    return sum(map(sum_metadata, root.children)) + sum(root.metadata)


if __name__ == '__main__':
    with open('input') as lines:
        numbers = tuple(int(word) for line in lines for word in line.split())
    tree = build_tree(numbers)
    print(sum_metadata(tree))

