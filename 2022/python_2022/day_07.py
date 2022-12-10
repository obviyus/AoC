from collections import defaultdict

file = open("../inputs/day_07.txt")

cwd = ["/"]
directories = defaultdict(int)

for line in file:
    line = line.strip()
    if line.startswith("$ cd"):
        command, argument = line.split(" ", 2)[1:]
        match command:
            case "cd":
                if argument == "..":
                    cwd.pop()
                elif argument == "/":
                    cwd = ["/"]
                else:
                    cwd.append(argument)
    elif not line.startswith("$"):
        if line.startswith("dir"):
            directories["".join(cwd) + line[3:]] = 0
        else:
            filesize, filename = line.split(" ", 2)
            directories["".join(cwd)] += int(filesize)
            for i in range(1, len(cwd)):
                directories["".join(cwd[:-i])] += int(filesize)

print(f"Part 1: {sum(v for v in directories.values() if v <= 100000)}")
print(
    f"Part 2: {min(v for v in directories.values() if v >= directories['/'] - 40000000)}"
)
