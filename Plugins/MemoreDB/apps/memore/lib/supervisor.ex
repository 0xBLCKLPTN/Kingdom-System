defmodule Superwizard do
  use Supervisor

  def start_link(opts) do
      Supervisor.start_link(__MODULE__, :ok, opts)
  end

  @impl true
  def init(:ok) do
    children = [
      {DynamicSupervisor, name: MemoreSupervisor, stategy: :one_for_one},
      {Registry, name: Registry},
    ]

    Supervisor.init(children, strategy: :one_for_all)
  end
end
