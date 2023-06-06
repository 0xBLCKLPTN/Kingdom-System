defmodule SchedulerWeb.GroupView do
  use SchedulerWeb, :view
  alias SchedulerWeb.GroupView

  def render("index.json", %{groups: groups}) do
    %{data: render_many(groups, GroupView, "group.json")}
  end

  def render("show.json", %{group: group}) do
    %{data: render_one(group, GroupView, "group.json")}
  end

  def render("group.json", %{group: group}) do
    %{id: group.id,
      short_name: group.short_name,
      full_name: group.full_name,
      course: group.course,
      budget: group.budget}
  end
end
