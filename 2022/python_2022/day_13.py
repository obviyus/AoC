from functools import cmp_to_key

file = open("../inputs/day_13.txt")
text = file.read()


def compare(left, right):
    match left, right:
        case int(), int():
            return left - right
        case int(), list():
            return compare([left], right)
        case list(), int():
            return compare(left, [right])
        case list(), list():
            for i in range(min(len(left), len(right))):
                result = compare(left[i], right[i])
                if result != 0:
                    return result
            return len(left) - len(right)


count = 0
for i, block in enumerate(text.split("\n\n")):
    block = block.split("\n")
    left, right = block[0], block[1]

    left = eval(left)
    right = eval(right)

    if compare(left, right) <= 0:
        count += i + 1

print(count)

# Part 2
custom_compare = cmp_to_key(compare)

lines = list(map(lambda y: eval(y), filter(lambda x: x, text.split("\n"))))
lines.extend([[[2]], [[6]]])

lines.sort(key=custom_compare)

p1 = lines.index([[2]])
p2 = lines.index([[6]])

print((p1 + 1) * (p2 + 1))
