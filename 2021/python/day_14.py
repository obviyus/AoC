from collections import deque, Counter


def window(seq, n=2):
    it = iter(seq)
    win = deque((next(it, None) for _ in range(n)), maxlen=n)
    yield win
    append = win.append
    for e in it:
        append(e)
        yield win


def part_1():
    template, insertion_rules = "", {}
    with open("inputs/day14", "r") as file:
        for line in file.read().splitlines():
            if " -> " in line:
                match, insert = line.split(" -> ")
                insertion_rules[match] = insert
            elif line != "":
                template = line

    pairs, chars = Counter(x + y for (x, y) in window(template)), Counter(template)
    for i in range(10):
        for (c1, c2), count in pairs.copy().items():
            insertion = insertion_rules.get(c1 + c2)
            pairs[c1 + c2] -= count
            pairs[c1 + insertion] += count
            pairs[insertion + c2] += count
            chars[insertion] += count

    c = chars.most_common()
    return c[0][1] - c[-1][1]


def part_2():
    template, insertion_rules = "", {}
    with open("inputs/day14", "r") as file:
        for line in file.read().splitlines():
            if " -> " in line:
                match, insert = line.split(" -> ")
                insertion_rules[match] = insert
            elif line != "":
                template = line

    pairs, chars = Counter(x + y for (x, y) in window(template)), Counter(template)
    for i in range(40):
        for (c1, c2), count in pairs.copy().items():
            insertion = insertion_rules.get(c1 + c2)
            pairs[c1 + c2] -= count
            pairs[c1 + insertion] += count
            pairs[insertion + c2] += count
            chars[insertion] += count

    c = chars.most_common()
    return c[0][1] - c[-1][1]


print(part_1())
print(part_2())
