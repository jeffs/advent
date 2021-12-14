import json
from collections import Counter
from math import ceil

from day14 import polymerism


def solve(chain, rules, n):
    for i in range(n):
        chain = polymerism.insert(chain, rules)
    counts = Counter(chain)
    return max(counts.values()) - min(counts.values())


def upsert(counts, key, count):
    counts[key] = counts.get(key, 0) + count


def count_pairs(chain):
    counts = {}
    for i in range(len(chain) - 1):
        upsert(counts, chain[i : i + 2], 1)
    return counts


def update(counts, rules):
    result = {}
    for (key, count) in counts.items():
        if key in rules:
            a, b, c = key[0], rules[key], key[1]
            upsert(result, a + b, count)
            upsert(result, b + c, count)
        else:
            upsert(result, key, count)
    return result


def dumps(counts):
    items = {k: counts[k] for k in sorted(counts.keys())}
    return json.dumps(items)


def solve2(chain, rules, n):
    counts = count_pairs(chain)
    for i in range(n):
        counts = update(counts, rules)

    c_counts = { }
    for (k, v) in counts.items():
        upsert(c_counts, k[0], v)
        upsert(c_counts, k[1], v)

    return ceil((max(c_counts.values()) - min(c_counts.values())) / 2)
