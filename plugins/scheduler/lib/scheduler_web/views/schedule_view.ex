defmodule SchedulerWeb.ScheduleView do
  use SchedulerWeb, :view
  alias SchedulerWeb.ScheduleView

  def render("index.json", %{schedules: schedules}) do
    %{data: render_many(schedules, ScheduleView, "schedule.json")}
  end

  def render("show.json", %{schedule: schedule}) do
    %{data: render_one(schedule, ScheduleView, "schedule.json")}
  end

  def render("schedule.json", %{schedule: schedule}) do
    %{id: schedule.id,
      timestamp: schedule.timestamp,
      class_number: schedule.class_number,
      lesson_number: schedule.lesson_number,
      lesson: render_one(schedule.lesson, __MODULE__, "lesson.json", as: :lesson),
      group: render_one(schedule.group, __MODULE__, "group.json", as: :group),
      teacher: render_one(schedule.teacher, __MODULE__, "teacher.json", as: :teacher)
    }
  end

  def render("teacher.json", %{teacher: teacher}) do
    %{
      id: teacher.id,
      first_name: teacher.first_name,
      middle_name: teacher.middle_name,
      last_name: teacher.last_name
    }
  end

  def render("lesson.json", %{lesson: lesson}) do
    %{
      id: lesson.id,
      short_name: lesson.short_name,
      full_name: lesson.full_name
    }
  end

  def render("group.json", %{group: group}) do
    %{
      id: group.id,
      short_name: group.short_name,
      full_name: group.full_name,
      course: group.course,
      budget: group.budget
    }
  end
end
