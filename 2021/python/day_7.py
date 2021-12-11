def part_1():
    with open("inputs/day7", "r") as file:
        crab_positions = sorted(list(map(int, file.read().split(","))))
        return sum(
            map(
                lambda x: abs(x - crab_positions[len(crab_positions) // 2]),
                crab_positions,
            )
        )


def part_2():
    with open("inputs/day7", "r") as file:
        crab_positions = list(map(int, file.read().split(",")))
        result = float("inf")

        ap_sum = lambda x: x * (x + 1) // 2
        for num in crab_positions:
            result = min(
                result, sum(map(lambda x: ap_sum(abs(x - num)), crab_positions))
            )

        return result


print(part_1())
print(part_2())
