file = open("../inputs/day_10.txt")

cycles = [1]

for index, line in enumerate(file):
    line = line.strip()

    if line == "noop":
        cycles.append(cycles[-1])
    elif line.startswith("addx"):
        value = int(line.split()[1])
        cycles.extend([cycles[-1], cycles[-1] + value])

total = cycles[20 - 1] * 20
total += cycles[60 - 1] * 60
total += cycles[100 - 1] * 100
total += cycles[140 - 1] * 140
total += cycles[180 - 1] * 180
total += cycles[220 - 1] * 220

print(total)

# Part 2
for x in range(len(cycles)):
    if abs(cycles[x] - (x % 40)) < 2:
        print("#", end="")
    else:
        print(".", end="")
    if x % 40 == 0:
        print("")
