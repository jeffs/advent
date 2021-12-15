import json


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


def solve(chain, rules, n):
    counts = count_pairs(chain)
    for i in range(n):
        counts = update(counts, rules)

    c_counts = {}
    for (k, v) in counts.items():
        upsert(c_counts, k[0], v)
    c_counts[chain[-1]] += 1

    return max(c_counts.values()) - min(c_counts.values())
