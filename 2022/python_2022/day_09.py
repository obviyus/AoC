file = open("../inputs/day_09.txt")

sign = lambda x: x and (1, -1)[x < 0]


def snake(instructions, knot_count):
    knots = [[0, 0] for _ in range(knot_count)]  # (x,y) coordinates of each knot
    visited = {(0, 0)}

    for instruction in instructions:
        direction, distance = instruction[0], int(instruction[1:])
        for _ in range(distance):
            [x, y] = knots[0]  # Position of the head
            if direction == "U":
                y += 1
            elif direction == "D":
                y -= 1
            elif direction == "R":
                x += 1
            elif direction == "L":
                x -= 1
            else:
                raise ValueError(f"Unknown direction: {direction}")

            knots[0] = [x, y]

            for x in range(1, knot_count):
                # Check distance to previous knot
                if (
                    abs(knots[x][0] - knots[x - 1][0]) >= 2
                    or abs(knots[x][1] - knots[x - 1][1]) >= 2
                ):
                    knots[x][0] += sign(knots[x - 1][0] - knots[x][0])
                    knots[x][1] += sign(knots[x - 1][1] - knots[x][1])

            visited.add((knots[-1][0], knots[-1][1]))

    return len(visited)


lines = file.read().splitlines()

print(snake(lines, 2))
print(snake(lines, 10))
