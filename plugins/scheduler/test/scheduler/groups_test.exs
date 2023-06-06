defmodule Scheduler.GroupsTest do
  use Scheduler.DataCase

  alias Scheduler.Groups

  describe "groups" do
    alias Scheduler.Groups.Group

    @valid_attrs %{budget: 42, course: 42, full_name: "some full_name", short_name: "some short_name"}
    @update_attrs %{budget: 43, course: 43, full_name: "some updated full_name", short_name: "some updated short_name"}
    @invalid_attrs %{budget: nil, course: nil, full_name: nil, short_name: nil}

    def group_fixture(attrs \\ %{}) do
      {:ok, group} =
        attrs
        |> Enum.into(@valid_attrs)
        |> Groups.create_group()

      group
    end

    test "list_groups/0 returns all groups" do
      group = group_fixture()
      assert Groups.list_groups() == [group]
    end

    test "get_group!/1 returns the group with given id" do
      group = group_fixture()
      assert Groups.get_group!(group.id) == group
    end

    test "create_group/1 with valid data creates a group" do
      assert {:ok, %Group{} = group} = Groups.create_group(@valid_attrs)
      assert group.budget == 42
      assert group.course == 42
      assert group.full_name == "some full_name"
      assert group.short_name == "some short_name"
    end

    test "create_group/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Groups.create_group(@invalid_attrs)
    end

    test "update_group/2 with valid data updates the group" do
      group = group_fixture()
      assert {:ok, %Group{} = group} = Groups.update_group(group, @update_attrs)
      assert group.budget == 43
      assert group.course == 43
      assert group.full_name == "some updated full_name"
      assert group.short_name == "some updated short_name"
    end

    test "update_group/2 with invalid data returns error changeset" do
      group = group_fixture()
      assert {:error, %Ecto.Changeset{}} = Groups.update_group(group, @invalid_attrs)
      assert group == Groups.get_group!(group.id)
    end

    test "delete_group/1 deletes the group" do
      group = group_fixture()
      assert {:ok, %Group{}} = Groups.delete_group(group)
      assert_raise Ecto.NoResultsError, fn -> Groups.get_group!(group.id) end
    end

    test "change_group/1 returns a group changeset" do
      group = group_fixture()
      assert %Ecto.Changeset{} = Groups.change_group(group)
    end
  end
end
