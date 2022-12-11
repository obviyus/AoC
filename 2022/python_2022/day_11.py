file = open("../inputs/day_11.txt")
text = file.read()

monkeys = (
    {}
)  # { monkey_index: { starting_items: [int], operation: { fx: int, value: int }, test: int, if_true: int, if_false: int } }

gcd = 1

lines = text.splitlines()
for line in lines:
    line = line.strip()
    if line.startswith("Monkey"):
        monkey_index = int(line.split(" ")[1].replace(":", ""))
        monkeys[monkey_index] = {}
    elif line.startswith("Starting items"):
        monkeys[monkey_index]["starting_items"] = [
            int(x) for x in line.split(": ")[1].split(", ")
        ]
    elif line.startswith("Operation"):
        operation = line.split(": ")[1].replace("new = old ", "").split(" ")
        monkeys[monkey_index]["operation"] = {
            "fx": 0 if operation[0] == "*" else 1,
            "value": -1 if operation[1] == "old" else int(operation[1]),
        }
    elif line.startswith("Test"):
        monkeys[monkey_index]["test"] = int(line.split(": ")[1].split(" ")[2])

        # Maintain a GCD to keep worry levels in check
        gcd *= monkeys[monkey_index]["test"]
    elif line.startswith("If true"):
        monkeys[monkey_index]["if_true"] = int(line.split(": ")[1].split(" ")[3])
    elif line.startswith("If false"):
        monkeys[monkey_index]["if_false"] = int(line.split(": ")[1].split(" ")[3])

inspect_count = [0] * len(monkeys)

for _ in range(10000):
    for monkey_index in range(len(monkeys)):
        monkey = monkeys[monkey_index]
        while monkey["starting_items"]:
            item = monkey["starting_items"].pop(0)
            if monkey["operation"]["fx"] == 0:
                item *= (
                    monkey["operation"]["value"]
                    if monkey["operation"]["value"] != -1
                    else item
                )
            else:
                item += (
                    monkey["operation"]["value"]
                    if monkey["operation"]["value"] != -1
                    else item
                )

            # item = item // 3
            if item % monkey["test"] == 0:
                monkeys[monkey["if_true"]]["starting_items"].append(item % gcd)
            else:
                monkeys[monkey["if_false"]]["starting_items"].append(item % gcd)
            inspect_count[monkey_index] += 1

inspect_count.sort(reverse=True)
print(f"Part 1: {inspect_count[0] * inspect_count[1]}")
