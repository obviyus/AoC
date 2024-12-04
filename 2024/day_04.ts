const file = Bun.file("inputs/day_04.txt");
const input = await file.text();

function partOne(input: string): number {
	const grid = input.split("\n").map((line) => line.split(""));
	const word = "XMAS";
	const directions = [
		[0, 1],
		[1, 0],
		[1, 1],
		[1, -1],
		[0, -1],
		[-1, 0],
		[-1, -1],
		[-1, 1],
	];
	let count = 0;

	function isValid(x: number, y: number): boolean {
		return x >= 0 && x < grid.length && y >= 0 && y < grid[0].length;
	}

	function search(x: number, y: number, dx: number, dy: number): boolean {
		for (let i = 0; i < word.length; i++) {
			const nx = x + i * dx;
			const ny = y + i * dy;
			if (!isValid(nx, ny) || grid[nx][ny] !== word[i]) {
				return false;
			}
		}
		return true;
	}

	for (let i = 0; i < grid.length; i++) {
		for (let j = 0; j < grid[0].length; j++) {
			for (const [dx, dy] of directions) {
				if (search(i, j, dx, dy)) {
					count++;
				}
			}
		}
	}

	return count;
}

function partTwo(input: string): number {
	const grid = input.split("\n").map((line) => line.split(""));
	let count = 0;

	function isValid(x: number, y: number): boolean {
		return x >= 0 && x < grid.length && y >= 0 && y < grid[0].length;
	}

	function checkMAS(
		x1: number,
		y1: number,
		x2: number,
		y2: number,
		x3: number,
		y3: number,
	): boolean {
		if (!isValid(x1, y1) || !isValid(x2, y2) || !isValid(x3, y3)) return false;
		const seq = grid[x1][y1] + grid[x2][y2] + grid[x3][y3];
		return seq === "MAS" || seq === "SAM";
	}

	for (let i = 0; i < grid.length; i++) {
		for (let j = 0; j < grid[0].length; j++) {
			if (grid[i][j] === "A") {
				// Check for both diagonals forming an X
				if (
					checkMAS(i - 1, j - 1, i, j, i + 1, j + 1) &&
					checkMAS(i - 1, j + 1, i, j, i + 1, j - 1)
				) {
					count++;
				}
			}
		}
	}

	return count;
}

const sampleInput = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`;

console.log(`Part 1: ${partOne(input)}`);
console.log(`Part 2: ${partTwo(input)}`);
