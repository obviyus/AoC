file = open("../inputs/day_06.txt")
message = file.read()

distinct_count_part_1 = 4
distinct_count_part_2 = 14

# Part 1
for i in range(distinct_count_part_1, len(message)):
    packet = message[i - distinct_count_part_1: i]
    if distinct_count_part_1 == len(set(packet)):
        print(packet)
        print(i)
        break

# Part 2
for i in range(distinct_count_part_2, len(message)):
    packet = message[i - distinct_count_part_2: i]
    if distinct_count_part_2 == len(set(packet)):
        print(i)
        break
