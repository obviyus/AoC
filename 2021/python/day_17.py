import re


def part_1():
    """
    For this part, velocity of x is irrelevant.

    To maximize height, only consider the velocity in Y. If launched upwards,
    due to gravity at some point the position of Y will be 0. Since our
    simulation only operates in discrete steps of 1 second (i.e. no decimal seconds)
    it means that the velocity at Y = 0 *must* be the lowest value of the
    target region. So after one second, the probe reaches last line of the target region.

    Since acceleration on Y = -1, the final velocity (at y = 0) will also be our launch velocity.

    At the highest point, the velocity will be 0. This lets us find the highest point
    by simply calculating the sum of the A.P. (-1 * lowest_y)..0.

    => n * (n + 1) / 2
    where n = lowest_y - 1
    """
    with open("inputs/day17", "r") as file:
        for line in file:
            _, _, y1, _ = map(int, re.findall(r"-?\d+", line.strip()))

    lowest_y = (-1 * y1) - 1
    return lowest_y * (lowest_y + 1) // 2


def part_2():
    with open("inputs/day17", "r") as file:
        for line in file:
            x1, x2, y1, y2 = map(int, re.findall(r"-?\d+", line.strip()))

    def simulate(velocity_x, velocity_y, x=0, y=0):
        if x > x2 or y < y1:
            return 0
        if x >= x1 and y <= y2:
            return 1
        return simulate(
            velocity_x - (velocity_x > 0),
            velocity_y - 1,
            x + velocity_x,
            y + velocity_y,
        )

    return sum(simulate(x, y) for x in range(1, 1 + x2) for y in range(y1, -y1))


print(part_1())
print(part_2())
