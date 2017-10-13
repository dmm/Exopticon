defmodule Exopticon.CameraSupervisor do
  use Supervisor

  def start_link do
    Supervisor.start_link(__MODULE__, [], name: __MODULE__)
  end

  def init(_) do
    children = [
      #      worker(Exopticon.CapturePort, [], restart: :permanent)
    ]

    supervise(children, strategy: :one_for_one)
  end

  def start_all_cameras([]) do
  end

  def start_all_cameras(cameras) do
    [cam | tail] = cameras
    Supervisor.start_child(Exopticon.CameraSupervisor, Exopticon.CapturePort.child_spec(cam))
    start_all_cameras(tail)
  end
end
