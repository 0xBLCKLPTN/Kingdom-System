# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :box,
  ecto_repos: [Box.Repo],
  generators: [binary_id: true]

# Configures the endpoint
config :box, BoxWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "S8NoDuo4/0UkIePA8FBlz3Jak1XCpoatkvpkMQ2U1JfdTM5StQ0wbk4FcTFvay9D",
  render_errors: [view: BoxWeb.ErrorView, accepts: ~w(json), layout: false],
  pubsub_server: Box.PubSub,
  live_view: [signing_salt: "xFvA/PnZ"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
