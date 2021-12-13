def part_1():
    dots, fold_instructions = set(), []
    with open("inputs/day13", "r") as file:
        for line in file.read().splitlines():
            if line.startswith("fold along"):
                fold = line.split("=")
                fold_instructions.append((fold[0][-1], int(fold[1])))
            elif line != "":
                dots.add(tuple(map(int, line.split(","))))

    axis, position = fold_instructions[0]
    if axis == "x":
        dots = set(
            (position - abs(x - position), y) if x > position else (x, y)
            for (x, y) in dots
        )
    elif axis == "y":
        dots = set(
            (x, position - abs(y - position)) if y > position else (x, y)
            for (x, y) in dots
        )

    return len(dots)


def part_2():
    dots, fold_instructions = set(), []
    with open("inputs/day13", "r") as file:
        for line in file.read().splitlines():
            if line.startswith("fold along"):
                fold = line.split("=")
                fold_instructions.append((fold[0][-1], int(fold[1])))
            elif line != "":
                dots.add(tuple(map(int, line.split(","))))

    for axis, position in fold_instructions:
        if axis == "x":
            dots = set(
                (position - abs(x - position), y) if x > position else (x, y)
                for (x, y) in dots
            )
        elif axis == "y":
            dots = set(
                (x, position - abs(y - position)) if y > position else (x, y)
                for (x, y) in dots
            )

    max_x = max(x for (x, y) in dots)
    max_y = max(y for (x, y) in dots)
    for y in range(max_y + 1):
        for x in range(max_x + 1):
            print("#" if (x, y) in dots else " ", end="")
        print()
    return len(dots)


print(part_1())
print(part_2())
