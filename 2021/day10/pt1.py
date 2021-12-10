def part_1():
    opening = ["(", "{", "<", "["]
    closing = [")", "}", ">", "]"]
    illegal_points = {")": 3, "]": 57, "}": 1197, ">": 25137}

    stack, error_score = [], 0
    with open("2021/inputs/day10", "r") as file:
        for line in file.read().splitlines():
            for char in line:
                if char in opening:
                    stack.append(char)
                elif char in closing:
                    if len(stack) == 0:
                        continue
                    if opening.index(stack.pop()) != closing.index(char):
                        error_score += illegal_points[char]

    return error_score


print(part_1())
