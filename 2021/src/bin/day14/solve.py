from collections import Counter

from day14 import polymerism


def solve(chain, rules, n):
    for i in range(n):
        chain = polymerism.insert(chain, rules)
    counts = Counter(chain)
    return max(counts.values()) - min(counts.values())
