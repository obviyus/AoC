from collections import Counter

def part_1():
    grid_counter = Counter()
    with open('/home/obviyus/Desktop/aoc/2021/inputs/day5', 'r') as file:
        for line in file:
            start, end = line.strip().split('->')
            x1, y1 = map(int, start.split(','))
            x2, y2 = map(int, end.split(','))

            if x1 == x2 or y1 == y2:
                for x in range(min(x1, x2), max(x1, x2) + 1):
                    for y in range(min(y1, y2), max(y1, y2) + 1):
                        grid_counter[(x, y)] += 1

    return len([x for _, count in grid_counter.items() if count > 1])

print(part_1())