from collections import Counter

def part_1():
    DAYS = 256
    with open('inputs/day6', 'r') as file:
        fishes = Counter(list(map(int, file.read().split(','))))
        for _ in range(DAYS):
            copy_fishes = Counter()
            for fish, count in fishes.items():
                if fish == 0:
                    copy_fishes[6] += count
                    copy_fishes[8] += count
                else:
                    copy_fishes[fish - 1] += count 
            fishes = copy_fishes
        return sum(fishes.values())

print(part_1())