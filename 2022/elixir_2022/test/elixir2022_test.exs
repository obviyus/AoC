defmodule Elixir2022Test do
  use ExUnit.Case
  doctest Elixir2022

  test "greets the world" do
    assert Elixir2022.hello() == :world
  end
end
