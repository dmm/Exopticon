defmodule ExopticonWeb.PlaybackChannel do
  use ExopticonWeb, :channel

  def join("playback:lobby", payload, socket) do
    if authorized?(payload) do
      {:ok, socket}
    else
      {:error, %{reason: "unauthorized"}}
    end
  end

  def join("playback:" <> params, _payload, socket) do
    [_, file_id, offset] = String.split(params, ",") |> Enum.map(&String.to_integer/1)
    file = Exopticon.Repo.get!(Exopticon.Video.File, file_id)
    Exopticon.PlaybackSupervisor.start_playback({"playback:" <> params, file, offset})

    {:ok, socket}
  end

  # Channels can be used in a request/response fashion
  # by sending replies to requests from the client
  def handle_in("ping", payload, socket) do
    {:reply, {:ok, payload}, socket}
  end

  # It is also common to receive messages from the client and
  # broadcast to everyone in the current topic (playback:lobby).
  def handle_in("shout", payload, socket) do
    broadcast(socket, "shout", payload)
    {:noreply, socket}
  end

  # Add authorization logic here as required.
  defp authorized?(_payload) do
    true
  end
end
