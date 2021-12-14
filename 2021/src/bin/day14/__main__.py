from day14 import puzzle, polymerism, sample
from day14.solve import solve
from day14.solve import solve2, update, count_pairs, dumps


from sys import exit, stderr

if __name__ == "__main__":

    # Print output like the Part 1 sample.
    chain = sample.chain
    counts = count_pairs(sample.chain)
    print("Template:    ", chain)
    for i in range(1, 5):
        chain = polymerism.insert(chain, sample.rules)
        counts = update(counts, sample.rules)
        got = dumps(counts)
        print(f"After step {i}:", chain)
        print("             ", got)
        want = dumps(count_pairs(chain))
        if want != got:
            print(file=stderr)
            print(f"want {want}\ngot {got}", file=stderr)
            exit(1)



    # print()
    # print(solve(sample.chain, sample.rules, 10))
    print(solve(puzzle.chain, puzzle.rules, 10))

    #print()
    #print(solve2(sample.chain, sample.rules, 10))
    #print(solve2(sample.chain, sample.rules, 40))

    #print()
    print(solve2(puzzle.chain, puzzle.rules, 40) - 1)
