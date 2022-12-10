import math

file = open("../inputs/day_08.txt")

# Part 1
grid = []

for line in file:
    line = line.strip()
    if line:
        grid.append([int(x) for x in line])

rows, cols = len(grid[0]), len(grid)

visible = 0
for x in range(rows):
    for y in range(cols):
        if x == 0 or x == rows - 1 or y == 0 or y == cols - 1:
            visible += 1
            continue

        current_tree = grid[x][y]
        current_row_till_current_tree = grid[x][:y]
        current_row_after_current_tree = grid[x][y + 1 :]
        current_col_till_current_tree = [grid[i][y] for i in range(x)]
        current_col_after_current_tree = [grid[i][y] for i in range(x + 1, cols)]

        if (
            current_tree > max(current_row_till_current_tree)
            or current_tree > max(current_row_after_current_tree)
            or current_tree > max(current_col_till_current_tree)
            or current_tree > max(current_col_after_current_tree)
        ):
            visible += 1

print(f"Part 1: {visible}")

# Part 2 To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if
# you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a
# tree is right on the edge, at least one of its viewing distances will be zero.)

max_scenic_score = 0
for x in range(rows):
    for y in range(cols):
        scores = [0, 0, 0, 0]

        current_tree = grid[x][y]
        current_row_till_current_tree = grid[x][:y]
        current_row_after_current_tree = grid[x][y + 1 :]
        current_col_till_current_tree = [grid[i][y] for i in range(x)]
        current_col_after_current_tree = [grid[i][y] for i in range(x + 1, cols)]

        # Up
        for i in range(len(current_col_till_current_tree) - 1, -1, -1):
            if current_col_till_current_tree[i] >= current_tree:
                scores[0] += 1
                break
            scores[0] += 1

        # Down
        for i in range(len(current_col_after_current_tree)):
            if current_col_after_current_tree[i] >= current_tree:
                scores[1] += 1
                break
            scores[1] += 1

        # Left
        for i in range(len(current_row_till_current_tree) - 1, -1, -1):
            if current_row_till_current_tree[i] >= current_tree:
                scores[2] += 1
                break
            scores[2] += 1

        # Right
        for i in range(len(current_row_after_current_tree)):
            if current_row_after_current_tree[i] >= current_tree:
                scores[3] += 1
                break
            scores[3] += 1

        max_scenic_score = max(max_scenic_score, math.prod(scores))

print(f"Part 2: {max_scenic_score}")
