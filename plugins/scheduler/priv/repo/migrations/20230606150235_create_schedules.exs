defmodule Scheduler.Repo.Migrations.CreateSchedules do
  use Ecto.Migration

  def change do
    create table(:schedules, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :timestamp, :integer
      add :class_number, :integer
      add :lesson_number, :integer
      add :lesson_id, references(:lessons, on_delete: :nothing, type: :binary_id)
      add :group_id, references(:groups, on_delete: :nothing, type: :binary_id)
      add :teacher_id, references(:teachers, on_delete: :nothing, type: :binary_id)

      timestamps()
    end

    create index(:schedules, [:lesson_id])
    create index(:schedules, [:group_id])
    create index(:schedules, [:teacher_id])
  end
end
