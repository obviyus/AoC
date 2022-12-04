# Part 1
file = open("../inputs/day_04.txt", "r")

full_overlap_count = 0
any_overlap_count = 0
for line in file:
    e1, e2 = line.split(",")

    e1s1, e1s2 = e1.split("-")
    e2s1, e2s2 = e2.split("-")
    # Check if a range fully contains another range
    if int(e1s1) <= int(e2s1) and int(e2s2) <= int(e1s2):
        full_overlap_count += 1
    elif int(e2s1) <= int(e1s1) and int(e1s2) <= int(e2s2):
        full_overlap_count += 1

    # Check if there's ANY overlap
    if int(e1s1) <= int(e2s1) <= int(e1s2):
        any_overlap_count += 1
    elif int(e2s1) <= int(e1s1) <= int(e2s2):
        any_overlap_count += 1

print(f"Part 1: {full_overlap_count}")
print(f"Part 2: {any_overlap_count}")
