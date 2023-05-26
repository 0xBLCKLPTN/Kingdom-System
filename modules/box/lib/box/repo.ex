defmodule Box.Repo do
  use Ecto.Repo,
    otp_app: :box,
    adapter: Ecto.Adapters.Postgres
end
