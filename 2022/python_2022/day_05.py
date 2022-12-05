import re

file = open("../inputs/day_05.txt", "r")

input_stacks = []

# Parse inputs
for line in file:
    if not line.strip():
        break

    # Step every 4 characters to check for a crate
    for i in range(0, len(line), 4):
        if line[i] == "[":
            while len(input_stacks) < i // 4 + 1:
                input_stacks.append("")
            input_stacks[i // 4] += line[i + 1]

# Parse instructions into a list of tuples
instructions = []
for line in file:
    if line == "" or line.strip() == "":
        break

    instruction = re.findall(r"move (\d+) from (\d+) to (\d+)", line)
    instructions.append(
        (int(instruction[0][0]), int(instruction[0][1]), int(instruction[0][2]))
    )


def crane(stacks, new_model):
    for (move, from_stack, to_stack) in instructions:
        crates_to_move = stacks[from_stack - 1][:move]
        stacks[to_stack - 1] = (
            crates_to_move if new_model else crates_to_move[::-1]
        ) + stacks[to_stack - 1]

        stacks[from_stack - 1] = stacks[from_stack - 1][move:]

    return stacks


# Print character on first position of each stack
print(
    f'Part 1: {"".join([position[0] for position in crane(input_stacks.copy(), False)])}'
)
print(
    f'Part 2: {"".join([position[0] for position in crane(input_stacks.copy(), True)])}'
)
