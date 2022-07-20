// LICENSE PLACEHOLDER
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::messages::*;

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
    tx: Sender<TypedMessage>,
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
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<TypedMessage>(16);
        return Self {
            mails: vec![],
            tx: tx,
            rx: rx,
        };
    }

    pub fn mails(&self) -> Vec<TypedMessage> {
        self.mails.clone()
    }

    pub fn sender(&self) -> Sender<TypedMessage> {
        self.tx.clone()
    }

    pub fn set_sender(&mut self, sender: Sender<TypedMessage>) {
        self.tx = sender;
    }

    pub fn send(&self, msg: TypedMessage) {
        self.tx.try_send(msg);
    }

    pub fn receive(&mut self) -> Result<(), String> {
        let msg = self.rx.try_recv();
        if msg.is_ok() {
            self.enqueue(msg.unwrap())
        } else {
            Err("Error!".to_owned())
        }
    }

    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut mbx = Mailbox::new();
    /// let msg = TypedMessage::SystemMsg(SystemCommand::CreateActor(4, String::from("raptor")));
    /// assert_eq!(mbx.len(), 0);
    ///
    /// mbx.send(msg.into());
    /// let recv = mbx.receive();
    /// assert_eq!(mbx.len(), 1);
    /// assert!(recv.is_ok());
    ///
    /// let res = mbx.deal();
    /// assert!(res.is_ok());
    /// assert_eq!(res.unwrap(), "Received SystemMsg".to_owned());
    /// ```
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

    /// ```
    /// use raptors::prelude::*;
    /// use raptors::mailbox::Mailbox;
    ///
    /// let mut mbx = Mailbox::new();
    /// let msg = SystemCommand::CreateActor(4, String::from("raptor"));
    /// mbx.enqueue(msg.into());
    /// assert_eq!(mbx.len(), 1);
    /// ```
    pub fn enqueue(&mut self, msg: TypedMessage) -> Result<(), String> {
        self.mails.push(msg);
        Ok(())
    }

    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut mbx = Mailbox::new();
    /// let msg1 = SystemCommand::CreateActor(4, String::from("raptor"));
    /// let msg2 = SystemCommand::CreateActor(2, String::from("raptor"));
    /// mbx.enqueue(msg1.into());
    /// mbx.enqueue(msg2.into());
    /// let msg = mbx.dequeue();
    /// assert_eq!(mbx.len(), 1);
    /// assert_eq!(msg.is_some(), true);
    /// assert_eq!(msg.unwrap(), SystemCommand::CreateActor(4, String::from("raptor")).into());
    /// ```
    ///
    /// test dequeue from empty message queue
    /// ```
    /// use raptors::prelude::*;
    ///
    /// let mut mbx = Mailbox::new();
    /// let msg = mbx.dequeue();
    /// assert_eq!(msg.is_none(), true);
    /// ```
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
    use std::time;

    #[test]
    fn create_mailbox_test() {
        let mbx = Mailbox::new();
        assert_eq!(mbx.mails(), vec![]);
    }

    #[test]
    fn send_among_mailboxes_test() {
        let mut m1 = Mailbox::new();
        let mut m2 = Mailbox::new();
        m1.set_sender(m2.sender());
        // Now test whether m2 could receive the msg from m1
        let msg = TypedMessage::SystemMsg(SystemCommand::CreateActor(4, String::from("raptor")));
        m1.send(msg);

        let recv = m2.receive();
        assert_eq!(m2.len(), 1);
        assert!(recv.is_ok());

        let res = m2.deal();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Received SystemMsg".to_owned());
    }

    #[test]
    fn send_recv_test() {
        let mut mbx = Mailbox::new();
        let msg = TypedMessage::SystemMsg(SystemCommand::CreateActor(4, String::from("raptor")));
        assert_eq!(mbx.len(), 0);

        mbx.send(msg);
        let recv = mbx.receive();
        assert_eq!(mbx.len(), 1);
        assert!(recv.is_ok());

        let res = mbx.deal();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Received SystemMsg".to_owned());
    }
}
