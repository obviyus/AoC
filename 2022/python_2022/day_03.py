def common_char_finder(words: list) -> str:
    word_set = set(words[0])
    for word in words[1:]:
        word_set = word_set.intersection(set(word))

    return "".join(word_set)


def char_to_priority(char: str) -> int:
    if char.islower():
        return ord(char) - 96
    else:
        return ord(char) - 38


# Part 1
file = open("../inputs/day_03.txt", "r")

priorities = []
for line in file:
    common_char = "".join(
        common_char_finder(
            [line.strip()[: len(line) // 2], line.strip()[len(line) // 2 :]]
        )
    )
    priorities.append(char_to_priority(common_char))

print(f"Part 1: {sum(priorities)}")

# Part 2
file = open("../inputs/day_03.txt", "r")
priorities = []

# Check common chars by groups of 3 lines
lines = file.readlines()
for i in range(0, len(lines), 3):
    common_char = "".join(
        common_char_finder(
            [
                lines[i].strip(),
                lines[i + 1].strip(),
                lines[i + 2].strip(),
            ]
        )
    )
    priorities.append(char_to_priority(common_char))

print(f"Part 2: {sum(priorities)}")
