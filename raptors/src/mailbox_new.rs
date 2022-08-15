// LICENSE PLACEHOLDER
use fnv::FnvHashMap;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::messages::*;

// ===-----------------------------------===
//  Note!!!
//  Design idea will read Context.rs
// ===-----------------------------------===

// wrap a dedicated executor module that only consider how to do computations
//
// TODO(long-term):
// as a interface, make refactor as Trait and expose to CRT level,
// make CRT vm to impl this trait
#[derive(Debug)]
pub struct Mailbox {
    // TODO replace with ringbuffer or deque
    // references:
    // *** circular-queue
    // *** ringbuffer
    // *** ringbuf
    // *** ring_queue
    mails: Vec<TypedMessage>, 
    rx: Receiver<TypedMessage>,
}

// TODO traits tobe put together, this is a trait only for util, not for interface
pub trait Len {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}

impl Len for Mailbox {
    fn is_empty(&self) -> bool {
        self.mails.is_empty()
    }

    fn len(&self) -> usize {
        self.mails.len()
    }
}

impl Mailbox {
    pub fn new(rx: Receiver<TypedMessage>) -> Self {
        // let (_, rx) = mpsc::channel::<TypedMessage>(16);
        return Self {
            mails: vec![],
            rx: rx,
        };
    }

    pub fn mails(&self) -> Vec<TypedMessage> {
        self.mails.clone()
    }

    pub fn receive(&mut self) -> Result<(), String> {
        let msg = self.rx.try_recv();
        if msg.is_ok() {
            self.enqueue(msg.unwrap())
        } else {
            Err("Error!".to_owned())
        }
    }

    pub fn deal(&mut self) -> Result<String, String> {
        let deq = self.dequeue();
        match deq {
            None => Err("Dequeue an empty mailbox".to_owned()),
            Some(msg) => match msg {
                TypedMessage::WorkloadMsg(workload) => Ok("Received WorkloadMsg".to_owned()),
                TypedMessage::SystemMsg(cmd) => Ok("Received SystemMsg".to_owned()),
                TypedMessage::ActorMsg => Ok("Received ActorMsg".to_owned()),
            },
        }
    }

    pub fn enqueue(&mut self, msg: TypedMessage) -> Result<(), String> {
        self.mails.push(msg);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<TypedMessage> {
        if self.mails.is_empty() {
            None
        } else {
            Some(self.mails.remove(0))
        }
    }
}

// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use crate::context::Context;
    use std::time;
    use uuid::Uuid;

    #[test]
    fn create_mailbox_test() {
        let (tx, rx) = mpsc::channel::<TypedMessage>(16);
        let uuid = Uuid::new_v4();
        let mut ctx = Context::new(uuid, tx);
        let mut mbx = Mailbox::new(rx);
        assert_eq!(mbx.mails(), vec![]);
    }

    #[test]
    fn send_among_mailboxes_test() {
        let mut map = FnvHashMap::default();

        let (tx_1, rx_1) = mpsc::channel::<TypedMessage>(16);
        let (tx_2, rx_2) = mpsc::channel::<TypedMessage>(16);
        let (id_1, id_2) = (Uuid::new_v4(), Uuid::new_v4());

        map.insert(id_1.clone(), tx_1.clone());
        map.insert(id_2.clone(), tx_2.clone());

        let mut ctx_1 = Context::new(id_1, tx_1);
        let mut ctx_2 = Context::new(id_2, tx_2);
        let mut mbx_1 = Mailbox::new(rx_1);
        let mut mbx_2 = Mailbox::new(rx_2);

        // Actor 1 sends to Actor 2
        // First, query the sender
        let new_tx = map.get(&id_2).unwrap();
        ctx_1.set_context(id_2.clone(), (*new_tx).clone());
        // Second, construct the message
        let msg = TypedMessage::SystemMsg(SystemCommand::CreateActor(4, String::from("raptor")));
        // Third, send the message
        ctx_1.send(msg);
        // Check whether Actor 2 received the message
        assert_eq!(mbx_2.len(), 0);
        let recv = mbx_2.receive();
        assert_eq!(mbx_2.len(), 1);
        assert!(recv.is_ok());
        let res = mbx_2.deal();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Received SystemMsg".to_owned());
    }
}
