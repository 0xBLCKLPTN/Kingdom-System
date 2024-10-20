defmodule MemoreServerTest do
  use ExUnit.Case
  @moduletag :capture_log
  doctest MemoreServer

  setup do
    Application.stop(:memore)
    :ok = Application.start(:memore)
  end

  setup do
    opts = [:binary, packet: :line, active: false]
    {:ok, socket} = :gen_tcp.connect('localhost', 4040, opts)
    %{socket: socket}
  end

  test "server integration", %{socket: socket} do
    assert send_and_recv(socket, "UNKNOWN shopping\r\n") ==
            "UNKNOWN COMMMAND\r\n"
    assert send_and_recv(socket, "GET shopping eggs\r\n") ==
            "NOT FOUND\r\n"

    assert send_and_recv(socket, "CREATE shopping\r\n") ==
            "OK\r\n"

    assert send_and_recv(socket, "PUT shopping eggs 3\r\n") ==
            "OK\r\n"

    assert send_and_recv(socket, "GET shopping eggs\r\n") == "3\r\n"
    assert send_and_recv(socket, "") == "OK\r\n"

    assert send_and_recv(socket, "DELETE shopping eggs\r\n") ==
            "OK\r\n"

    # GET returns two lines
    assert send_and_recv(socket, "GET shopping eggs\r\n") == "\r\n"
    assert send_and_recv(socket, "") == "OK\r\n"
  end

  defp send_and_recv(socket, command) do
    :ok = :gen_tcp.send(socket, command)
    {:ok, data} = :gen_tcp.recv(socket, 0, 1000)
    data
  end

  test "greets the world" do
    assert MemoreServer.hello() == :world
  end
end
