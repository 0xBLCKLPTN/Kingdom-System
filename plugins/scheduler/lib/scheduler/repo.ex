defmodule Scheduler.Repo do
  use Ecto.Repo,
    otp_app: :scheduler,
    adapter: Ecto.Adapters.Postgres
end
