const file = Bun.file("inputs/day_02.txt");
const input = await file.text();

function parseInput(input: string): number[][] {
	return input.split("\n").map((line) => line.split(" ").map(Number));
}

function isReportSafe(report: number[]): boolean {
	if (report.length < 2) return true;

	const increasing = report[1] > report[0];
	let prev = report[0];

	for (let i = 1; i < report.length; i++) {
		const diff = report[i] - prev;

		// Check both conditions at once:
		// 1. Direction consistency (diff should match initial direction)
		// 2. Difference magnitude (between 1 and 3)
		if (
			(increasing && diff <= 0) ||
			(!increasing && diff >= 0) ||
			Math.abs(diff) < 1 ||
			Math.abs(diff) > 3
		) {
			return false;
		}

		prev = report[i];
	}

	return true;
}

function partOne(input: string): number {
	return parseInput(input).filter(isReportSafe).length;
}

function partTwo(input: string): number {
	return parseInput(input).reduce((count, report) => {
		if (isReportSafe(report)) return count + 1;

		for (let i = 0; i < report.length; i++) {
			const newReport = [...report.slice(0, i), ...report.slice(i + 1)];
			if (isReportSafe(newReport)) {
				return count + 1;
			}
		}

		return count;
	}, 0);
}

const sampleInput = `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`;

console.log(`Part 1: ${partOne(input)}`);
console.log(`Part 2: ${partTwo(input)}`);
