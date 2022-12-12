file = open("../inputs/day_12.txt")

grid = []
start_position, end_position = None, None


def get_neighbors(x, y, grid):
    adjacent = []
    for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
        if 0 <= x + dx < len(grid) and 0 <= y + dy < len(grid[0]):
            if grid[x + dx][y + dy] - grid[x][y] <= 1:
                adjacent.append((x + dx, y + dy))

    return adjacent


neighbors = []

for x, line in enumerate(file):
    line = line.strip()
    grid.append([])
    for y, char in enumerate(line):
        if char == "S":
            start_position = (x, y)
            char = "a"
        elif char == "E":
            end_position = (x, y)
            char = "z"

        current_elevation = ord(char) - 97
        grid[-1].append(current_elevation)

        if char == "a":
            neighbors.append((x, y))

steps, visited = -1, set()

while neighbors:
    next_neighbors = []
    for neighbor in neighbors:
        if neighbor not in visited:
            visited.add(neighbor)
            next_neighbors.extend(get_neighbors(neighbor[0], neighbor[1], grid))

    neighbors = next_neighbors
    steps += 1

    if end_position in visited:
        break

print(steps)
