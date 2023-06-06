defmodule Scheduler.Repo.Migrations.CreateLessons do
  use Ecto.Migration

  def change do
    create table(:lessons, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :short_name, :string
      add :full_name, :string

      timestamps()
    end

  end
end
