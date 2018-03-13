defmodule Exvif.Mixfile do
  use Mix.Project

  def project do
    [
      app: :exvif,
      version: "0.1.0",
      elixir: "~> 1.5",
      start_permanent: Mix.env == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger, :httpoison, :timex]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:httpoison, "~> 1.0"},
      {:exml, "~> 0.1.1"},
      {:uuid, "~> 1.1"},
      {:timex, "~> 3.2.1"},
      {:dialyzex, "~> 1.1.0", only: :dev}
    ]
  end
end
