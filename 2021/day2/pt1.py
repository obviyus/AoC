def part_1():
    hor, ver = 0, 0
    with open('../inputs/day2', 'r') as file:
        for line in file:
            direction, distance = line.strip().split(' ')
            distance = int(distance)
            
            if direction == "forward":
                hor += distance
            elif direction == "up":
                ver -= distance
            elif direction == "down":
                ver += distance
    return hor * ver

print(part_1())