file =
  File.read!("../inputs/day_02.txt")
  |> String.split("\n")

myMoveMap = %{
  "X" => 0,
  "Y" => 1,
  "Z" => 2
}

opponentMoveMap = %{
  "A" => 0,
  "B" => 1,
  "C" => 2
}

defmodule Day02 do
  def computeScore(opponentMove, myMove) do
    winningMove = (opponentMove + 1) |> rem(3)
    score = myMove + 1

    cond do
      myMove == winningMove -> score + 6
      myMove == opponentMove -> score + 3
      true -> score
    end
  end
end

# Part 1
file
|> Enum.map(fn line ->
  [opponentMove, myMove] = String.split(line)
  Day02.computeScore(opponentMoveMap[opponentMove], myMoveMap[myMove])
end)
|> Enum.sum()
|> IO.inspect()

# Part 2
file
|> Enum.map(fn line ->
  [opponentMove, gameResult] = String.split(line)

  myMove =
    cond do
      gameResult == "X" -> (opponentMoveMap[opponentMove] + 2) |> rem(3)
      gameResult == "Y" -> opponentMoveMap[opponentMove]
      gameResult == "Z" -> (opponentMoveMap[opponentMove] + 1) |> rem(3)
    end

  Day02.computeScore(opponentMoveMap[opponentMove], myMove)
end)
|> Enum.sum()
|> IO.inspect()
