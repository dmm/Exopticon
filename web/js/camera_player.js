'use strict';

import SuperImage from './super_image';

/**
 * CameraPlayer controls a camera stream
 * @class
 */
class CameraPlayer {
  /**
   * @param {Exopticon.Camera} camera - Camera object to play
   * @param {Phoenix.Socket} socket - socket to connect over
   */
  constructor(camera, socket) {
    this.camera = camera;
    Object.assign(this, camera);
    this.relativeMoveUrl = `/v1/cameras/${camera.id}/relativeMove`;
    this.annotationUrl = '/v1/annotations';
    this.status = 'paused';
    this.socket = socket;
    this.channel = undefined;
    this.img = null;
    this.statusCallback = () => {};
    this.lastFrame = undefined;
    this.resolutionSuffix = 'sd';
    this.cb = () => {};

    // Bind functions so they can be used as callbacks
    // and use 'this'.
    this.left = this.left.bind(this);
    this.right = this.right.bind(this);
    this.up = this.up.bind(this);
    this.down = this.down.bind(this);
    this.playRealtime = this.playRealtime.bind(this);
    this.takeSnapshot = this.takeSnapshot.bind(this);
  }

  /**
   * Change player status, firing callbacks if different
   * @param {string} newStatus - status to change to
   * @private
   */
  setStatus(newStatus) {
    const oldStatus = this.status;
    this.status = newStatus;
    if (oldStatus !== newStatus) {
      this.statusCallback(newStatus);
    }
  }

  /**
   * begin realtime playback of camera to given Image object
   * @param {Image} img - image object to stream video to
   * @param {Function} cb - function to call when playback starts
   */
  playRealtime(img, cb = ()=>{}) {
    this.setStatus('loading');
    this.domImg = img;
    this.cb = cb;
    this.img = new SuperImage(img);

    this.channel = this.socket.channel(`camera:${this.camera.id}${this.resolutionSuffix}`);
    this.channel.on("frame", frame => {
      this.channel.push('ack', {ts: frame.ts});
      if (this.status !== 'paused' && this.img !== null) {
        this.lastFrame = frame;
        this.setStatus('playing');
        this.img.renderArrayIfReady(frame.frameJpeg);
        this.cb();
      }
    });
    this.channel.join();
  }

  /**
   * stop all playback of given camera
   */
  stop() {
    if (this.channel) {
      this.channel.leave();
    }
    this.setStatus('paused');
    this.img = null;
  }

  /**
   * allows configuration of video resolution
   * @param {string} resolution - resolution flag, either 'sd' or 'hd'
   */
  setResolution(resolution) {
    const oldResolution = this.resolutionSuffix;
    if (resolution === 'hd') {
      this.resolutionSuffix = '';
    } else if (resolution === 'sd') {
      this.resolutionSuffix = 'sd';
    }

    if (oldResolution !== this.resolutionSuffix) {
    this.stop();
      this.playRealtime(this.domImg, this.cb);
    }
  }

  /**
   * @return {boolean} true if camera report ptz capability
   */
  hasPtz() {
    if (this.camera.ptzType === null) {
      return false;
    } else {
      return true;
    }
  }

  /**
   * request relative movement from camera
   * @param {number} x - number between -1 and 1 specifying amount to
   *                     move horizontally
   * @param {number} y - number between -1 and 1 specifying amount to
   *                     move vertically
   * @param {Function} callback - movement complete callback
   */
  relativeMove(x, y, callback) {
    fetch(this.relativeMoveUrl,
          {
            method: 'post',
            credentials: 'same-origin',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({x: x, y: y}),
    }).then(function(response) {
      if (callback) callback(response);
    });
  }

  /**
   * move camera left
   */
  left() {
    this.relativeMove('-0.05', '0.0');
  }

  /**
   * move camera right
   */
  right() {
    this.relativeMove('0.05', '0.0');
  }

  /**
   * move camera up
   */
  up() {
    this.relativeMove('0.0', '0.1');
  }

  /**
   * move camera down
   */
  down() {
    this.relativeMove('0.0', '-0.1');
  }

  /**
   * take snapshot of frame
   * @param {Function} callback - function to call when snapshot complete.
   */
  takeSnapshot(callback) {
    const id = this.lastFrame.videoUnitId;
    const index = this.lastFrame.frameIndex;
    const offset = this.lastFrame.offset;
    console.log(`Taking snapshot of ${id} ${index}`);
    if (id !== 0 && index !== 0) {
      fetch(this.annotationUrl,
          {
            method: 'post',
            credentials: 'same-origin',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({
              video_unit_id: id,
              frame_index: index,
              offset: offset,
              key: 'snapshot',
              value: 'snapshot',
              source: 'user',
              ul_x: -1, ul_y: -1, width: -1, height: -1}),
          });
    }
  }
}

export default CameraPlayer;
