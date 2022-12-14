file = open("../inputs/day_14.txt")


def sliding_window(input_list, window_size):
    for i in range(len(input_list) - window_size + 1):
        yield input_list[i : i + window_size]


blocked = set()

for line in file:
    rocks = line.strip().split(" -> ")
    for rock_pair in sliding_window(rocks, 2):
        (x1, y1), (x2, y2) = map(int, rock_pair[0].split(",")), map(
            int, rock_pair[1].split(",")
        )
        blocked.update(
            set(
                (x, y)
                for x in range(min(x1, x2), max(x1, x2) + 1)
                for y in range(min(y1, y2), max(y1, y2) + 1)
            )
        )

lowest, initial, blocks_placed = max(block[1] for block in blocked), len(blocked), 0

while (500, 0) not in blocked:
    current_position = (500, 0)
    while True:
        if current_position[1] > lowest:
            if blocks_placed == 0:
                blocks_placed = len(blocked) - initial
            break
        for (dx, dy) in (0, 1), (-1, 1), (1, 1):
            next_position = (current_position[0] + dx, current_position[1] + dy)
            if next_position not in blocked:
                current_position = next_position
                break
        # Reached the last one without moving at all
        else:
            break
    blocked.add(current_position)

print(f"Part 1: {blocks_placed}")
print(f"Part 2: {len(blocked) - initial}")
