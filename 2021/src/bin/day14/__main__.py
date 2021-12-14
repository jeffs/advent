from day14 import puzzle, polymerism, sample
from day14.solve import solve


if __name__ == "__main__":

    # Print output like the Part 1 sample.
    chain = sample.chain
    print("Template:    ", chain)
    for i in range(1, 5):
        chain = polymerism.insert(chain, sample.rules)
        print(f"After step {i}:", chain)

    print()
    print(solve(sample.chain, sample.rules, 10))

    # Part 1
    print()
    print(solve(puzzle.chain, puzzle.rules, 10))
