defmodule Scheduler.TeachersTest do
  use Scheduler.DataCase

  alias Scheduler.Teachers

  describe "teachers" do
    alias Scheduler.Teachers.Teacher

    @valid_attrs %{first_name: "some first_name", last_name: "some last_name", middle_name: "some middle_name"}
    @update_attrs %{first_name: "some updated first_name", last_name: "some updated last_name", middle_name: "some updated middle_name"}
    @invalid_attrs %{first_name: nil, last_name: nil, middle_name: nil}

    def teacher_fixture(attrs \\ %{}) do
      {:ok, teacher} =
        attrs
        |> Enum.into(@valid_attrs)
        |> Teachers.create_teacher()

      teacher
    end

    test "list_teachers/0 returns all teachers" do
      teacher = teacher_fixture()
      assert Teachers.list_teachers() == [teacher]
    end

    test "get_teacher!/1 returns the teacher with given id" do
      teacher = teacher_fixture()
      assert Teachers.get_teacher!(teacher.id) == teacher
    end

    test "create_teacher/1 with valid data creates a teacher" do
      assert {:ok, %Teacher{} = teacher} = Teachers.create_teacher(@valid_attrs)
      assert teacher.first_name == "some first_name"
      assert teacher.last_name == "some last_name"
      assert teacher.middle_name == "some middle_name"
    end

    test "create_teacher/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Teachers.create_teacher(@invalid_attrs)
    end

    test "update_teacher/2 with valid data updates the teacher" do
      teacher = teacher_fixture()
      assert {:ok, %Teacher{} = teacher} = Teachers.update_teacher(teacher, @update_attrs)
      assert teacher.first_name == "some updated first_name"
      assert teacher.last_name == "some updated last_name"
      assert teacher.middle_name == "some updated middle_name"
    end

    test "update_teacher/2 with invalid data returns error changeset" do
      teacher = teacher_fixture()
      assert {:error, %Ecto.Changeset{}} = Teachers.update_teacher(teacher, @invalid_attrs)
      assert teacher == Teachers.get_teacher!(teacher.id)
    end

    test "delete_teacher/1 deletes the teacher" do
      teacher = teacher_fixture()
      assert {:ok, %Teacher{}} = Teachers.delete_teacher(teacher)
      assert_raise Ecto.NoResultsError, fn -> Teachers.get_teacher!(teacher.id) end
    end

    test "change_teacher/1 returns a teacher changeset" do
      teacher = teacher_fixture()
      assert %Ecto.Changeset{} = Teachers.change_teacher(teacher)
    end
  end
end
