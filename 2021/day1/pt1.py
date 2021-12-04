def part_1():
    prev, count = None, 0

    with open('../inputs/day1', 'r') as file:
        for line in file:
            line = int(line)
            
            if prev and line > prev:
                count += 1
            prev = line
    return count

if __name__ == '__main__':
    import timeit
    print(timeit.timeit("part_1()", globals=locals(), number=1000))