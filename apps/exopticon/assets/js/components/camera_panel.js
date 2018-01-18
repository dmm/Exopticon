'use strict';

import React from 'react';
import CameraView from './camera_view';
import CameraPlayer from '../camera_player';

class CameraPanel extends React.Component {
  constructor(props) {
    super(props);

    this.updateCameras = this.updateCameras.bind(this);
    this.updateCameras();
    this.cameraElements = new Map();

    var channel = props.socket.channel('camera:stream');
    channel.join();

    this.state = {
      cameras: props.initialCameras,
      channel: channel,
      viewColumns: 0
    };
  }

  componentDidMount() {

  }

  componentWillUnmount() {
    this.state.channel.leave();
  }

  updateCameras() {
    fetch('/v1/cameras', {
      credentials: 'same-origin',
      headers: {
        'Content-Type': 'application/json'
      }
    }).then((response) => {
      return response.json();
    }).then((cameras) => {
      this.setState({cameras: cameras});
    });
  }

  render() {
    var cameraPanelClass = 'camera-panel';

    if (this.state.viewColumns !== 0) {
      cameraPanelClass += `panel-col-${this.state.viewColumns.toString()}`;
    }
    this.cameraElements.clear();
    const cameras =[];
    this.state.cameras.forEach((cam) => {
      let player = new CameraPlayer(cam, this.state.channel);
      cameras.push(
        <div key={cam.id} className="wrapper">
          <div className="camera-width"></div>
          <div className="content">
            <CameraView camera={cam}
                        cameraPlayer={player}

                        ref={
                          (el) => {
                            this.cameraElements.set(cam.id, el);
                          }
              }/>
          </div>
        </div>
      );
    });
    return (
      <div className={cameraPanelClass}>
        {cameras}
      </div>
    );
  }
}

export default CameraPanel;
