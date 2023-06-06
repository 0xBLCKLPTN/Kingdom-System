defmodule Scheduler.Schedules.Schedule do
  use Ecto.Schema
  import Ecto.Changeset
  alias Scheduler.Groups.Group
  alias Scheduler.Lessons.Lesson
  alias Scheduler.Teachers.Teacher

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "schedules" do
    field :class_number, :integer
    field :lesson_number, :integer
    field :timestamp, :integer

    belongs_to :group, Group
    belongs_to :teacher, Teacher
    belongs_to :lesson, Lesson

    timestamps()
  end

  @doc false
  def changeset(schedule, attrs) do
    schedule
    |> cast(attrs, [:timestamp, :class_number, :lesson_number, :group_id, :teacher_id, :lesson_id])
    |> validate_required([:timestamp, :class_number, :lesson_number])
  end
end
