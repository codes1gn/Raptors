// LICENSE PLACEHOLDER

use tokio::sync::mpsc::{self, Receiver, Sender};
use uuid::Uuid;

use crate::messages::TypedMessage;

// DESIGN IDEA:
//  Split Mailbox into Context and Mailbox
//
//  Context contains the destination actor id and the sender.
//      Default is its own actor id and its sender.
//
//  Mailbox contians the receiver of the actor to deal with
//      receiving, dealing and skipping.
//
//  System could manage a HashMap(fnv), key: actor id, value: sender.
//      Once an actor tries to send a message,  it will query the sender first, then finish sending.

pub struct Context {
    actor_id: Uuid,           // Uuid of the destination actor
    tx: Sender<TypedMessage>, // The sender of the channel
}

impl Context {
    pub fn new(id: Uuid, tx: Sender<TypedMessage>) -> Self {
        return Self {
            actor_id: id,
            tx: tx,
        };
    }

    pub fn id(&self) -> Uuid {
        self.actor_id.clone()
    }

    pub fn set_context(&mut self, id: Uuid, tx: Sender<TypedMessage>) {
        self.actor_id = id;
        self.tx = tx;
    }

    pub fn send(&self, msg: TypedMessage) {
        self.tx.try_send(msg);
    }
}
