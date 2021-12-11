import statistics
import math


def part_1():
    with open("inputs/day7", "r") as file:
        crab_positions = list(map(int, file.read().split(",")))
        median = statistics.median(crab_positions)

        return sum(
            map(
                lambda x: abs(x - median),
                crab_positions,
            )
        )


def part_2():
    with open("inputs/day7", "r") as file:
        crab_positions = list(map(int, file.read().split(",")))
        mean = statistics.mean(crab_positions)

        cost = lambda x: x * (x + 1) / 2
        return min(
            sum(
                map(cost, map(abs, map(lambda x: x - math.floor(mean), crab_positions)))
            ),
            sum(
                map(cost, map(abs, map(lambda x: x - math.ceil(mean), crab_positions)))
            ),
        )


print(part_1())
print(part_2())
