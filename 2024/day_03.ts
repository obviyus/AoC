const file = Bun.file("inputs/day_03.txt");
const input = await file.text();

function partOne(input: string): number {
	const re = /mul\((\d+),(\d+)\)/g;
	const matches = input.match(re);

	if (!matches) return 0;

	let result = 0;
	for (const match of matches) {
		const [a, b] = (match.match(/\d+/g) ?? []).map(Number);
		result += a * b;
	}

	return result;
}

function partTwo(input: string): number {
	const reMul = /mul\((\d+),(\d+)\)/g;
	const reDoInst = /do\(\)/g;
	const reDontInst = /don't\(\)/g;
	let result = 0;
	let canMultiply = true;

	let lastIndex = 0;
	let mulMatch: RegExpExecArray | null;

	while (true) {
		mulMatch = reMul.exec(input);
		if (mulMatch === null) break;

		// Check for any do()/don't() between last position and current mul
		const beforeMul = input.slice(lastIndex, mulMatch.index);
		const doInsts = [...beforeMul.matchAll(reDoInst)];
		const dontInsts = [...beforeMul.matchAll(reDontInst)];

		// Combine and sort all by their position
		const allInsts = [
			...doInsts.map((m) => ({ pos: m.index, type: "do" })),
			...dontInsts.map((m) => ({ pos: m.index, type: "don't" })),
		].sort((a, b) => (a.pos ?? 0) - (b.pos ?? 0));

		if (allInsts.length > 0) {
			// Update state on last instruction
			const lastInst = allInsts[allInsts.length - 1];
			canMultiply = lastInst.type === "do";
		}

		if (canMultiply) {
			const [a, b] = [mulMatch[1], mulMatch[2]].map(Number);
			result += a * b;
		}

		lastIndex = mulMatch.index + mulMatch[0].length;
	}

	return result;
}

const sampleInput =
	"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

console.log(`Part 1: ${partOne(input)}`);
console.log(`Part 2: ${partTwo(input)}`);
