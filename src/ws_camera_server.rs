use std::collections::{HashMap, HashSet};

use crate::actix::prelude::*;

/// Available frame types
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
#[serde(tag = "type")]
pub enum FrameResolution {
    /// Standard definition frame, 480p
    SD,
    /// High definition frame, camera native resolution
    HD,
}

/// An actor address that can receive `CameraFrame` messages
type Client = Recipient<CameraFrame>;

// MESSAGES
/// Represents a frame of video
#[derive(Clone, Message, Serialize)]
pub struct CameraFrame {
    /// id of camera that produced frame
    pub camera_id: i32,
    /// jpeg image data
    pub jpeg: Vec<u8>,
    /// resolution of frame
    pub resolution: FrameResolution,
}

/// subscribe message
#[derive(Clone, Message)]
pub struct Subscribe {
    /// camera id
    pub camera_id: i32,
    /// address of WsSession to send frames
    pub client: Client,
    /// resolution specifier
    pub resolution: FrameResolution,
}

/// Unsubscribe message
#[derive(Clone, Message)]
pub struct Unsubscribe {
    /// camera id
    pub camera_id: i32,
    /// Address of WsSession to stop sending frames
    pub client: Client,
    /// resolution specifier
    pub resolution: FrameResolution,
}

// Server definitions

/// Type of camera frames
#[derive(Clone, PartialEq, Eq, Hash)]
struct FrameType {
    /// Camera id, identifies source of frame
    pub camera_id: i32,
    /// Represents resolution of frame
    pub resolution: FrameResolution,
}

/// Represents the `WsCameraServer` actor
#[derive(Default)]
pub struct WsCameraServer {
    /// Collection of Client's subscribed to a particular FrameType
    subscriptions: HashMap<FrameType, HashSet<Client>>,
}

impl WsCameraServer {
    /// Add WsSession subscriber
    fn add_subscriber(
        &mut self,
        camera_id: i32,
        client: Client,
        resolution: FrameResolution,
    ) -> bool {
        // Try to find CameraSubscription for camera_id
        let frame_type = FrameType {
            camera_id,
            resolution,
        };

        if let Some(subscription) = self.subscriptions.get_mut(&frame_type) {
            subscription.insert(client)
        } else {
            self.subscriptions
                .insert(frame_type.clone(), HashSet::new());
            self.subscriptions
                .get_mut(&frame_type)
                .expect("The subscriptions HashSet we just inserted is missing.")
                .insert(client)
        }
    }

    /// Removes WsSession as subscriber
    fn remove_subscriber(&mut self, camera_id: i32, client: &Client, resolution: &FrameResolution) {
        let frame_type = FrameType {
            camera_id,
            resolution: resolution.clone(),
        };
        if let Some(subscription) = self.subscriptions.get_mut(&frame_type) {
            if subscription.remove(client) {
                debug!("Removing subscriber... {}", camera_id);
            } else {
                debug!("Couldn't find the subscription camera...");
            }
        } else {
            return;
        }
    }

    /// Sends frame to subscribed WsSessions
    ///
    /// # Arguments
    ///
    /// * `frame` - CameraFrame to send
    /// * `ctx` - WsCameraServer Context
    ///
    fn send_frame(&mut self, frame: &CameraFrame, ctx: &<Self as Actor>::Context) {
        let frame_type = FrameType {
            camera_id: frame.camera_id,
            resolution: frame.resolution.clone(),
        };
        if let Some(subscription) = self.subscriptions.get(&frame_type) {
            for client in subscription.iter() {
                if client.do_send(frame.to_owned()).is_err() {
                    debug!(
                        "Send failed for {} {:?}",
                        &frame.camera_id, &frame.resolution
                    );
                    ctx.address().do_send(Unsubscribe {
                        camera_id: frame.camera_id,
                        resolution: frame.resolution.clone(),
                        client: client.clone(),
                    });
                }
            }
        }
    }
}

impl Actor for WsCameraServer {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Starting ws_camera_server...");
    }
}

impl Handler<Subscribe> for WsCameraServer {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _ctx: &mut Self::Context) -> Self::Result {
        self.add_subscriber(msg.camera_id, msg.client, msg.resolution);
    }
}

impl Handler<Unsubscribe> for WsCameraServer {
    type Result = ();

    fn handle(&mut self, msg: Unsubscribe, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Message: Unsubscribe");
        self.remove_subscriber(msg.camera_id, &msg.client, &msg.resolution);
    }
}

impl Handler<CameraFrame> for WsCameraServer {
    type Result = ();

    fn handle(&mut self, msg: CameraFrame, ctx: &mut Self::Context) -> Self::Result {
        self.send_frame(&msg, ctx);
    }
}

impl SystemService for WsCameraServer {}
impl Supervised for WsCameraServer {}
