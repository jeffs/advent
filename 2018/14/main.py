#!/usr/bin/env python3


import sys


DEBUG = False

if DEBUG:
    def log_scores(scores, elf1, elf2):
        for i in range(len(scores)):
            template = '<{}>' if elf1 == i and elf2 == i \
                    else '({})' if elf1 == i \
                    else '[{}]' if elf2 == i \
                    else ' {} '
            print(template.format(scores[i]), end='')
        print()
else:
    def log_scores(scores, elf1, elf2):
        pass


def part1(count):
    scores = [3, 7]
    elf1, elf2 = 0, 1
    log_scores(scores, elf1, elf2)
    while len(scores) < count + 10:
        n = scores[elf1] + scores[elf2]
        recipe1, recipe2 = n // 10, n % 10
        if recipe1:
            scores.append(recipe1)
        if len(scores) < count + 10:
            scores.append(recipe2)
        elf1 = (elf1 + 1 + scores[elf1]) % len(scores)
        elf2 = (elf2 + 1 + scores[elf2]) % len(scores)
        log_scores(scores, elf1, elf2)
    return ''.join(map(str, scores[-10:]))


def part1_test(count, expected):
    value = part1(count)
    if value != expected:
        print('FAIL: count={}: {} != {}'.format(count, value, expected))
        sys.exit(1)


def part2(tail):
    scores = [3, 7]
    elf1, elf2 = 0, 1
    log_scores(scores, elf1, elf2)
    def not_done():
        return ''.join(map(str, scores[-len(tail):])) != tail
    while not_done():
        print('\r{}'.format(len(scores)), end='', file=sys.stderr)
        n = scores[elf1] + scores[elf2]
        recipe1, recipe2 = n // 10, n % 10
        if recipe1:
            scores.append(recipe1)
        if not_done():
            scores.append(recipe2)
        elf1 = (elf1 + 1 + scores[elf1]) % len(scores)
        elf2 = (elf2 + 1 + scores[elf2]) % len(scores)
        log_scores(scores, elf1, elf2)
    print('\r{}'.format(len(scores)), file=sys.stderr)
    return len(scores) - len(tail)

def part2_test(tail, expected):
    count = part2(tail)
    if count != expected:
        print('FAIL: tail={}: {} != {}'.format(tail, count, expected))
        sys.exit(2)


if __name__ == '__main__':
    part1_test(9, '5158916779')
    part1_test(5, '0124515891')
    part1_test(18, '9251071085')
    part1_test(2018, '5941429882')
    print(part1(440231))

    part2_test('51589', 9)
    part2_test('01245', 5)
    part2_test('92510', 18)
    part2_test('59414', 2018)
    print(part2('440231'))
