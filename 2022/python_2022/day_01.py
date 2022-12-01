import heapq

file = open("../inputs/day_01.txt", "r")

highest_calories = []
current_group_sum = 0

for line in file:
    line = line.strip()
    if line == "":
        heapq.heappush(highest_calories, -current_group_sum)
        current_group_sum = 0
    else:
        current_group_sum += int(line)

# Print the highest calorie group
print(-highest_calories[0])

# print top 3 groups
print(-sum(heapq.nsmallest(3, highest_calories)))
