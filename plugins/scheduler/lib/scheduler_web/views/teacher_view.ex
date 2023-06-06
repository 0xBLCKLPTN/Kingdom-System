defmodule SchedulerWeb.TeacherView do
  use SchedulerWeb, :view
  alias SchedulerWeb.TeacherView

  def render("index.json", %{teachers: teachers}) do
    %{data: render_many(teachers, TeacherView, "teacher.json")}
  end

  def render("show.json", %{teacher: teacher}) do
    %{data: render_one(teacher, TeacherView, "teacher.json")}
  end

  def render("teacher.json", %{teacher: teacher}) do
    %{id: teacher.id,
      first_name: teacher.first_name,
      middle_name: teacher.middle_name,
      last_name: teacher.last_name}
  end
end
