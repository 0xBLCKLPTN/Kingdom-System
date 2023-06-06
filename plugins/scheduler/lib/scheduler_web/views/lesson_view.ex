defmodule SchedulerWeb.LessonView do
  use SchedulerWeb, :view
  alias SchedulerWeb.LessonView

  def render("index.json", %{lessons: lessons}) do
    %{data: render_many(lessons, LessonView, "lesson.json")}
  end

  def render("show.json", %{lesson: lesson}) do
    %{data: render_one(lesson, LessonView, "lesson.json")}
  end

  def render("lesson.json", %{lesson: lesson}) do
    %{id: lesson.id,
      short_name: lesson.short_name,
      full_name: lesson.full_name}
  end
end
