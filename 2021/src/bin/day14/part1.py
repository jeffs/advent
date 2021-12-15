from collections import Counter


def flatten(pairs):
    return "".join(c for pair in pairs for c in pair)


def insert(chain, rules):
    insertions = []
    for i in range(1, len(chain)):
        pair = chain[i - 1 : i + 1]
        insertions.append(rules.get(pair, ""))
    pairs = tuple(zip(chain, insertions))
    prefix = flatten(pairs)
    return prefix + chain[-1] if len(pairs) < len(chain) else prefix


def solve(chain, rules, n):
    for i in range(n):
        chain = insert(chain, rules)
    counts = Counter(chain)
    return max(counts.values()) - min(counts.values())
