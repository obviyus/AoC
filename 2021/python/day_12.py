from collections import defaultdict


def traverse(current_node, connections, visited):
    if current_node == "end":
        return 1
    elif current_node.islower() and current_node in visited:
        return 0

    # Create a copy of the current set of visited nodes
    visited = visited.union({current_node})
    return sum(
        traverse(node, connections, visited) for node in connections[current_node]
    )


def part_1():
    connections = defaultdict(list)
    with open("inputs/day12", "r") as file:
        for line in file.read().splitlines():
            a, b = line.split("-")
            connections[a].append(b)
            connections[b].append(a)

    return traverse("start", connections, set())


def traverse_double(current_node, connections, visited, duplicate=None):
    if current_node == "end":
        return 1
    elif current_node == "start" and visited:
        return 0
    elif current_node.islower() and current_node in visited:
        if duplicate:
            return 0
        duplicate = current_node

    # Create a copy of the current set of visited nodes
    visited = visited.union({current_node})
    return sum(
        traverse_double(node, connections, visited, duplicate)
        for node in connections[current_node]
    )


def part_2():
    connections = defaultdict(list)
    with open("inputs/day12", "r") as file:
        for line in file.read().splitlines():
            a, b = line.split("-")
            connections[a].append(b)
            connections[b].append(a)

    return traverse_double("start", connections, set())


print(part_1())
print(part_2())
