def part_1():
    counter = 0
    with open("2021/inputs/day8", "r") as file:
        for line in file.read().splitlines():
            counter += len(
                list(
                    filter(
                        lambda x: len(x) in [2, 4, 3, 7],
                        line.split(" | ")[1].split(" "),
                    )
                )
            )
        return counter


print(part_1())
