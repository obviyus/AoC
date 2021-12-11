import heapq


def is_low_point(grid, x, y):
    for p, q in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]:
        if 0 <= p < len(grid) and 0 <= q < len(grid[0]) and grid[x][y] >= grid[p][q]:
            return False
    return True


def part_1():
    grid = []
    with open("2021/inputs/day9", "r") as file:
        for line in file.read().splitlines():
            grid.append(list(map(int, list(line))))
    return sum(
        grid[x][y] + 1
        for x in range(len(grid))
        for y in range(len(grid[0]))
        if is_low_point(grid, x, y)
    )


def neighbors(grid, x, y):
    for p, q in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]:
        if 0 <= p < len(grid) and 0 <= q < len(grid[0]) and grid[p][q] != 9:
            yield (p, q)
    return None


def part_2():
    grid = []
    with open("2021/inputs/day9", "r") as file:
        for line in file.read().splitlines():
            grid.append(list(map(int, list(line))))

    low_points = list(
        (x, y)
        for x in range(len(grid))
        for y in range(len(grid[0]))
        if is_low_point(grid, x, y)
    )
    basins = []

    for low_point in low_points:
        queue, seen = [low_point], set()
        while queue:
            point = queue.pop()
            if point not in seen:
                seen.add(point)
                queue.extend(list(neighbors(grid, *point)))
        heapq.heappush(basins, (-len(seen)))

    return heapq.heappop(basins) * heapq.heappop(basins) * heapq.heappop(basins) * -1


print(part_1())
print(part_2())
