def part_1():
    counter = 0
    with open("inputs/day8", "r") as file:
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


def convert(input, output) -> int:
    output = list(map("".join, map(sorted, output)))

    # Map the unique values we know to be 1, 4, 7 and 8 by length of input
    input.sort(key=lambda x: len(x))
    one, four, seven, eight = input[0], input[2], input[1], input[9]
    zero, two, three, five, six, nine = "", "", "", "", "", ""

    top_right, bottom_right = "", ""
    for i in range(6, 9):
        # We get six if after removing all symbols of "1" gives us 5 remaning symbols
        if len(set(input[i]) - set(input[0])) == 5:
            six = input[i]
            top_right = (set(input[9]) - set(input[i])).pop()
            bottom_right = (set(one) - set(top_right)).pop()
            break

    for i in range(3, 6):
        if top_right in input[i] and bottom_right in input[i]:
            three = input[i]
        elif top_right in input[i]:
            two = input[i]
        else:
            five = input[i]

    for i in range(6, 9):
        if input[i] == six:
            continue
        else:
            if len(set(input[i]) - set(four)) == 2:
                nine = input[i]
            else:
                zero = input[i]

    mappings = list(
        map(
            "".join,
            map(sorted, [zero, one, two, three, four, five, six, seven, eight, nine]),
        ),
    )

    display = ""
    for digit in output:
        display += str(mappings.index(digit))

    return int(display)


def part_2():
    total = 0
    with open("inputs/day8", "r") as file:
        for line in file.read().splitlines():
            pair = line.split(" | ")
            total += convert(pair[0].split(" "), pair[1].split(" "))

        return total


print(part_1())
print(part_2())
