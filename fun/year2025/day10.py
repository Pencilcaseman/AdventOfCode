# pyright: standard

import math
import time

from multiprocessing import Pool

from fractions import Fraction


def swap_rows(mat, src, dst):
    mat[src], mat[dst] = mat[dst], mat[src]


def scale_row(mat, row, alpha):
    for i in range(len(mat[0])):
        mat[row][i] *= alpha


def add_and_scale_row(mat, src, dst, alpha):
    for i in range(len(mat[0])):
        mat[dst][i] += mat[src][i] * alpha


def rref(mat):
    rows = len(mat)
    cols = len(mat[0])

    pivot_row = 0

    for col in range(cols):
        if pivot_row >= rows:
            break

        pivot_candidate = -1
        for r in range(pivot_row, rows):
            if abs(mat[r][col]) > 1e-7:
                pivot_candidate = r
                break

        if pivot_candidate == -1:
            continue

        swap_rows(mat, pivot_row, pivot_candidate)

        pivot_val = mat[pivot_row][col]
        scale_row(mat, pivot_row, 1 / pivot_val)

        for r in range(rows):
            if r != pivot_row:
                factor = mat[r][col]
                add_and_scale_row(mat, pivot_row, r, -factor)

        pivot_row += 1

    return mat


def find_free_variables(rref_mat):
    rows = len(rref_mat)
    cols = len(rref_mat[0])
    free = []

    col = 0

    for row in range(rows):
        while col < cols - 1 and rref_mat[row][col] == 0:
            free.append(col)
            col += 1
        col += 1

    while col < cols - 1:
        free.append(col)
        col += 1

    return free


def solve_with_attempt(rref_mat, free_vars, attempt):
    vars = len(rref_mat[0]) - 1
    solved = [0 for _ in range(vars)]

    for i, x in enumerate(free_vars):
        solved[x] = attempt[i]

    col = 0
    for row in range(len(rref_mat)):
        while col in free_vars:
            col += 1

        if col >= vars:
            break

        num = rref_mat[row][-1]
        for b_idx in range(len(free_vars)):
            num -= attempt[b_idx] * rref_mat[row][free_vars[b_idx]]

        solved[col] = num

        col += 1

    return solved


def solve_recursive(rref_mat, max_vals, free_vars, attempt=None, depth=0):
    if attempt is None:
        attempt = []

    if depth == len(free_vars):
        return solve_with_attempt(rref_mat, free_vars, attempt)

    # Find lower and upper bounds for free variable b_depth given the current
    # variable assignment.

    num_vars = len(rref_mat[0]) - 1

    high = max_vals[free_vars[depth]]

    for row in rref_mat:
        target = row[-1]

        # Undecided coefficients with opposite signs => no limit
        seen_neg = False

        index = 0
        col = 0
        while col < num_vars:
            if index < len(attempt) and col == free_vars[index]:
                target -= attempt[index] * row[free_vars[index]]
                index += 1
            elif row[col] < 0:
                seen_neg = True

            col += 1

        coef = row[free_vars[depth]]

        if not seen_neg and coef != 0:
            high = min((high, target / coef))

    best = None

    for free_var_val in range(0, int(high + 1)):
        attempt.append(free_var_val)
        solved = solve_recursive(rref_mat, max_vals, free_vars, attempt, depth + 1)
        attempt.pop()

        if solved is not None and all(
            x >= 0 and (x.numerator % x.denominator == 0)
            for x in solved
            for x in solved
        ):
            if best is None or sum(solved) < sum(best):
                best = solved

    return best


def gen_matrix(buttons, joltage):
    rows = len(joltage)
    cols = len(buttons)

    mat = [[0 for _ in range(cols + 1)] for _ in range(rows)]

    for col in range(cols):
        for switch in buttons[col]:
            mat[switch][col] = Fraction(1)

    for i in range(rows):
        mat[i][cols] = joltage[i]

    max_vals = [2048 for _ in range(cols)]

    for row in mat:
        for i in range(len(row) - 1):
            if row[i] != 0 and row[-1] < max_vals[i]:
                max_vals[i] = row[-1]

    return mat, max_vals


def full_solve(buttons, joltage):
    matrix, max_vals = gen_matrix(buttons, joltage)
    rref_mat = rref(matrix)
    free_vars = find_free_variables(rref_mat)
    return solve_recursive(rref_mat, max_vals, free_vars)


def parse_line(line):
    parts = line.split()[1:]

    buttons = []
    joltage = []

    for part in parts:
        if part.startswith("("):
            b = list(map(int, part.strip("()").split(",")))
            buttons.append(b)
        else:
            joltage = list(map(int, part.strip("{}").split(",")))

    return (buttons, joltage)


def part2(txt):
    res = 0

    for line in txt.split("\n"):
        buttons, joltage = parse_line(line)
        partial = full_solve(buttons, joltage)
        if partial is None:
            return None
        res += sum(partial)

    return res


def main():
    # buttons = [[3], [1, 3], [2], [2, 3], [0, 2], [0, 1]]
    # joltage = [3, 5, 4, 7]

    # buttons = [(0, 2, 3, 4), (2, 3), (0, 4), (0, 1, 2), (1, 2, 3, 4)]
    # joltage = [7, 5, 12, 7, 2]

    # buttons = [(0, 1, 2, 3, 4), (0, 3, 4), (0, 1, 2, 4, 5), (1, 2)]
    # joltage = [10, 11, 11, 5, 10, 5]

    buttons = [
        (3, 6),
        (0, 1, 2, 3, 4, 5, 7, 9),
        (0, 1, 5, 6, 7, 8, 9),
        (1, 9),
        (0, 1, 3, 4, 5, 6, 7),
        (0, 1, 2, 3, 4, 5),
        (1, 2, 3, 4, 5, 6, 7, 8),
        (2, 3, 5, 7, 8),
        (2, 3, 5, 7, 9),
        (0, 1, 2, 3, 4, 6, 9),
        (4, 5, 6, 7, 8),
        (3, 6, 7, 8, 9),
    ]
    joltage = [52, 67, 66, 109, 49, 65, 70, 66, 33, 72]

    start = time.perf_counter_ns()
    matrix, max_vals = gen_matrix(buttons, joltage)
    end = time.perf_counter_ns()
    print(f"Matrix Creation: {end - start}ns")

    print("\n".join(map(str, map_mat(matrix, int))))
    print()

    start = time.perf_counter_ns()
    res = rref(matrix)
    end = time.perf_counter_ns()
    print(f"RREF: {end - start}ns")

    print("\n".join(map(str, map_mat(res, int))))

    free_vars = find_free_variables(res)
    print(free_vars)

    print()

    solved = solve_recursive(res, max_vals, free_vars)
    print(solved)
    print()

    test = """[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"""
    print(part2(test.strip()))

    print()

    with open("../../input/year2025/day10.txt", "r") as f:
        print(part2(f.read().strip()))


def map_mat(mat, fn):
    rows = len(mat)
    cols = len(mat[0])

    res = [[0 for _ in range(cols)] for _ in range(rows)]

    for i in range(rows):
        for j in range(cols):
            res[i][j] = fn(mat[i][j])

    return res


if __name__ == "__main__":
    main()
