def part_1():
    with open('2021/inputs/day7', 'r') as file:
        crab_positions = sorted(list(map(int, file.read().split(','))))
        return sum(map(lambda x: abs(x - crab_positions[len(crab_positions) // 2]), crab_positions))

print(part_1())