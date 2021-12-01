def part_1():
    prev, count = None, 0

    with open('input', 'r') as file:
        try:
            while (line := int(file.readline().rstrip())):
                if prev and line > prev:
                    count += 1
                prev = line
        except(ValueError):
            pass
    return count

if __name__ == '__main__':
    import timeit
    print(timeit.timeit("part_1()", globals=locals(), number=1000))