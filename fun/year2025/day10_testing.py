# pyright: standard

import math
import time


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


def map_mat(mat, fn):
    rows = len(mat)
    cols = len(mat[0])

    res = [[0 for _ in range(cols)] for _ in range(rows)]

    for i in range(rows):
        for j in range(cols):
            res[i][j] = fn(mat[i][j])

    return res


def gen_matrix(buttons, joltage):
    rows = len(joltage)
    cols = len(buttons)

    mat = [[0 for _ in range(cols + 1)] for _ in range(rows)]
    upper_bounds = [2048 for _ in range(cols)]
    lower_bounds = [0 for _ in range(cols)]

    for col in range(cols):
        for toggle in buttons[col]:
            mat[toggle][col] = 1

            if joltage[toggle] < upper_bounds[col]:
                upper_bounds[col] = joltage[toggle]

    for i in range(rows):
        mat[i][cols] = joltage[i]

    return mat, lower_bounds, upper_bounds


def print_matrix(mat):
    max_len = [2 for _ in range(len(mat[0]))]

    for row in mat:
        for i in range(len(max_len)):
            num_len = len(str(row[i]))
            if num_len > max_len[i]:
                max_len[i] = num_len

    for row in mat:
        for i in range(len(row)):
            print(f"{row[i]:>{max_len[i]}} ", end="")
        print()


def swap_rows(mat, src, dst):
    if src != dst:
        mat[src], mat[dst] = mat[dst], mat[src]


def scale_row(mat, row, alpha):
    for i in range(len(mat[0])):
        mat[row][i] *= alpha


def div_row(mat, row, alpha):
    for i in range(len(mat[0])):
        mat[row][i] //= alpha


def add_and_scale_row(mat, src, dst, alpha):
    for i in range(len(mat[0])):
        mat[dst][i] += mat[src][i] * alpha


def rref(mat):
    rows = len(mat)
    cols = len(mat[0]) - 1
    row = 0
    col = 0

    while row < rows and col < cols:
        # Find a pivot row
        pivot_row = -1
        pivot_val = 0

        for r in range(row, rows):
            v = mat[r][col]

            if v != 0:
                pivot_row = r
                pivot_val = v
                break

        # No pivot found, so skip this column
        if pivot_row == -1:
            col += 1
            continue

        # Move pivot row to the current row
        swap_rows(mat, pivot_row, row)

        if mat[row][col] < 0:
            scale_row(mat, row, -1)
            pivot_val *= -1

        # Scale by LCM so all elements are divisible
        # Remove row from remaining rows if possible

        for r in range(rows):
            coef = mat[r][col]
            if r != row and coef != 0:
                lcm = math.lcm(pivot_val, abs(coef))
                scale_dst = lcm // abs(coef)
                scale_src = lcm // pivot_val * (1 if coef > 0 else -1)

                for c in range(cols + 1):
                    mat[r][c] = mat[r][c] * scale_dst - mat[row][c] * scale_src

        row += 1
        col += 1


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


"""

Things:

b0 + b11 = 25
b0 <= 25 - b11
b0 <= 25, b11 <= 25

b1 - b11 - b12 = -184
b1 <= -184 + b11 + b12  <== Upper bound

-b11 <= -184 - b1 + b12
b11 >= 184 + b1 - b12   <== Lower bound!!!



b1 - b11 - b12 = -184
0 <= b1 <= 94
0 <= b11 <= 10
0 <= b12 <= 230

0 - 0 - 184

-b11 = -184 - b1 + b12
-b11 <= -184 -0 + 230
-b11 <= 46
b11 >= -46

[0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -184]

b1 - b11 - b12 = -184
0 <= b1 <= 94
-b12 = -184 - b1 + b11


0  0  1  0  0  3  0  2  1  0  44
b2 + 3b5 + 2b7 + b8 = 44
b5 = 6
b7 = 0

b2 + 24 + b8 = 44
b2 + b8 = 20


1  0  0  0  0 -2  0 -1 -1  0 -22
b0 - 2b5 - b7 - b8 = -22
b5 = 6
b7 = 0

b0 - 12 - 0 - b8 = -22
b0 - b8 = -10
-b8 = -12 - b0
b8 >= 10

"""


def solve_with_assignment(rref_mat, free_vars, assignment):
    rows = len(rref_mat)
    cols = len(rref_mat[0]) - 1

    row = 0
    col = 0

    total = sum(assignment)

    res = [0 for _ in range(cols)]

    while row < rows and col < cols:
        while col < cols and rref_mat[row][col] == 0:
            col += 1

        if col >= cols:
            continue

        target = rref_mat[row][cols]

        for var, val in zip(free_vars, assignment):
            target -= rref_mat[row][var] * val

        presses = target // rref_mat[row][col]

        if presses < 0:
            return None

        res[col] = presses

        total += presses

        row += 1

    return total


def recurse(rref_mat, free_vars, lower_bounds, upper_bounds, assignment, depth):
    rows = len(rref_mat)
    cols = len(rref_mat[0]) - 1

    if len(assignment) == len(free_vars):
        # Fully assigned. Solve remaining variables
        return solve_with_assignment(rref_mat, free_vars, assignment)

    # Identify lower and upper bounds for the current free variable
    free_col_idx = free_vars[depth]

    lower_bound = lower_bounds[free_vars[depth]]
    upper_bound = upper_bounds[free_vars[depth]]

    # lower_bound = 0
    # upper_bound = 2048

    for row in rref_mat:
        target = row[cols]
        coef = row[free_col_idx]

        if coef == 0:
            continue

        # Keep track of assigned variables
        assigned_idx = 0

        for c in range(cols):
            if assigned_idx < depth and c == free_vars[assigned_idx]:
                target -= row[free_vars[assigned_idx]] * assignment[assigned_idx]
                assigned_idx += 1
                continue

            if c != free_col_idx:
                if row[c] > 0:
                    target -= row[c] * lower_bounds[c]
                else:
                    target -= row[c] * upper_bounds[c]

        # coef > 0 => upper bound
        # coef < 0 => -(lower bound)

        if coef > 0:
            upper_bound = min((upper_bound, target // coef))
        else:
            lower_bound = max((lower_bound, target // coef))

        if upper_bound < lower_bound:
            # Impossible to satisfy
            return None

    # Try each possible value
    best = 1000000

    for b in range(lower_bound, upper_bound + 1):
        assignment.append(b)
        result = recurse(
            rref_mat, free_vars, lower_bounds, upper_bounds, assignment, depth + 1
        )
        assignment.pop()

        if result is not None and result <= best:
            best = result

    return best


def part2():
    total = 0

    with open("../../input/year2025/day10.txt") as f:
        for line in f.readlines():
            buttons, joltage = parse_line(line)
            mat, lower_bounds, upper_bounds = gen_matrix(buttons, joltage)

            rref(mat)

            free_vars = find_free_variables(mat)

            res = recurse(mat, free_vars, lower_bounds, upper_bounds, [], 0)

            # print(buttons)
            # print(joltage)
            print(res)
            # print()

            if res is not None:
                total += res

    return total


def main():
    #

    #   0   1   2   3   4   5   6   7   8   9  19  11  12
    # [16, 17,  2, 18,  2, 19,  0, 18, 21, 20,  0,  9, 192]
    buttons = [
        (6, 7, 8),
        (3, 5, 7),
        (2, 4),
        (1, 3, 4, 9),
        (0, 1, 2, 3, 6, 7, 9),
        (0, 1, 2, 3, 5, 8),
        (3, 8),
        (2, 3, 4, 6, 7, 8, 9),
        (3, 4, 7, 8),
        (0, 1, 2, 3, 4, 5, 7, 8),
        (0, 1, 2, 4, 7),
        (2, 4, 6),
        (5, 6, 8, 9),
    ]
    joltage = [41, 59, 70, 115, 88, 248, 237, 94, 286, 230]

    # # => 0  1  2  3  4  5
    # #   [1, 3, 0, 3, 1, 2]
    # buttons = [[3], [1, 3], [2], [2, 3], [0, 2], [0, 1]]
    # joltage = [3, 5, 4, 7]

    # buttons = [(0, 2, 3, 4), (2, 3), (0, 4), (0, 1, 2), (1, 2, 3, 4)]
    # joltage = [7, 5, 12, 7, 2]

    # buttons = [(0, 1, 2, 3, 4), (0, 3, 4), (0, 1, 2, 4, 5), (1, 2)]
    # joltage = [10, 11, 11, 5, 10, 5]

    # buttons = [
    #     [2, 3, 6],
    #     [3, 4, 5, 6, 7, 8],
    #     [0, 1, 2, 3, 4, 6, 8],
    #     [0, 1, 2, 3, 4, 5, 6, 8],
    #     [1, 3],
    #     [1, 4, 5, 6, 8],
    #     [6, 8],
    #     [0, 1, 3, 4, 6, 7],
    #     [1, 2, 3, 4, 8],
    #     [4, 6],
    # ]
    # joltage = [44, 211, 64, 232, 70, 14, 80, 19, 53]

    # buttons = [
    #     [0, 1, 2, 3, 6, 9],
    #     [1, 2, 3, 4, 6, 7, 8],
    #     [0, 2, 4, 8],
    #     [0, 2, 4, 5, 6, 8, 9],
    #     [0, 3, 4, 6],
    #     [0, 2, 3, 4, 5, 6, 8, 9],
    #     [0, 1, 5, 7, 8],
    #     [2, 3, 7, 8, 9],
    # ]
    # joltage = [71, 32, 75, 68, 71, 42, 74, 32, 73, 53]

    buttons = [
        [3, 5],
        [0, 1, 2, 3, 4, 5],
        [0, 1, 3, 4, 5, 6],
        [0, 2, 3, 4, 7],
        [1, 2, 3, 5, 7],
        [2, 6],
        [0, 1, 3, 4, 6, 7],
        [6, 7],
        [1, 6, 7],
        [2, 3, 4, 7],
    ]
    joltage = [37, 66, 49, 72, 47, 43, 45, 64]

    mat, lower_bounds, upper_bounds = gen_matrix(buttons, joltage)

    print("initial matrix")
    print_matrix(mat)
    print()

    print(f"lower bounds: {lower_bounds}")
    print(f"upper bounds: {upper_bounds}")
    print()

    rref(mat)

    print("rref matrix")
    print_matrix(mat)
    print()

    free_vars = find_free_variables(mat)
    print("free:", free_vars)

    res = recurse(mat, free_vars, lower_bounds, upper_bounds, [], 0)

    print(res)

    print(part2())


if __name__ == "__main__":
    main()
