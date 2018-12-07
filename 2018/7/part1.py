#!/usr/bin/env python3


def parse_edge(line):
    words = line.split()
    return words[1], words[7]


nodes = set()
incoming_edges = {}
outgoing_edges = {}
with open('input') as lines:
    for source, target in map(parse_edge, lines):
        nodes.update(source, target)
        incoming_edges.setdefault(target, set()).add(source)
        outgoing_edges.setdefault(source, set()).add(target)


for node in nodes:
    incoming_edges.setdefault(node, set())
    outgoing_edges.setdefault(node, set())


order = []
roots = set(node for node in nodes if not incoming_edges[node])
while roots:
    root = min(roots)
    roots.remove(root)
    order.append(root)
    for child in set(outgoing_edges[root]):
        incoming_edges[child].remove(root)
        outgoing_edges[root].remove(child)
        if not incoming_edges[child]:
            roots.add(child)
            

assert not any(incoming_edges.values())
assert not any(outgoing_edges.values())

print(''.join(order))
