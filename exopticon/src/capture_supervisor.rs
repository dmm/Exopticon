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

use std::time::Duration;

use actix::fut::wrap_future;
use actix::prelude::*;

use crate::capture_actor::CaptureActor;

/// Message instructing `CaptureSupervisor` to start a capture actor
pub struct StartCaptureWorker {
    /// id of camera to start capture worker for
    pub id: i32,
    /// rtsp url of video stream to capture
    pub stream_url: String,
    /// full path to capture video to
    pub storage_path: String,
}

impl Message for StartCaptureWorker {
    type Result = ();
}

/// Message instructing `CaptureSupervisor` to stop the specified worker
pub struct StopCaptureWorker {
    /// stop the capture actor associated with this camera id
    pub id: i32,
}

impl Message for StopCaptureWorker {
    type Result = ();
}

/// Message requesting a `CaptureWorker` restart
pub struct RestartCaptureWorker {
    /// id of camera to restart capture worker for
    pub id: i32,
    /// rtsp url of video stream to capture
    pub stream_url: String,
    /// full path to capture video to
    pub storage_path: String,
}

impl Message for RestartCaptureWorker {
    type Result = ();
}

/// holds state of `CaptureSupervisor` actor
pub struct CaptureSupervisor {
    /// Child workers
    workers: Vec<(i32, Addr<CaptureActor>)>,
}

impl Actor for CaptureSupervisor {
    type Context = Context<Self>;
}

impl Default for CaptureSupervisor {
    fn default() -> Self {
        Self {
            workers: Vec::new(),
        }
    }
}

impl Supervised for CaptureSupervisor {}

impl SystemService for CaptureSupervisor {}

impl Handler<StartCaptureWorker> for CaptureSupervisor {
    type Result = ();

    fn handle(&mut self, msg: StartCaptureWorker, _ctx: &mut Context<Self>) -> Self::Result {
        info!("Supervisor: Starting camera id: {}", msg.id);
        let id = msg.id.to_owned();
        let address = CaptureActor::new(msg.id, msg.stream_url, msg.storage_path).start();
        self.workers.push((id, address));
    }
}

impl Handler<StopCaptureWorker> for CaptureSupervisor {
    type Result = ();

    fn handle(&mut self, msg: StopCaptureWorker, _ctx: &mut Context<Self>) -> Self::Result {
        info!("Stopping camera id: {}", msg.id);
        self.workers.retain(|(id, _)| *id != msg.id);
    }
}

impl Handler<RestartCaptureWorker> for CaptureSupervisor {
    type Result = ();

    fn handle(&mut self, msg: RestartCaptureWorker, ctx: &mut Context<Self>) -> Self::Result {
        info!("Restarting camera id: {}", msg.id);
        let fut = wrap_future(ctx.address().send(StopCaptureWorker { id: msg.id })).map(
            |_res, _act: &mut Self, ctx: &mut Context<Self>| {
                ctx.notify_later(
                    StartCaptureWorker {
                        id: msg.id,
                        stream_url: msg.stream_url,
                        storage_path: msg.storage_path,
                    },
                    Duration::new(5, 0),
                );
            },
        );
        ctx.spawn(fut);
    }
}
