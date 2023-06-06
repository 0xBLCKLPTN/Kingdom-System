defmodule Scheduler.Teachers.Teacher do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "teachers" do
    field :first_name, :string
    field :last_name, :string
    field :middle_name, :string

    timestamps()
  end

  @doc false
  def changeset(teacher, attrs) do
    teacher
    |> cast(attrs, [:first_name, :middle_name, :last_name])
    |> validate_required([:first_name, :middle_name, :last_name])
  end
end
