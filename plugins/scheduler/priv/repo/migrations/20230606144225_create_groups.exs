defmodule Scheduler.Repo.Migrations.CreateGroups do
  use Ecto.Migration

  def change do
    create table(:groups, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :short_name, :string
      add :full_name, :string
      add :course, :integer
      add :budget, :integer

      timestamps()
    end

  end
end
