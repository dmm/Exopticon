defmodule Exopticon.VideoTest do
  use ExUnit.Case
  @moduletag integration: true
  use Exopticon.DataCase

  alias Exopticon.Video

  describe "camera_groups" do
    alias Exopticon.Video.CameraGroup

    @valid_attrs %{max_storage_size: 42, name: "some name", storage_path: "some storage_path"}
    @update_attrs %{max_storage_size: 43, name: "some updated name", storage_path: "some updated storage_path"}
    @invalid_attrs %{max_storage_size: nil, name: nil, storage_path: nil}

    def camera_group_fixture(attrs \\ %{}) do
      {:ok, camera_group} =
        attrs
        |> Enum.into(@valid_attrs)
        |> Video.create_camera_group()

      camera_group
    end

    test "list_camera_groups/0 returns all camera_groups" do
      camera_group = camera_group_fixture()
      assert Video.list_camera_groups() == [camera_group]
    end

    test "get_camera_group!/1 returns the camera_group with given id" do
      camera_group = camera_group_fixture()
      assert Video.get_camera_group!(camera_group.id) == camera_group
    end

    test "create_camera_group/1 with valid data creates a camera_group" do
      assert {:ok, %CameraGroup{} = camera_group} = Video.create_camera_group(@valid_attrs)
      assert camera_group.max_storage_size == 42
      assert camera_group.name == "some name"
      assert camera_group.storage_path == "some storage_path"
    end

    test "create_camera_group/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Video.create_camera_group(@invalid_attrs)
    end

    test "update_camera_group/2 with valid data updates the camera_group" do
      camera_group = camera_group_fixture()
      assert {:ok, camera_group} = Video.update_camera_group(camera_group, @update_attrs)
      assert %CameraGroup{} = camera_group
      assert camera_group.max_storage_size == 43
      assert camera_group.name == "some updated name"
      assert camera_group.storage_path == "some updated storage_path"
    end

    test "update_camera_group/2 with invalid data returns error changeset" do
      camera_group = camera_group_fixture()
      assert {:error, %Ecto.Changeset{}} = Video.update_camera_group(camera_group, @invalid_attrs)
      assert camera_group == Video.get_camera_group!(camera_group.id)
    end

    test "delete_camera_group/1 deletes the camera_group" do
      camera_group = camera_group_fixture()
      assert {:ok, %CameraGroup{}} = Video.delete_camera_group(camera_group)
      assert_raise Ecto.NoResultsError, fn -> Video.get_camera_group!(camera_group.id) end
    end

    test "change_camera_group/1 returns a camera_group changeset" do
      camera_group = camera_group_fixture()
      assert %Ecto.Changeset{} = Video.change_camera_group(camera_group)
    end
  end

  describe "cameras" do
    alias Exopticon.Video.Camera

    @valid_attrs %{fps: 42, ip: "some ip", mac: "some mac", name: "some name", onvif_port: 42, password: "some password", rtsp_url: "some rtsp_url", type: "some type", username: "some username"}
    @update_attrs %{fps: 43, ip: "some updated ip", mac: "some updated mac", name: "some updated name", onvif_port: 43, password: "some updated password", rtsp_url: "some updated rtsp_url", type: "some updated type", username: "some updated username"}
    @invalid_attrs %{fps: nil, ip: nil, mac: nil, name: nil, onvif_port: nil, password: nil, rtsp_url: nil, type: nil, username: nil}

    def camera_fixture(attrs \\ %{}) do
      {:ok, camera} =
        attrs
        |> Enum.into(@valid_attrs)
        |> Video.create_camera()

      camera
    end

    test "list_cameras/0 returns all cameras" do
      camera = camera_fixture()
      assert Video.list_cameras() == [camera]
    end

    test "get_camera!/1 returns the camera with given id" do
      camera = camera_fixture()
      assert Video.get_camera!(camera.id) == camera
    end

    test "create_camera/1 with valid data creates a camera" do
      assert {:ok, %Camera{} = camera} = Video.create_camera(@valid_attrs)
      assert camera.fps == 42
      assert camera.ip == "some ip"
      assert camera.mac == "some mac"
      assert camera.name == "some name"
      assert camera.onvif_port == 42
      assert camera.password == "some password"
      assert camera.rtsp_url == "some rtsp_url"
      assert camera.type == "some type"
      assert camera.username == "some username"
    end

    test "create_camera/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Video.create_camera(@invalid_attrs)
    end

    test "update_camera/2 with valid data updates the camera" do
      camera = camera_fixture()
      assert {:ok, camera} = Video.update_camera(camera, @update_attrs)
      assert %Camera{} = camera
      assert camera.fps == 43
      assert camera.ip == "some updated ip"
      assert camera.mac == "some updated mac"
      assert camera.name == "some updated name"
      assert camera.onvif_port == 43
      assert camera.password == "some updated password"
      assert camera.rtsp_url == "some updated rtsp_url"
      assert camera.type == "some updated type"
      assert camera.username == "some updated username"
    end

    test "update_camera/2 with invalid data returns error changeset" do
      camera = camera_fixture()
      assert {:error, %Ecto.Changeset{}} = Video.update_camera(camera, @invalid_attrs)
      assert camera == Video.get_camera!(camera.id)
    end

    test "delete_camera/1 deletes the camera" do
      camera = camera_fixture()
      assert {:ok, %Camera{}} = Video.delete_camera(camera)
      assert_raise Ecto.NoResultsError, fn -> Video.get_camera!(camera.id) end
    end

    test "change_camera/1 returns a camera changeset" do
      camera = camera_fixture()
      assert %Ecto.Changeset{} = Video.change_camera(camera)
    end
  end

  describe "files" do
    alias Exopticon.Video.File

    @valid_attrs %{begin_monotonic: 42, begin_time: "2010-04-17 14:00:00.000000Z", end_monotonic: 42, end_time: "2010-04-17 14:00:00.000000Z", filename: "some filename", monotonic_index: 42, size: 42}
    @update_attrs %{begin_monotonic: 43, begin_time: "2011-05-18 15:01:01.000000Z", end_monotonic: 43, end_time: "2011-05-18 15:01:01.000000Z", filename: "some updated filename", monotonic_index: 43, size: 43}
    @invalid_attrs %{begin_monotonic: nil, begin_time: nil, end_monotonic: nil, end_time: nil, filename: nil, monotonic_index: nil, size: nil}

    def file_fixture(attrs \\ %{}) do
      {:ok, file} =
        attrs
        |> Enum.into(@valid_attrs)
        |> Video.create_file()

      file
    end

    test "list_files/0 returns all files" do
      file = file_fixture()
      assert Video.list_files() == [file]
    end

    test "get_file!/1 returns the file with given id" do
      file = file_fixture()
      assert Video.get_file!(file.id) == file
    end

    test "create_file/1 with valid data creates a file" do
      assert {:ok, %File{} = file} = Video.create_file(@valid_attrs)
      assert file.begin_monotonic == 42
      assert file.begin_time == DateTime.from_naive!(~N[2010-04-17 14:00:00.000000Z], "Etc/UTC")
      assert file.end_monotonic == 42
      assert file.end_time == DateTime.from_naive!(~N[2010-04-17 14:00:00.000000Z], "Etc/UTC")
      assert file.filename == "some filename"
      assert file.monotonic_index == 42
      assert file.size == 42
    end

    test "create_file/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Video.create_file(@invalid_attrs)
    end

    test "update_file/2 with valid data updates the file" do
      file = file_fixture()
      assert {:ok, file} = Video.update_file(file, @update_attrs)
      assert %File{} = file
      assert file.begin_monotonic == 43
      assert file.begin_time == DateTime.from_naive!(~N[2011-05-18 15:01:01.000000Z], "Etc/UTC")
      assert file.end_monotonic == 43
      assert file.end_time == DateTime.from_naive!(~N[2011-05-18 15:01:01.000000Z], "Etc/UTC")
      assert file.filename == "some updated filename"
      assert file.monotonic_index == 43
      assert file.size == 43
    end

    test "update_file/2 with invalid data returns error changeset" do
      file = file_fixture()
      assert {:error, %Ecto.Changeset{}} = Video.update_file(file, @invalid_attrs)
      assert file == Video.get_file!(file.id)
    end

    test "delete_file/1 deletes the file" do
      file = file_fixture()
      assert {:ok, %File{}} = Video.delete_file(file)
      assert_raise Ecto.NoResultsError, fn -> Video.get_file!(file.id) end
    end

    test "change_file/1 returns a file changeset" do
      file = file_fixture()
      assert %Ecto.Changeset{} = Video.change_file(file)
    end
  end
end
