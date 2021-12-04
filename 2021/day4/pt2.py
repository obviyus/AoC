from heapq import *

def check_if_present(board, number):
    for i, row in enumerate(board):
        for j, val in enumerate(row):
            if val == number:
                return (i, j)
    return False


def sum_of_unmarked(board, marked):
    unmarked_sum = 0
    for i, row in enumerate(marked):
        for j, val in enumerate(row):
            if val == 0:
                unmarked_sum += board[i][j]
    return unmarked_sum


def is_solved(marked):
    for row in marked:
        if all(row) == 1:
            return True
    for col in zip(*marked):
        if all(col) == 1:
            return True
    return False


def part_2():
    with open('/home/obviyus/Desktop/aoc/2021/inputs/day4', 'r') as file:
        bingo_numbers = list(map(int, file.__next__().strip().split(',')))

        results, current_board, current_marked = [], [], [] # (solved_in, sum_of_unmarked, final_number)
        for line in file:
            if line.strip():
                current_board.append(list(map(int, line.strip().split())))
            else:
                time_to_solve = 0
                current_marked = [[0] * 5 for _ in range(5)]
                for val in bingo_numbers:
                    if check_if_present(current_board, val):
                        current_marked[check_if_present(current_board, val)[0]][check_if_present(current_board, val)[1]] = 1
                        if is_solved(current_marked):
                            heappush(results, (time_to_solve * -1, sum_of_unmarked(current_board, current_marked), val))
                            break
                    time_to_solve += 1
                current_board, current_marked = [], []
        
    answer = heappop(results)
    print(answer)
    return answer[1] * answer[2]

print(part_2())