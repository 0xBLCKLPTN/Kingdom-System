defmodule Memore do
  use Agent, restart: :temporary
  use Application
  @moduledoc """
  Documentation for Memore.
  """

  @impl true
  def start(_type, _args) do
    Superwizard.start_link(name: Superwizard)
  end

  @doc """
    Starts a new memore instance.
  """
  def start_link(_opts) do
    Agent.start_link(fn -> %{} end)
  end

  @doc """
    Get a value from the 'memore' by `key`
  """
  def get(memore, key) do
      Agent.get(memore, &Map.get(&1, key))
  end

  @doc """
    Pus the `value` for the giving `key` in the `memore`.
  """
  def put(memore, key, value) do
    Agent.update(memore, &Map.put(&1, key, value))
  end

  @doc """
  Deletes `key` from `bucket`.

  Returns the current value of `key`, if `key` exists.
  """
  def delete(memore, key) do
    Agent.get_and_update(memore, fn dict -> Map.pop(dict, key) end)
  end

end
