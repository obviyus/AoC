from collections import Counter

def part_1():
    eps, gam = '', ''
    with open('../inputs/day3', 'r') as file:
        results = []

        for line in file:
            if not results:
                results = [Counter() for _ in range(len(line))]
            for i, bit in enumerate(line.strip()):
                results[i][bit] += 1
        
        for counter in results:
            if counter:
                r = counter.most_common()
                gam += r[0][0]
                eps += r[-1][0]
    print(gam, eps)
    return int(gam, 2) * int(eps, 2)

print(part_1())