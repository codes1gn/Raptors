// LICENSE PLACEHOLDER
use uuid::Uuid;

use crate::messages::*;
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address(Uuid);

impl Address {
    pub fn new(uuid: Uuid) -> Address {
        Address(uuid)
    }

    pub fn into_aid(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Envelope {
    pub msg: LoadfreeMessage<MockTensor>,
    // sender: Address,
    pub receiver: Address,
}

// wrap a dedicated executor module that only consider how to do computations
//
// TODO(long-term):
// as a interface, make refactor as Trait and expose to CRT level,
// make CRT vm to impl this trait
#[derive(Debug, PartialEq, Eq)]
pub struct Mailbox {
    // TODO replace with ringbuffer or deque
    // references:
    // *** circular-queue
    // *** ringbuffer
    // *** ringbuf
    // *** ring_queue
    mails: Vec<LoadfreeMessage<MockTensor>>,
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
        return Self { mails: vec![] };
    }

    fn mails(&self) -> Vec<LoadfreeMessage<MockTensor>> {
        self.mails.clone()
    }

    pub fn enqueue(&mut self, msg: LoadfreeMessage<MockTensor>) -> Result<(), String> {
        self.mails.push(msg);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<LoadfreeMessage<MockTensor>> {
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
}
