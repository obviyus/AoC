def part_1():
    opening = ["(", "{", "<", "["]
    closing = [")", "}", ">", "]"]
    illegal_points = {")": 3, "]": 57, "}": 1197, ">": 25137}

    stack, error_score = [], 0
    with open("inputs/day10", "r") as file:
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


def part_2():
    opening = ["(", "{", "<", "["]
    closing = [")", "}", ">", "]"]
    auto_complete_points = {")": 1, "]": 2, "}": 3, ">": 4}

    auto_complete_scores = []
    with open("inputs/day10", "r") as file:
        for line in file.read().splitlines():
            corrupted, stack = False, []
            for char in line:
                if char in opening:
                    stack.append(char)
                elif char in closing:
                    if len(stack) == 0:
                        continue
                    if opening.index(stack.pop()) != closing.index(char):
                        corrupted = True
                        break

            if corrupted:
                continue
            current_score = 0
            for char in stack[::-1]:
                current_score = (current_score * 5) + auto_complete_points[
                    closing[opening.index(char)]
                ]
            auto_complete_scores.append(current_score)

    auto_complete_scores.sort()
    return auto_complete_scores[len(auto_complete_scores) // 2]


print(part_1())
print(part_2())
