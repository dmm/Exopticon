/*
 * Exopticon - A free video surveillance system.
 * Copyright (C) 2020 David Matthew Mattli <dmm@mattli.us>
 *
 * This file is part of Exopticon.
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

import {
  ChangeDetectorRef,
  Component,
  HostListener,
  NgZone,
  OnInit,
} from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { Camera } from "../camera";
import { CameraPanelService } from "../camera-panel.service";
import { CameraService, PtzDirection } from "../camera.service";
import { CameraResolution } from "../frame-message";
import { VideoService } from "../video.service";

@Component({
  selector: "app-camera-panel",
  templateUrl: "./camera-panel.component.html",
  styleUrls: ["./camera-panel.component.css"],
  providers: [CameraPanelService],
})
export class CameraPanelComponent implements OnInit {
  cameras: Camera[];
  enabledCameras: Camera[];
  enabledCamerasOffset: number = 0;
  error: any;
  selectedCameraId?: number;
  overlayDisabledId?: number;
  private cameraVisibility: Map<number, boolean>;
  private columns: number;
  private rows: number;
  private resolution: CameraResolution;

  constructor(
    public cameraPanelService: CameraPanelService,
    private cameraService: CameraService,
    public videoService: VideoService,
    private cdr: ChangeDetectorRef,
    private route: ActivatedRoute,
    private router: Router,
    private ngZone: NgZone
  ) {
    this.resolution = CameraResolution.Sd;
  }

  getCameras(): void {
    this.cameraService.getCameras().subscribe((cameras) => {});
  }

  ngOnInit() {
    this.route.paramMap.subscribe((params) => {
      if (params.has("cols")) {
        this.cameraPanelService.setCols(parseInt(params.get("cols"), 10));
      }
      if (params.has("rows")) {
        this.cameraPanelService.setRows(parseInt(params.get("rows"), 10));
      }

      if (params.has("offset")) {
        this.cameraPanelService.setOffset(parseInt(params.get("offset"), 10));
      }

      if (params.has("res")) {
        if (params.get("res").toLowerCase() === "hd") {
          this.cameraPanelService.setResolution(CameraResolution.Hd);
        }
      }
    });

    this.videoService.connect();
  }

  @HostListener("window:keyup", ["$event"])
  KeyEvent(event: KeyboardEvent) {
    console.log("keycode: " + event.keyCode);
    let offset = this.cameraPanelService.offset;
    let cameraId = this.getKeyboardControlCameraId();
    console.log("keyboard control camera id: " + cameraId);
    switch (event.keyCode) {
      case 78:
        // 'n'
        offset++;
        break;
      case 80:
        // 'p'
        offset--;
        break;
      case 65:
        // 'a'
        this.cameraPanelService.ptz(PtzDirection.left);
        break;
      case 68:
        // 'd'
        this.cameraPanelService.ptz(PtzDirection.right);
        break;
      case 87:
        // 'w'
        this.cameraPanelService.ptz(PtzDirection.up);
        break;
      case 83:
        // 's'
        this.cameraPanelService.ptz(PtzDirection.down);
        break;
    }

    if (offset !== this.cameraPanelService.offset) {
      this.router.navigate(
        ["./", this.merge({ offset: offset }, this.route.snapshot.params)],
        {
          queryParamsHandling: "preserve",
          relativeTo: this.route,
        }
      );
    }
  }

  merge(newParams: any, oldParams: any): any {
    let params = Object.assign({}, oldParams);

    for (var prop in newParams) {
      if (newParams.hasOwnProperty(prop)) {
        let newValue = newParams[prop];
        if (newValue === null) {
          delete params[prop];
        } else {
          params[prop] = newValue;
        }
      }
    }

    return params;
  }

  selectCameraView(cameraIndex: number) {
    if (cameraIndex !== this.overlayDisabledId) {
      this.selectedCameraId = cameraIndex;
    }
  }

  deselectCameraView() {
    this.selectedCameraId = null;
  }

  onTouchStart(cameraIndex: number) {
    if (this.selectedCameraId === cameraIndex) {
      this.selectedCameraId = null;
      this.overlayDisabledId = cameraIndex;
    } else {
      this.selectedCameraId = cameraIndex;
      this.overlayDisabledId = null;
    }
  }

  updateCameraViewVisibility(cameraId: number, visible: boolean) {
    this.cameraVisibility.set(cameraId, visible);
  }

  getKeyboardControlCameraId(): number {
    return;
    if (this.enabledCameras.length === 1) {
      return this.enabledCameras[0].id;
    } else if (this.selectedCameraId) {
      return this.selectedCameraId;
    }

    return -1;
  }
}
