file = open("../inputs/day_02.txt", "r")

my_move_map = {
    "X": 0,
    0: "X",
    "Y": 1,
    1: "Y",
    "Z": 2,
    2: "Z",
}

opponent_move_map = {
    "A": 0,
    0: "A",
    "B": 1,
    1: "B",
    "C": 2,
    2: "C",
}


def compute_score(opponent_move, my_move) -> int:
    score = my_move_map[my_move] + 1

    winning_move = (opponent_move_map[opponent_move] + 1) % 3
    if my_move_map[my_move] == opponent_move_map[opponent_move]:
        return score + 3
    elif my_move_map[my_move] == winning_move:
        return score + 6
    else:
        return score


p1_score, p2_score = 0, 0
for line in file:
    opponent_move, my_move = line.strip().split(" ", 2)
    p1_score += compute_score(opponent_move, my_move)

    game_result = my_move
    if game_result == "X":
        my_move = (opponent_move_map[opponent_move] + 2) % 3
    elif game_result == "Y":
        my_move = opponent_move_map[opponent_move]
    else:
        my_move = (opponent_move_map[opponent_move] + 1) % 3

    p2_score += compute_score(opponent_move, my_move_map[my_move])

print(f"Part 1: {p1_score}")
print(f"Part 2: {p2_score}")
