defmodule Scheduler.Lessons.Lesson do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "lessons" do
    field :full_name, :string
    field :short_name, :string

    timestamps()
  end

  @doc false
  def changeset(lesson, attrs) do
    lesson
    |> cast(attrs, [:short_name, :full_name])
    |> validate_required([:short_name, :full_name])
  end
end
