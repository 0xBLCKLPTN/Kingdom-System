defmodule Memore.MixProject do
  use Mix.Project

  def project do
    [
      app: :memore,
      build_path: "../../_build",
      config_path: "../../config/config.exs",
      deps_path: "../../deps",
      lockfile: "../../mix.lock",
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {Memore, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
      [
      ]

  end
end
