defmodule Exopticon.Accounts.User do
  @moduledoc """
  Provides Accounts.User schema.
  """
  use Ecto.Schema
  import Ecto.Changeset
  alias Exopticon.Accounts.User

  import Comeonin.Bcrypt, only: [hashpwsalt: 1]

  schema "users" do
    field(:name, :string)
    field(:password, :string, virtual: true)
    field(:password_hash, :string)
    field(:username, :string)
    field(:timezone, :string)

    timestamps()
  end

  @doc false
  def changeset(%User{} = user, attrs) do
    user
    |> cast(attrs, [:name, :username, :password, :timezone])
    |> validate_required([:name, :username, :password, :timezone])
    |> unique_constraint(:username)
  end

  def registration_changeset(%User{} = user, attrs \\ :empty) do
    user
    |> changeset(attrs)
    |> cast(attrs, ~w(password), [])
    |> validate_length(:password, min: 6, max: 100)
    |> put_pass_hash()
  end

  defp put_pass_hash(changeset) do
    case changeset do
      %Ecto.Changeset{valid?: true, changes: %{password: pass}} ->
        put_change(changeset, :password_hash, hashpwsalt(pass))

      _ ->
        changeset
    end
  end
end
