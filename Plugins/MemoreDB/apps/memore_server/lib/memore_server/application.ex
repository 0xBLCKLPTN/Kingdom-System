defmodule MemoreServer.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    port = String.to_integer(System.get_env("PORT") || "4040")
    children = [
      # Starts a worker by calling: MemoreServer.Worker.start_link(arg)
      # {MemoreServer.Worker, arg}
      {Task.Supervisor, name: MemoreServer.TaskSupervisor},
      Supervisor.child_spec({Task, fn -> MemoreServer.accept(port) end}, restart: :permanent)
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: MemoreServer.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
