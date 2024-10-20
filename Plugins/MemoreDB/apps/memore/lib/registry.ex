defmodule Registry do
  use GenServer

  ## Client API

  @doc """
    Starts the registry
  """

  def start_link(opts) do

    # 1. pass the name to GenServer's init
    server = Keyword.fetch!(opts, :name)
    GenServer.start_link(__MODULE__, server, opts)
  end

  @doc """
    Looks up the memore pid for `name` stored in `server`

    Returns `{:ok, pid}` if the bucket exists, `:error` otherwise.
  """
  def lookup(server, name) do

    # 2. Lookup os now done directrly in ETS, without accessing the server.
    case :ets.lookup(server, name) do
      [{^name, pid}] -> {:ok, pid}
      [] -> :error
    end
  end

  @doc """
    Ensures there is a memore associated with given `name` in `server`.
  """
  def create(server, name) do
    GenServer.call(server, {:create, name})
  end

  ## Server callbacks

  @impl true
  def init(table) do
    # 3. We have replaced the names map by the ETS table.
    names = :ets.new(table, [:named_table, read_concurrency: true])
    refs = %{}
    {:ok, {names, refs}}
  end

  # 4. The previous handle_call callback for lookup was removed

  @impl true
  def handle_call({:create, name}, _from, {names, refs}) do

    # 5. Read and write to the ETS table insead of the map
    case lookup(names, name) do
      {:ok, pid} ->
        {:reply, pid, {names,refs}}
      :error ->
        {:ok, pid} = DynamicSupervisor.start_child(MemoreSupervisor, Memore)
        ref = Process.monitor(pid)
        refs = Map.put(refs, ref, name)
        :ets.insert(names, {name, pid})
        {:reply, pid, {names, refs}}
    end
  end

  @impl true
  def handle_info({:DOWN, ref, :process, _pid, _reason}, {names,refs}) do
    # 6. Delete from the ETS table instead of the map
    {name, refs} = Map.pop(refs, ref)
    :ets.delete(names, name)
    {:noreply, {names, refs}}
  end

  @impl true
  def handle_info(_msg, state) do
    {:noreply, state}
  end

end
