defmodule Scheduler.Groups.Group do
  use Ecto.Schema
  import Ecto.Changeset

  @primary_key {:id, :binary_id, autogenerate: true}
  @foreign_key_type :binary_id
  schema "groups" do
    field :budget, :integer
    field :course, :integer
    field :full_name, :string
    field :short_name, :string

    timestamps()
  end

  @doc false
  def changeset(group, attrs) do
    group
    |> cast(attrs, [:short_name, :full_name, :course, :budget])
    |> validate_required([:short_name, :full_name, :course, :budget])
  end
end
