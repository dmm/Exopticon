# This file is a part of Exopticon, a free video surveillance tool. Visit
# https://exopticon.org for more information.
#
# Copyright (C) 2018 David Matthew Mattli
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

defmodule ExopticonWeb.AnnotationController do
  use ExopticonWeb, :controller

  plug(:authenticate_user)

  alias Exopticon.Video
  alias Exopticon.Video.Camera

  def sd_snapshot(conn, %{"id" => id}) do
    annotation = Video.get_annotation!(id)

    conn
    |> put_resp_content_type("image/jpeg")
    |> send_file(200, annotation.sd_filename)
  end

  def hd_snapshot(conn, %{"id" => id}) do
    annotation = Video.get_annotation!(id)

    conn
    |> put_resp_content_type("image/jpeg")
    |> send_file(200, annotation.hd_filename)
  end
end
