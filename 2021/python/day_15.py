from heapq import heappop, heappush


def neighbors(grid, x, y):
    for p, q in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]:
        if 0 <= p < len(grid) and 0 <= q < len(grid[0]):
            yield p, q


def maze_solver(grid) -> int:
    costs = [[float("inf")] * len(grid[0]) for _ in range(len(grid))]
    queue = [(0, 0, 0)]

    while queue:
        cost, x, y = heappop(queue)
        if x == len(grid) - 1 and y == len(grid[0]) - 1:
            return cost
        if costs[x][y] < cost:
            continue

        for p, q in neighbors(grid, x, y):
            if cost + grid[p][q] < costs[p][q]:
                costs[p][q] = cost + grid[p][q]
                heappush(queue, (costs[p][q], p, q))


def part_1():
    grid = []
    with open("inputs/day15", "r") as file:
        for line in file.read().splitlines():
            grid.append(list(map(int, list(line))))

    return maze_solver(grid)


def part_2():
    grid = []
    with open("inputs/day15", "r") as file:
        for line in file.read().splitlines():
            grid.append(list(map(int, list(line))))

    expanded = [[0 for _ in range(5 * len(grid))] for _ in range(5 * len(grid[0]))]
    for x in range(len(expanded)):
        for y in range(len(expanded[0])):
            cost = (
                grid[x % len(grid)][y % len(grid[0])]
                + x // len(grid)
                + y // len(grid[0])
            )
            expanded[x][y] = cost if cost < 10 else cost - 9

    return maze_solver(expanded)


print(part_1())
print(part_2())
