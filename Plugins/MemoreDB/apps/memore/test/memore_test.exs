defmodule MemoreTest do
  use ExUnit.Case, async: true
  doctest Memore

  setup context do
    _ = start_supervised!({Registry, name: context.test})
    %{registry: context.test}
  end

  test "spawn memores", %{registry: registry} do
    assert Registry.lookup(registry, "shopping") == :error

    Registry.create(registry, "shopping")
    assert {:ok, memore} = Registry.lookup(registry, "shopping")

    Memore.put(memore, "milk", 1)
    assert Memore.get(memore, "milk") == 1
  end

  test "remove memores on exit", %{registry: registry} do
    Registry.create(registry, "shopping")
    {:ok, memore} = Registry.lookup(registry, "shopping")
    Agent.stop(memore)

    _ = Registry.create(registry, "bogus")
    assert Registry.lookup(registry, "shopping") == :error
  end

  test "removes bucket on crash", %{registry: registry} do
    Registry.create(registry, "shopping")
    {:ok, memore} = Registry.lookup(registry, "shopping")

    # Stop the memore with non-normal reason
    Agent.stop(memore, :shutdown)

    _ = Registry.create(registry, "bogus")
    assert Registry.lookup(registry, "shopping") == :error
  end

  test "are temporary workers" do
    assert Supervisor.child_spec(Memore, []).restart == :temporary
  end

  test "memore can crash at any time", %{registry: registry} do
    Registry.create(registry, "shopping")
    {:ok, memore} = Registry.lookup(registry, "shopping")

    # Simulate a memore crash by explicitly and synchronously shuttibg it down
    Agent.stop(memore, :shutdown)
    catch_exit Memore.put(memore, "milk", 3)
  end
end
