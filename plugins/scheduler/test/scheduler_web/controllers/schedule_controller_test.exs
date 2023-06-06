defmodule SchedulerWeb.ScheduleControllerTest do
  use SchedulerWeb.ConnCase

  alias Scheduler.Schedules
  alias Scheduler.Schedules.Schedule

  @create_attrs %{
    class_number: 42,
    lesson_number: 42,
    timestamp: 42
  }
  @update_attrs %{
    class_number: 43,
    lesson_number: 43,
    timestamp: 43
  }
  @invalid_attrs %{class_number: nil, lesson_number: nil, timestamp: nil}

  def fixture(:schedule) do
    {:ok, schedule} = Schedules.create_schedule(@create_attrs)
    schedule
  end

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all schedules", %{conn: conn} do
      conn = get(conn, Routes.schedule_path(conn, :index))
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create schedule" do
    test "renders schedule when data is valid", %{conn: conn} do
      conn = post(conn, Routes.schedule_path(conn, :create), schedule: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, Routes.schedule_path(conn, :show, id))

      assert %{
               "id" => id,
               "class_number" => 42,
               "lesson_number" => 42,
               "timestamp" => 42
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, Routes.schedule_path(conn, :create), schedule: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update schedule" do
    setup [:create_schedule]

    test "renders schedule when data is valid", %{conn: conn, schedule: %Schedule{id: id} = schedule} do
      conn = put(conn, Routes.schedule_path(conn, :update, schedule), schedule: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, Routes.schedule_path(conn, :show, id))

      assert %{
               "id" => id,
               "class_number" => 43,
               "lesson_number" => 43,
               "timestamp" => 43
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, schedule: schedule} do
      conn = put(conn, Routes.schedule_path(conn, :update, schedule), schedule: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete schedule" do
    setup [:create_schedule]

    test "deletes chosen schedule", %{conn: conn, schedule: schedule} do
      conn = delete(conn, Routes.schedule_path(conn, :delete, schedule))
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, Routes.schedule_path(conn, :show, schedule))
      end
    end
  end

  defp create_schedule(_) do
    schedule = fixture(:schedule)
    %{schedule: schedule}
  end
end
