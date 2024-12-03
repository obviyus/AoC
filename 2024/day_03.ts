const file = Bun.file("inputs/day_03.txt");
const input = await file.text();

type Instruction = {
	pos: number | undefined;
	type: "do" | "don't";
};

const PATTERNS = {
	multiply: /mul\((\d+),(\d+)\)/g,
	do: /do\(\)/g,
	dont: /don't\(\)/g,
} as const;

function partOne(input: string): number {
	return Array.from(input.matchAll(PATTERNS.multiply)).reduce((sum, match) => {
		const [a, b] = [match[1], match[2]].map(Number);
		return sum + a * b;
	}, 0);
}

function partTwo(input: string): number {
	let result = 0;
	let canMultiply = true;
	let lastIndex = 0;

	for (const mulMatch of input.matchAll(PATTERNS.multiply)) {
		const beforeMul = input.slice(lastIndex, mulMatch.index);

		// Get instructions between multiplications
		const instructions: Instruction[] = [
			...Array.from(beforeMul.matchAll(PATTERNS.do)).map((m) => ({
				pos: m.index,
				type: "do" as const,
			})),
			...Array.from(beforeMul.matchAll(PATTERNS.dont)).map((m) => ({
				pos: m.index,
				type: "don't" as const,
			})),
		].sort((a, b) => (a.pos ?? 0) - (b.pos ?? 0));

		// Update state by last instruction
		if (instructions.length > 0) {
			canMultiply = instructions[instructions.length - 1].type === "do";
		}

		if (canMultiply) {
			const [a, b] = [mulMatch[1], mulMatch[2]].map(Number);
			result += a * b;
		}

		if (mulMatch.index !== undefined) {
			lastIndex = mulMatch.index + mulMatch[0].length;
		}
	}

	return result;
}

const sampleInput =
	"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

console.log(`Part 1: ${partOne(input)}`);
console.log(`Part 2: ${partTwo(input)}`);
