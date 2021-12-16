import math

hex_to_bin_map = {
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "A": "1010",
    "B": "1011",
    "C": "1100",
    "D": "1101",
    "E": "1110",
    "F": "1111",
}


def read_raw(input, n):
    parsed = input[:n]
    del input[:n]

    return parsed


def read_n(input, n):
    parsed_num = int("".join(input[:n]), 2)
    del input[:n]

    return parsed_num


def packet_parser(binary_string):
    global version_sum

    version = read_n(binary_string, 3)
    type_id = read_n(binary_string, 3)
    version_sum += version

    if type_id == 4:
        literal_value = []
        while True:
            prefix, *value = read_raw(binary_string, 5)
            literal_value += value
            if prefix == "0":
                break

        return int("".join(literal_value), 2)

    length_type_id = read_n(binary_string, 1)
    decoded = []

    if length_type_id == 0:
        sub_packets = read_raw(binary_string, read_n(binary_string, 15))
        while sub_packets:
            decoded.append(packet_parser(sub_packets))
    else:
        decoded = [
            packet_parser(binary_string) for _ in range(read_n(binary_string, 11))
        ]

    if type_id == 0:
        return sum(decoded)
    elif type_id == 1:
        return math.prod(decoded)
    elif type_id == 2:
        return min(decoded)
    elif type_id == 3:
        return max(decoded)
    elif type_id == 5:
        return int(decoded[0] > decoded[1])
    elif type_id == 6:
        return int(decoded[0] < decoded[1])
    elif type_id == 7:
        return int(decoded[0] == decoded[1])


version_sum = 0


def part_1():
    binary_string = []
    with open("inputs/day16", "r") as file:
        for line in file.read().splitlines():
            for char in line:
                binary_string.extend(list(hex_to_bin_map[char]))

    packet_parser(binary_string)

    global version_sum
    return version_sum


def part_2():
    binary_string = []
    with open("inputs/day16", "r") as file:
        for line in file.read().splitlines():
            for char in line:
                binary_string.extend(list(hex_to_bin_map[char]))

    return packet_parser(binary_string)


print(part_1())
print(part_2())
