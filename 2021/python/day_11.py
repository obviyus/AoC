def neighbors(grid, x, y):
    for p, q in [
        (x + 1, y),
        (x - 1, y),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x, y + 1),
        (x, y - 1),
    ]:
        if 0 <= p < len(grid) and 0 <= q < len(grid[0]):
            yield p, q


def part_1():
    grid = []
    with open("inputs/day11", "r") as file:
        for line in file.read().splitlines():
            grid.append(list(map(int, list(line))))

    flashes = 0
    for _ in range(100):
        queue = []
        for x, row in enumerate(grid):
            for y, _ in enumerate(row):
                grid[x][y] += 1
                if grid[x][y] > 9:
                    queue.append((x, y))

        flashed = set(queue)
        while queue:
            x, y = queue.pop()
            grid[x][y] = 0

            for p, q in neighbors(grid, x, y):
                if (p, q) not in flashed:
                    grid[p][q] += 1
                    if grid[p][q] > 9:
                        flashed.add((p, q))
                        queue.append((p, q))

        flashes += len(flashed)

    return flashes


def part_2():
    grid = []
    with open("inputs/day11", "r") as file:
        for line in file.read().splitlines():
            grid.append(list(map(int, list(line))))

    steps = 1
    while True:
        queue = []
        for x, row in enumerate(grid):
            for y, _ in enumerate(row):
                grid[x][y] += 1
                if grid[x][y] > 9:
                    queue.append((x, y))

        flashed = set(queue)
        while queue:
            x, y = queue.pop()
            grid[x][y] = 0

            for p, q in neighbors(grid, x, y):
                if (p, q) not in flashed:
                    grid[p][q] += 1
                    if grid[p][q] > 9:
                        flashed.add((p, q))
                        queue.append((p, q))

        if len(flashed) == 100:
            return steps
        steps += 1


print(part_1())
print(part_2())
