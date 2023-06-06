defmodule Scheduler.Repo.Migrations.CreateTeachers do
  use Ecto.Migration

  def change do
    create table(:teachers, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :first_name, :string
      add :middle_name, :string, null: true
      add :last_name, :string

      timestamps()
    end

  end
end
