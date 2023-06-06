defmodule SchedulerWeb.GroupControllerTest do
  use SchedulerWeb.ConnCase

  alias Scheduler.Groups
  alias Scheduler.Groups.Group

  @create_attrs %{
    budget: 42,
    course: 42,
    full_name: "some full_name",
    short_name: "some short_name"
  }
  @update_attrs %{
    budget: 43,
    course: 43,
    full_name: "some updated full_name",
    short_name: "some updated short_name"
  }
  @invalid_attrs %{budget: nil, course: nil, full_name: nil, short_name: nil}

  def fixture(:group) do
    {:ok, group} = Groups.create_group(@create_attrs)
    group
  end

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all groups", %{conn: conn} do
      conn = get(conn, Routes.group_path(conn, :index))
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create group" do
    test "renders group when data is valid", %{conn: conn} do
      conn = post(conn, Routes.group_path(conn, :create), group: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, Routes.group_path(conn, :show, id))

      assert %{
               "id" => id,
               "budget" => 42,
               "course" => 42,
               "full_name" => "some full_name",
               "short_name" => "some short_name"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, Routes.group_path(conn, :create), group: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update group" do
    setup [:create_group]

    test "renders group when data is valid", %{conn: conn, group: %Group{id: id} = group} do
      conn = put(conn, Routes.group_path(conn, :update, group), group: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, Routes.group_path(conn, :show, id))

      assert %{
               "id" => id,
               "budget" => 43,
               "course" => 43,
               "full_name" => "some updated full_name",
               "short_name" => "some updated short_name"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, group: group} do
      conn = put(conn, Routes.group_path(conn, :update, group), group: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete group" do
    setup [:create_group]

    test "deletes chosen group", %{conn: conn, group: group} do
      conn = delete(conn, Routes.group_path(conn, :delete, group))
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, Routes.group_path(conn, :show, group))
      end
    end
  end

  defp create_group(_) do
    group = fixture(:group)
    %{group: group}
  end
end
