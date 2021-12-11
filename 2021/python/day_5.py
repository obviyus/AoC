from collections import Counter


def part_1():
    grid_counter = Counter()
    with open("inputs/day5", "r") as file:
        for line in file:
            start, end = line.strip().split("->")
            x1, y1 = map(int, start.split(","))
            x2, y2 = map(int, end.split(","))

            if x1 == x2 or y1 == y2:
                for x in range(min(x1, x2), max(x1, x2) + 1):
                    for y in range(min(y1, y2), max(y1, y2) + 1):
                        grid_counter[(x, y)] += 1

    return len([x for _, count in grid_counter.items() if count > 1])


def part_2():
    grid_counter = Counter()
    with open("inputs/day5", "r") as file:
        for line in file:
            start, end = line.strip().split("->")
            x1, y1 = map(int, start.split(","))
            x2, y2 = map(int, end.split(","))

            slope_x = 1 if x2 > x1 else -1
            slope_y = 1 if y2 > y1 else -1
            if x1 == x2:
                slope_x = 0
            if y1 == y2:
                slope_y = 0

            grid_counter[(x1, y1)] += 1
            while x1 != x2 or y1 != y2:
                x1 += slope_x
                y1 += slope_y
                grid_counter[(x1, y1)] += 1

    return len([_ for _, count in grid_counter.items() if count > 1])


print(part_1())
print(part_2())
