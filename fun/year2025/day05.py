# pyright: standard


def part1(ranges, nums):
    count = 0

    for n in nums:
        for r in ranges:
            if r[0] <= n <= r[1]:
                count += 1
                break

    return count


def part2(ranges):
    active = []
    merged = []

    def push(r):
        merged.append(r)
        active.append(True)

    current = None
    r_idx = 0

    while r_idx < len(ranges):
        if current is None:
            current = ranges[r_idx]

        overlap = False

        # Check for overlap
        for i in range(len(merged)):
            if not active[i]:
                continue

            a = current[0]
            b = current[1]

            x = merged[i][0]
            y = merged[i][1]

            if a <= y and b >= x:
                # Overlap
                new = (min((a, x)), max((b, y)))
                active[i] = False
                current = new
                overlap = True

                break

        if not overlap:
            push(current)
            r_idx += 1
            current = None

        pass

    res = 0
    for i in range(len(merged)):
        if active[i]:
            res += merged[i][1] - merged[i][0] + 1

    return res


def parse():
    ranges = []
    nums = []

    input = None

    with open("../../input/year2025/day05.txt", "r") as f:
        input = f.read()

    is_ranges = True
    for line in input.split("\n"):
        line = line.strip()

        if line == "":
            is_ranges = False
            continue

        if is_ranges:
            ranges.append(list(map(int, line.split("-"))))
        else:
            nums.append(int(line))

    return ranges, nums


def main():
    ranges, nums = parse()
    print(part1(ranges, nums))
    print(part2(ranges))


if __name__ == "__main__":
    main()
