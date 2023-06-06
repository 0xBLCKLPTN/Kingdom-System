defmodule Scheduler.SchedulesTest do
  use Scheduler.DataCase

  alias Scheduler.Schedules

  describe "schedules" do
    alias Scheduler.Schedules.Schedule

    @valid_attrs %{class_number: 42, lesson_number: 42, timestamp: 42}
    @update_attrs %{class_number: 43, lesson_number: 43, timestamp: 43}
    @invalid_attrs %{class_number: nil, lesson_number: nil, timestamp: nil}

    def schedule_fixture(attrs \\ %{}) do
      {:ok, schedule} =
        attrs
        |> Enum.into(@valid_attrs)
        |> Schedules.create_schedule()

      schedule
    end

    test "list_schedules/0 returns all schedules" do
      schedule = schedule_fixture()
      assert Schedules.list_schedules() == [schedule]
    end

    test "get_schedule!/1 returns the schedule with given id" do
      schedule = schedule_fixture()
      assert Schedules.get_schedule!(schedule.id) == schedule
    end

    test "create_schedule/1 with valid data creates a schedule" do
      assert {:ok, %Schedule{} = schedule} = Schedules.create_schedule(@valid_attrs)
      assert schedule.class_number == 42
      assert schedule.lesson_number == 42
      assert schedule.timestamp == 42
    end

    test "create_schedule/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Schedules.create_schedule(@invalid_attrs)
    end

    test "update_schedule/2 with valid data updates the schedule" do
      schedule = schedule_fixture()
      assert {:ok, %Schedule{} = schedule} = Schedules.update_schedule(schedule, @update_attrs)
      assert schedule.class_number == 43
      assert schedule.lesson_number == 43
      assert schedule.timestamp == 43
    end

    test "update_schedule/2 with invalid data returns error changeset" do
      schedule = schedule_fixture()
      assert {:error, %Ecto.Changeset{}} = Schedules.update_schedule(schedule, @invalid_attrs)
      assert schedule == Schedules.get_schedule!(schedule.id)
    end

    test "delete_schedule/1 deletes the schedule" do
      schedule = schedule_fixture()
      assert {:ok, %Schedule{}} = Schedules.delete_schedule(schedule)
      assert_raise Ecto.NoResultsError, fn -> Schedules.get_schedule!(schedule.id) end
    end

    test "change_schedule/1 returns a schedule changeset" do
      schedule = schedule_fixture()
      assert %Ecto.Changeset{} = Schedules.change_schedule(schedule)
    end
  end
end
