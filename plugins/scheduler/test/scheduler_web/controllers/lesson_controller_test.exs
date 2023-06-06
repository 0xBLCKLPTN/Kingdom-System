defmodule SchedulerWeb.LessonControllerTest do
  use SchedulerWeb.ConnCase

  alias Scheduler.Lessons
  alias Scheduler.Lessons.Lesson

  @create_attrs %{
    full_name: "some full_name",
    short_name: "some short_name"
  }
  @update_attrs %{
    full_name: "some updated full_name",
    short_name: "some updated short_name"
  }
  @invalid_attrs %{full_name: nil, short_name: nil}

  def fixture(:lesson) do
    {:ok, lesson} = Lessons.create_lesson(@create_attrs)
    lesson
  end

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all lessons", %{conn: conn} do
      conn = get(conn, Routes.lesson_path(conn, :index))
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create lesson" do
    test "renders lesson when data is valid", %{conn: conn} do
      conn = post(conn, Routes.lesson_path(conn, :create), lesson: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, Routes.lesson_path(conn, :show, id))

      assert %{
               "id" => id,
               "full_name" => "some full_name",
               "short_name" => "some short_name"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, Routes.lesson_path(conn, :create), lesson: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update lesson" do
    setup [:create_lesson]

    test "renders lesson when data is valid", %{conn: conn, lesson: %Lesson{id: id} = lesson} do
      conn = put(conn, Routes.lesson_path(conn, :update, lesson), lesson: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, Routes.lesson_path(conn, :show, id))

      assert %{
               "id" => id,
               "full_name" => "some updated full_name",
               "short_name" => "some updated short_name"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, lesson: lesson} do
      conn = put(conn, Routes.lesson_path(conn, :update, lesson), lesson: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete lesson" do
    setup [:create_lesson]

    test "deletes chosen lesson", %{conn: conn, lesson: lesson} do
      conn = delete(conn, Routes.lesson_path(conn, :delete, lesson))
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, Routes.lesson_path(conn, :show, lesson))
      end
    end
  end

  defp create_lesson(_) do
    lesson = fixture(:lesson)
    %{lesson: lesson}
  end
end
