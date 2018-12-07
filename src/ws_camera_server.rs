use crate::actix::prelude::*;

//use std::collections::HashMap;
//use std::mem;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum FrameResolution {
    SD,
    HD,
}

type Client = Recipient<CameraFrame>;

// MESSAGES
#[derive(Clone, Message, Serialize)]
pub struct CameraFrame {
    pub camera_id: i32,
    pub jpeg: Vec<u8>,
    pub resolution: FrameResolution,
}

#[derive(Clone, Message)]
pub struct Subscribe {
    pub camera_id: i32,
    pub client: Client,
    pub resolution: FrameResolution,
}

#[derive(Clone, Message)]
pub struct Unsubscribe {
    pub camera_id: i32,
    pub client: Client,
    pub resolution: FrameResolution,
}

// Server definitions

struct CameraSubscription {
    pub camera_id: i32,
    pub subscribers: Vec<(Client, FrameResolution)>,
}

#[derive(Default)]
pub struct WsCameraServer {
    subscriptions: Vec<CameraSubscription>,
}

impl WsCameraServer {
    fn add_subscriber(&mut self, camera_id: i32, client: Client, resolution: FrameResolution) {
        // Try to find CameraSubscription for camera_id
        let pos = self
            .subscriptions
            .iter()
            .position(|s| s.camera_id == camera_id);

        let pos = match pos {
            Some(p) => p,
            None => {
                self.subscriptions.push(CameraSubscription {
                    camera_id: camera_id,
                    subscribers: Vec::new(),
                });
                self.subscriptions.len() - 1
            }
        };

        let client_pos = self.subscriptions[pos]
            .subscribers
            .iter()
            .position(|(c, r)| *c == client && *r == resolution);

        match client_pos {
            Some(_c) => {
                // client is already subscribed, do nothing
            }
            None => {
                self.subscriptions[pos]
                    .subscribers
                    .push((client, resolution));
            }
        };
    }

    fn remove_subscriber(&mut self, camera_id: i32, client: Client, resolution: FrameResolution) {
        if let Some(pos) = self
            .subscriptions
            .iter()
            .position(|ref s| s.camera_id == camera_id)
        {
            debug!("Couldn't find the subscription camera...");
            if let Some(client_pos) = self.subscriptions[pos]
                .subscribers
                .iter()
                .position(|(c, r)| *c == client && *r == resolution)
            {
                debug!("Removing subscriber... {} {:?}", camera_id, resolution);
                self.subscriptions[pos].subscribers.remove(client_pos);
            }
        }
    }

    fn send_frame(&mut self, frame: CameraFrame) {
        let mut failed = Vec::new();

        if let Some(pos) = self
            .subscriptions
            .iter()
            .position(|ref s| s.camera_id == frame.camera_id)
        {
            for (i, (client, resolution)) in self.subscriptions[pos].subscribers.iter().enumerate()
            {
                if *resolution == frame.resolution && !client.do_send(frame.to_owned()).is_ok() {
                    failed.push(i);
                }
            }

            for i in failed.iter() {
                debug!("Send failed. Removing subscriber...");
                self.subscriptions[pos].subscribers.remove(*i);
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
        self.remove_subscriber(msg.camera_id, msg.client, msg.resolution);
    }
}

impl Handler<CameraFrame> for WsCameraServer {
    type Result = ();

    fn handle(&mut self, msg: CameraFrame, _ctx: &mut Self::Context) -> Self::Result {
        self.send_frame(msg);
    }
}

impl SystemService for WsCameraServer {}
impl Supervised for WsCameraServer {}
