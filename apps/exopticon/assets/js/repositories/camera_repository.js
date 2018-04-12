/*
 * This file is part of Exopticon (https://github.com/dmm/exopticon).
 * Copyright (c) 2018 David Matthew Mattli
 *
 * Exopticon is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Exopticon is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Exopticon.  If not, see <http://www.gnu.org/licenses/>.
 */

/**
 * CameraRepository fetches cameras and information about camera
 * recordings
 * @class
 */
class CameraRepository {
  /**
   * fetches coverage information and available files
   * @param {Number} cameraId - id of camera to get file for
   * @param {ZonedDateTime} beginTime - start of time to get availability for
   * @param {ZonedDateTime} endTime - end of time to get availability for
   */
  getAvailability(cameraId, beginTime, endTime) {
    let url = `/v1/cameras/${cameraId}/availability`;
    fetch(url, {
      credentials: 'same-origin',
      headers: {
        'Content-Type': 'application/json',
      },
    }).then((response) => {
      return response.json();
    });
  }
}

export default CameraRepository;
