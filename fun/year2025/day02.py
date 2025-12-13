# pyright: standard

# Testing and notes while trying to find a better solution to day 02.
# Some inspiration taken from
# https://www.reddit.com/r/adventofcode/comments/1pcbgai/2025_day_2_day_2_should_be_easy_right_closed/

import math


def p(r, q):
    return (10 ** (r * q) - 1) // (10**q - 1)


# N s.t. N * p(r, q) <= a
def t(a, r, q):
    return a // p(r, q)


def t_v2(a, r, q):
    return min((a // p(r, q), 10**q - 1))


# def t_v2(a, r, q):
#     return min((a // p(r, q), 10**q))


# def t_v2(a, r, q):
#     return min((a // p(r, q), 10 ** (q - 1)))


# Sum from 1 to N = 1/2 (n) (n + 1)
# Need to sum from 10^q to N
# ...
# sum_between_inclusive(a, b) = 1/2 (a+b) (b-a+1)


def sum_between_inclusive(a, b):
    # print(f"summing from {a} to {b} inclusive")
    return (a + b) * (b - a + 1) // 2


def sum_between(a, b):
    # print(f"summing from {a} to {b}")
    return (a + b - 1) * (b - a) // 2


def part1_maybe(start, end):
    # Repeat segment twice
    r = 2

    sum = 0

    # q is pattern length
    # for q in range(1, 10):
    #     print("t(start), t(end) =", t_v2(start, r, q), t(end, r, q))
    #
    #     sum_start = sum_between(10 ** (q - 1), t_v2(start, r, q))
    #     sum_end = sum_between_inclusive(10 ** (q - 1), t_v2(end, r, q))
    #
    #     print("start sum:", sum_start)
    #     print("end sum:", sum_end)
    #     print()
    #
    #     # Is this even correct?
    #     if sum_end < 0 or sum_start < 0:
    #         break
    #
    #     print(f"p(r, q) = {p(r, q)}")
    #
    #     sum += (sum_end - sum_start) * p(r, q)

    # return sum

    # return single_sum(end) - single_sum(start)

    return single_sum_r(end, 2) - single_sum_r(start, 2)


# Hehe this seems to be correct
def single_sum(n):
    # Repeat segment twice
    r = 2

    sum = 0

    # q is pattern length
    for q in range(1, 10):
        s = sum_between_inclusive(10 ** (q - 1), t_v2(n, r, q))

        # Is this even correct?
        if s <= 0:
            break

        # print(f"p(r, q) = {p(r, q)}")

        sum += s * p(r, q)

    return sum


# def single_sum_r(n, r):
#     sum = 0
#
#     # q is pattern length
#     q = 1
#     # for q in range(1, 10):
#     while 10 ** (q - 1) * p(r, q) <= n:
#         s = sum_between_inclusive(10 ** (q - 1), t_v2(n, r, q))
#
#         # Is this even correct?
#         # if s < 0:
#         #     break
#
#         # print(f"p(r, q) = {p(r, q)}")
#
#         sum += s * p(r, q)
#         q += 1
#
#     return sum


# def single_sum_r(n, r):
#     sum = 0
#
#     # q is pattern length
#     q = 1
#
#     while 10 ** (q - 1) * p(r, q) <= n:
#         q += 1
#
#     print(q)
#     print()
#
#     s = sum_between_inclusive(1, t_v2(n, r, q))
#     sum += s * p(r, q)
#
#     return sum


def single_sum_r(n, r):
    sum = 0

    # q is pattern length
    q = 1
    while 10 ** (q - 1) * p(r, q) <= n:
        s = sum_between_inclusive(10 ** (q - 1), t_v2(n, r, q))

        print(10 ** (q - 1), t_v2(n, r, q), p(r, q))

        sum += s * p(r, q)
        q += 1

    print()

    return sum


def single_sum_many(n):
    sum = 0

    double_count_removal = [None, None, 1, 1, 0, 1, -1, 1, 0, 0, -1, 1, 0, 1, -1, -1]

    # we are double counting
    # r is segment repeats
    for r in range(2, math.ceil(math.log10(n + 1)) + 1):
        # for r in range(2, math.ceil(math.log10(n + 1))):
        sum += double_count_removal[r] * single_sum_r(n, r)
        # sum += single_sum_r(n, r)

    return sum


def naive(start, end):
    # pattern lengths 1, 2, ...
    # repeats 2, 3, ...
    # avoid double counting??

    for q in range(1, 5):
        for r in range(1, 5):
            pass

    pass


def parse(txt):
    res = []
    for pair in txt.split(","):
        res.append(tuple(map(int, pair.split("-"))))
    return res


def part1(input):
    sum = 0
    for thing in input:
        s = part1_maybe(thing[0] - 1, thing[1])
        print(f" ==> {s}")
        sum += s

    return sum


def part2(input):
    sum = 0
    for thing in input:
        s1 = single_sum_many(thing[0] - 1)
        s2 = single_sum_many(thing[1])
        sum += s2 - s1

    return sum


def main():
    with open("../../input/year2025/day02.txt") as f:
        input = f.read().strip()

    # input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    # input = "11-22,95-115,998-1012"
    # input = "11-22,95-115,998-1012,1188511880-1188511890"
    # input = "11-22,95-115,998-1012"

    input = parse(input)
    p1 = part1(input)
    p2 = part2(input)

    print(p1, " (1227775554)")
    print(p2, " (4174379265)")

    print()
    print(f"44137147775 < {p2} < 44986401385 => {44137147775 < p2 < 44986401385}")


if __name__ == "__main__":
    main()
