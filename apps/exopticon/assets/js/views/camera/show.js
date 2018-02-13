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

import {use as jsJodaUse, ZonedDateTime, ZoneOffset} from 'js-joda';
import jsJodaTimeZone from 'js-joda-timezone';
import React from 'react';
import ReactDOM from 'react-dom';

import CameraManager from '../../camera_manager.js';
import MainView from '../main';
import ProgressBar from '../../components/progress_bar';
import socket from '../../socket';

jsJodaUse(jsJodaTimeZone);

/**
 * CameraShow class
 * Implements page that shows a single camera
 */
export default class view extends MainView {
  /**
   * fetches camera details
   * @param {number} cameraId
   */
  fetchCamera(cameraId) {
    let request = new XMLHttpRequest();
    request.open('GET', '/v1/cameras/' + cameraId, true);
    request.onload = function() {
      if (this.status >= 200 && this.status < 400) {
        // Success!
        let camera = JSON.parse(this.response);
        window.cameraManager.updateCameras([camera]);
      } else {
        console.log('reached server but something went wrong');
      }
    };

    request.onerror = function() {
      console.log('There was a connection error of some sort...');
    };

    request.send();
  }

  /**
   * fetches files for camera between two datetimes
   * @param {number} cameraId
   * @param {string} beginTime - iso8601 datetime
   * @param {string} endTime - iso8601 datetime
   * @param {object} progress - progresbar component
   */
  fetchFiles(cameraId, beginTime, endTime, progress) {
    fetch(`/v1/files/${cameraId}?begin_time=${beginTime}&end_time=${endTime}`, {
      credentials: 'same-origin',
      headers: {
        'Content-Type': 'application/json',
      },
    }).then((response) => {
      return response.json();
    }).then((files) => {
      console.log('Got ' + files.length + ' files. Setting state...');
      progress.setState({files: files});
    }).catch((error) => {
      console.log('There was an error fetching files: ' + error);
    });
  }

  /**
   * page entry point
   */
  mount() {
    super.mount();
    console.log('ShowCameraView mounted.');

    let cameraId = parseInt(document.getElementById('singleCamera')
                            .getAttribute('data-id'), 10);
    window.cameraManager = new CameraManager(socket);
    this.fetchCamera(cameraId);
    const now = ZonedDateTime.now(ZoneOffset.UTC);
    const then = now.minusHours(12);

    let progressBar = React.createElement(ProgressBar,
                                          {

                                          });
    this.progressComponent =
      ReactDOM.render(progressBar, document.getElementById('progress'));
    this.fetchFiles(cameraId, then.toString(),
                    now.toString(),
                    this.progressComponent);
  }
}
