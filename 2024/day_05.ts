const file = Bun.file("inputs/day_05.txt");
const input = await file.text();

const parseInput = (
	input: string,
): { rules: number[][]; updates: number[][] } => {
	const [rules, updates] = input.split("\n\n");
	return {
		rules: rules.split("\n").map((line) => line.split("|").map(Number)),
		updates: updates.split("\n").map((line) => line.split(",").map(Number)),
	};
};

const isValidUpdate = (update: number[], rules: number[][]): boolean =>
	rules.every(
		([before, after]) =>
			!update.includes(before) ||
			!update.includes(after) ||
			update.indexOf(before) < update.indexOf(after),
	);

function partOne(input: string): number {
	const { rules, updates } = parseInput(input);

	const validUpdates = updates.filter((update) => isValidUpdate(update, rules));

	return validUpdates.reduce((sum, update) => {
		const middleIndex = Math.floor(update.length / 2);
		return sum + update[middleIndex];
	}, 0);
}

function partTwo(input: string): number {
	const { rules, updates } = parseInput(input);

	const invalidUpdates = updates.filter(
		(update) => !isValidUpdate(update, rules),
	);

	function sortUpdateByRules(a: number, b: number) {
		// 'b' should come before 'a' => 1 (swap)
		if (rules.some(([before, after]) => a === after && b === before)) {
			return 1;
		}
		// 'a' should come before 'b' => -1 (no swap)
		if (rules.some(([before, after]) => a === before && b === after)) {
			return -1;
		}
		return 0;
	}

	return invalidUpdates.reduce((sum, update) => {
		const sortedUpdate = update.sort(sortUpdateByRules);
		const middleIndex = Math.floor(sortedUpdate.length / 2);
		return sum + sortedUpdate[middleIndex];
	}, 0);
}

const sampleInput = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`;

console.log(`Part 1: ${partOne(input)}`);
console.log(`Part 2: ${partTwo(input)}`);
