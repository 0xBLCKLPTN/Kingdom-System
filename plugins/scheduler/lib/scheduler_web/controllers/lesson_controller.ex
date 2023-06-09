defmodule SchedulerWeb.LessonController do
  use SchedulerWeb, :controller

  alias Scheduler.Lessons
  alias Scheduler.Lessons.Lesson

  action_fallback SchedulerWeb.FallbackController

  def index(conn, _params) do
    lessons = Lessons.list_lessons()
    render(conn, "index.json", lessons: lessons)
  end

  def create(conn, %{"lesson" => lesson_params}) do
    with {:ok, %Lesson{} = lesson} <- Lessons.create_lesson(lesson_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", Routes.lesson_path(conn, :show, lesson))
      |> render("show.json", lesson: lesson)
    end
  end

  def show(conn, %{"id" => id}) do
    lesson = Lessons.get_lesson!(id)
    render(conn, "show.json", lesson: lesson)
  end

  def update(conn, %{"id" => id, "lesson" => lesson_params}) do
    lesson = Lessons.get_lesson!(id)

    with {:ok, %Lesson{} = lesson} <- Lessons.update_lesson(lesson, lesson_params) do
      render(conn, "show.json", lesson: lesson)
    end
  end

  def delete(conn, %{"id" => id}) do
    lesson = Lessons.get_lesson!(id)

    with {:ok, %Lesson{}} <- Lessons.delete_lesson(lesson) do
      send_resp(conn, :no_content, "")
    end
  end
end
