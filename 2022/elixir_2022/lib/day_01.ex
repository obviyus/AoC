file =
  File.read!("../inputs/day_01.txt")
  |> String.split("\n\n")
  |> Enum.map(fn line ->
    String.split(line, "\n", trim: true)
    |> Enum.map(&String.to_integer/1)
    |> Enum.sum()
  end)

# Part 1
file
|> Enum.max()
|> IO.inspect()

# Part 2
file
|> Enum.sort()
|> Enum.reverse()
|> Enum.take(3)
|> Enum.sum()
|> IO.inspect()
