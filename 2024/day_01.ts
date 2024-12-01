const file = Bun.file("inputs/day_01.txt");
const input = await file.text();

type ParsedLists = {
	list1: number[];
	list2: number[];
};

function parseInput(input: string): ParsedLists {
	return input.split("\n").reduce(
		(acc, line) => {
			const [num1, num2] = line.split("   ").map(Number);
			acc.list1.push(num1);
			acc.list2.push(num2);
			return acc;
		},
		{ list1: [], list2: [] } as ParsedLists,
	);
}

function partOne(input: string): number {
	const { list1, list2 } = parseInput(input);

	const sorted1 = [...list1].sort((a, b) => a - b);
	const sorted2 = [...list2].sort((a, b) => a - b);

	return sorted1.reduce((sum, num, i) => sum + Math.abs(num - sorted2[i]), 0);
}

function partTwo(input: string): number {
	const { list1, list2 } = parseInput(input);

	const frequency = list2.reduce((map, num) => {
		map.set(num, (map.get(num) || 0) + 1);
		return map;
	}, new Map<number, number>());

	return list1.reduce(
		(score, num) => score + (frequency.get(num) || 0) * num,
		0,
	);
}

const sampleInput = `3   4
4   3
2   5
1   3
3   9
3   3`;

console.log(`Part 1: ${partOne(input)}`);
console.log(`Part 2: ${partTwo(input)}`);
