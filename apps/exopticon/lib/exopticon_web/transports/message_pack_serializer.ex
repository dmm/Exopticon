defmodule ExopticonWeb.Transports.MessagePackSerializer do
  @moduledoc false

  @behaviour Phoenix.Transports.Serializer

  alias Phoenix.Socket.Reply
  alias Phoenix.Socket.Message
  alias Phoenix.Socket.Broadcast

  def fastlane!(%Broadcast{} = msg) do
    {:socket_push, :binary, pack_data(%{
      topic: msg.topic,
      event: msg.event,
      payload: msg.payload
    })}
  end

  def encode!(%Reply{} = reply) do
    packed = pack_data(%{
      topic: reply.topic,
      event: "phx_reply",
      ref: reply.ref,
      payload: %{status: reply.status, response: reply.payload}
    })
    {:socket_push, :binary, packed}
  end

  def encode!(%Message{} = msg) do
    # We need to convert the Message struct into a plain map for MessagePack to work properly.
    # Alternatively we could have implemented the Enumerable behaviour. Pick your poison :)
    {:socket_push, :binary, pack_data(Map.from_struct msg)}
  end

  # messages received from the clients are still in json format;
  # for our use case clients are mostly passive listeners and made no sense
  # to optimize incoming traffic
    @doc """
  Decodes JSON String into `Phoenix.Socket.Message` struct.
  """
  def decode!(raw_message, _opts) do
    [join_ref, ref, topic, event, payload | _] = Poison.decode!(raw_message)

    %Phoenix.Socket.Message{
      topic: topic,
      event: event,
      payload: payload,
      ref: ref,
      join_ref: join_ref,
    }
  end

  defp pack_data(data) do
    MessagePack.pack!(data, enable_string: true)
  end
end
