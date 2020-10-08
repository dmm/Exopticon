use std::collections::HashMap;
//use std::convert::TryInto;

use actix::prelude::*;
use actix_interop::{critical_section, with_ctx, FutureInterop};

use crate::db_registry;
use crate::models::FetchAllNotifier;
use crate::telegram_actor::TelegramActor;

/// Notifier Supervisor starts child notifiers and routes notifications
pub struct NotifierSupervisor {
    /// child workers
    workers: HashMap<i32, Addr<TelegramActor>>,
}

impl Actor for NotifierSupervisor {
    type Context = Context<Self>;
}

impl Default for NotifierSupervisor {
    /// default initializer
    fn default() -> Self {
        Self {
            workers: HashMap::new(),
        }
    }
}

impl Supervised for NotifierSupervisor {}

impl SystemService for NotifierSupervisor {
    /// Service start handler that loads child notifiers
    fn service_started(&mut self, ctx: &mut Context<Self>) {
        debug!("Notifier Supervisor started");
        ctx.notify(SyncNotifiers {});
    }
}

/// Message requesting the supervisor reload child notifiers from the database
pub struct SyncNotifiers {}

impl Message for SyncNotifiers {
    type Result = ();
}

impl Handler<SyncNotifiers> for NotifierSupervisor {
    type Result = ();

    fn handle(&mut self, _msg: SyncNotifiers, ctx: &mut Context<Self>) -> Self::Result {
        // fetch notifiers
        let fut = async move {
            critical_section::<Self, _>(async move {
                // remove all references to Notifier workers
                with_ctx(|actor: &mut Self, _| {
                    actor.workers.clear();
                });

                // Fetch notifiers
                let notifiers = match db_registry::get_db().send(FetchAllNotifier {}).await {
                    Ok(Ok(n)) => n,
                    Ok(Err(_)) | Err(_) => return,
                };

                for n in notifiers {
                    debug!("Starting notifier actor!");
                    let address =
                        TelegramActor::new(n.password.clone().unwrap_or_else(|| String::from("")))
                            .start();
                    with_ctx(|actor: &mut Self, _| {
                        actor.workers.insert(n.id, address);
                    });
                }
            })
            .await;
        }
        .interop_actor(self);
        ctx.spawn(fut);
    }
}

/// Message requesting to send an alert to the given notifier
pub struct SendNotification {
    /// id of target notifier
    pub notifier_id: i32,
    /// name of targeted contact group
    pub contact_group: String,
    /// Optional Message
    pub message: Option<String>,
    /// Optional image attachment
    pub attachment: Option<Vec<u8>>,
}

impl Message for SendNotification {
    type Result = ();
}

impl Handler<SendNotification> for NotifierSupervisor {
    type Result = ();

    fn handle(&mut self, msg: SendNotification, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(addr) = self.workers.get(&msg.notifier_id) {
            addr.do_send(msg);
        }
    }
}