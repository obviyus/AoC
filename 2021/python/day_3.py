from collections import Counter


def part_1():
    eps, gam = "", ""
    with open("../inputs/day3", "r") as file:
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


def count_at_index(cache, index):
    result = {"0": 0, "1": 0}
    for num in cache:
        result[num[index]] += 1
    return result


def most_common_at_index(cache, index):
    r = count_at_index(cache, index)
    return "1" if r["1"] >= r["0"] else "0"


def least_common_at_index(cache, index):
    r = count_at_index(cache, index)
    return "0" if r["1"] >= r["0"] else "1"


def filter_until_one(cache, common):
    index = 0
    while len(cache) > 1:
        value = (
            most_common_at_index(cache, index)
            if common
            else least_common_at_index(cache, index)
        )
        cache = list(
            filter(
                lambda x: x[index] == value,
                cache,
            )
        )
        index += 1
    return cache[0]


def part_2():
    with open("../inputs/day3", "r") as file:
        cache = []

        for line in file:
            cache.append(line.strip())

    return int(filter_until_one(cache, True), 2) * int(
        filter_until_one(cache, False), 2
    )


print(part_1())
print(part_2())
