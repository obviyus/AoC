const file = Bun.file("inputs/day_02.txt");
const input = await file.text();

function parseInput(input: string): number[][] {
	return input.split("\n").reduce((acc, line) => {
		const nums: number[] = line.split(" ").map(Number);
		acc.push(nums);
		return acc;
	}, [] as number[][]);
}

function isReportSafe(report: number[]): boolean {
	const uniqueList = [...new Set(report)];
	const ascending = uniqueList.toSorted((a, b) => a - b);
	const descending = uniqueList.toSorted((a, b) => b - a);

	if (
		report.join(",") !== ascending.join(",") &&
		report.join(",") !== descending.join(",")
	) {
		return false;
	}

	for (let i = 1; i < report.length; i++) {
		const adjacentDifference = Math.abs(report[i] - report[i - 1]);
		if (adjacentDifference < 1 || adjacentDifference > 3) {
			return false;
		}
	}

	return true;
}

function partOne(input: string): number {
	const list = parseInput(input);

	return list.reduce((count, report) => {
		if (isReportSafe(report)) {
			return count + 1;
		}
		return count;
	}, 0);
}

function partTwo(input: string): number {
	const list = parseInput(input);

	return list.reduce((count, report) => {
		let newCount = count;
		if (isReportSafe(report)) {
			newCount++;
		} else {
			for (let i = 0; i < report.length; i++) {
				const newReport = [...report];
				newReport.splice(i, 1);
				if (isReportSafe(newReport)) {
					newCount++;
					break;
				}
			}
		}
		return newCount;
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
