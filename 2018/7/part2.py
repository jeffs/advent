#!/usr/bin/env python3

import sys


TIME_OVERHEAD = 60
NUM_WORKERS = 4


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


# Work ready to start.
roots = set(node for node in nodes if not incoming_edges[node])

# Work currently in progress (one queue per worker).
work_queues = [None] * NUM_WORKERS

cycle_count = 0
while roots or any(work_queues):
    # There's new work to begin, or work still in progress.

    # Let each worker do one second of work.
    completed = []
    for i in range(NUM_WORKERS):
        queue = work_queues[i]
        if queue:
            node, time = queue
            time -= 1
            if time:
                work_queues[i] = (node, time)
            else:
                completed.append(node)
                work_queues[i] = None

    # Add any newly unblocked tasks the set of available work.
    for node in completed:
        for child in outgoing_edges[node]:
            incoming_edges[child].remove(node)
            if not incoming_edges[child]:
                roots.add(child)

    # Assign idle workers new tasks, if any are ready.
    for i in range(NUM_WORKERS):
        if roots and not work_queues[i]:
            root = min(roots)
            roots.remove(root)
            time = TIME_OVERHEAD + (ord(root) - ord('A') + 1)
            work_queues[i] = (root, time)

    print(  '{:2d}'.format(cycle_count),
            ''.join(q[0] if q else '.' for q in work_queues),
            file=sys.stderr)

    if any(work_queues):
        cycle_count += 1

print(cycle_count)
