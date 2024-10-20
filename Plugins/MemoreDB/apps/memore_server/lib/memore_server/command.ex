defmodule MemoreServer.Command do
  @doc """
  Runs the given command.
  """
  def run(command)

  def run({:create, memore}) do
    Registry.create(Registry, memore)
    {:ok, "OK\r\n"}
  end

  def run({:get, memore, key}) do
    lookup(memore, fn pid ->
      value = Memore.get(pid, key)
      {:ok, "#{value}\r\nOK\r\n"}
    end)
  end

  def run({:put, memore, key, value}) do
    lookup(memore, fn pid ->
      Memore.put(pid, key, value)
      {:ok, "OK\r\n"}
    end)
  end

  def run({:delete, memore, key}) do
    lookup(memore, fn pid ->
      Memore.delete(pid, key)
      {:ok, "OK\r\n"}
    end)
  end

  def lookup(memore, callback) do
    case Registry.lookup(Registry, memore) do
      {:ok, pid} -> callback.(pid)
      :error -> {:error, :not_found}
    end
  end

  @doc ~S"""
  Parses the given `line` into a command.

  ## Examples

      iex> MemoreServer.Command.parse "CREATE shopping\r\n"
      {:ok, {:create, "shopping"}}

      iex> MemoreServer.Command.parse "CREATE  shopping  \r\n"
      {:ok, {:create, "shopping"}}

      iex> MemoreServer.Command.parse "PUT shopping milk 1\r\n"
      {:ok, {:put, "shopping", "milk", "1"}}

      iex> MemoreServer.Command.parse "GET shopping milk\r\n"
      {:ok, {:get, "shopping", "milk"}}

      iex> MemoreServer.Command.parse "DELETE shopping eggs\r\n"
      {:ok, {:delete, "shopping", "eggs"}}

  Unknown commands or commands with the wrong number of
  arguments return an error:

      iex> MemoreServer.Command.parse "UNKNOWN shopping eggs\r\n"
      {:error, :unknown_command}

      iex> MemoreServer.Command.parse "GET shopping\r\n"
      {:error, :unknown_command}

  """
  def parse(line) do
    case String.split(line) do
      ["CREATE", memore] -> {:ok, {:create, memore}}
      ["GET", memore, key] -> {:ok, {:get, memore, key}}
      ["PUT", memore, key, value] -> {:ok, {:put, memore, key, value}}
      ["DELETE", memore, key] -> {:ok, {:delete, memore, key}}
      _ ->  {:error, :unknown_command}
    end
  end
end
