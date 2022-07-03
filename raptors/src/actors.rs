// LICENSE PLACEHOLDER
// test
use crate::messages::{self, ActorMsg};
use crate::executor;

use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
//use uuid::Uuid;

//===----------------------------------------------------------------------===//
/// Status of Actors
//===----------------------------------------------------------------------===//
#[derive(Debug)]
pub enum Status {
    Stop,
    Done,
    Wait,
    Reset,
}

impl Status {
    pub fn stop<T>(state: T) -> (T, Status) {
        (state, Status::Stop)
    }

    pub fn done<T>(state: T) -> (T, Status) {
        (state, Status::Done)
    }

    pub fn wait<T>(state: T) -> (T, Status) {
        (state, Status::Wait)
    }

    pub fn reset<T>(state: T) -> (T, Status) {
        (state, Status::Reset)
    }
}
/* 
//===----------------------------------------------------------------------===//
/// Errors returned by Actors
//===----------------------------------------------------------------------===//
pub enum AidError {
    CantConvertToBincode,
    CantConvertFromBincode,
    ActorAlreadyStopped,
    AidNotLocal,
    SendTimeOut(Aid),
    UnableToSchedule,
}

impl std::fmt::Display for AidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AidError {}

//===----------------------------------------------------------------------===//
/// ActorSender
/// An Actor uses the sender to send messages to the destination actor. Messages
/// sent to actors running on this actor system are wrapped in an Arc for efficiency.
//===----------------------------------------------------------------------===//
enum ActorSender {
    // Although we share the same abstraction between inter & intra systems,
    // deal with messages in different ways
    Local {
        stopped: AtomicBool,
        sender: SeccSender<Message>,
        system: ActorSystem,
    },

    Remote { sender: SeccSender<WireMessage>},
}

impl std::fmt::Debug for ActorSender {
    fn fmt(&self, formatter: &'_ mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match *self {
                ActorSender::Local { .. } => "ActorSender::Local",
                ActorSender::Remote { .. } => "ActorSender::Remote",
            }
        )
    }
}

//===----------------------------------------------------------------------===//
/// Inner data of an Actor
//===----------------------------------------------------------------------===//
struct ActorData {
    uuid: Uuid,
    system_uuid: Uuid,
    name: Option<String>,
    sender: ActorSender,

}
*/
//===----------------------------------------------------------------------===//
/// Actors:
///     1. DummyActors fns
///      2. RealActors fns
//===----------------------------------------------------------------------===//
#[derive(Debug)]
pub struct Actor {
    id: usize,
    live: AtomicBool,
}

impl Actor {
    pub fn new(id: usize, live: AtomicBool) -> Actor {
        return Self {
            id: id,
            live: live,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn alive(&self) -> bool {
        self.live.load(Ordering::Relaxed)
    }

    pub fn getAddr(actor: Actor) -> usize {
        actor.id
    }
    
    // TODO: make it message passing, test with inter-threads
    // TODO: gradually support higher granularity parallelism
    pub fn receiveDummy(&self, msg: messages::DummyWorkload) -> () {
        self.on_compute(msg);
    }

    pub fn receiveMsg(&self, msg: ActorMsg) {}

    pub fn sendMsg(&self, msg: ActorMsg) {
        
    }

    fn on_compute(&self, workload: messages::DummyWorkload) -> () {
        workload.mock_run();
    }
}





// unit tests
#[cfg(test)]

mod tests {
    use super::*;
    use std::{thread, time};

    // test visibility
    #[test]
    fn create_dummy_workload_test() {
        let load = messages::DummyWorkload::new(16);
        assert_eq!(load.payload(), 16 as usize);
    }

    /// Doc test for actors
    /// ```
    /// let load = messages::DummyWorkload::new(16);
    /// let now = time::Instant::now();
    /// load.mock_run();
    /// assert!(now.elapsed() >= time::Duration::from_millis(16));
    /// ```
    #[test]
    fn workload_mock_run_test() {
        let load = messages::DummyWorkload::new(16);
        let now = time::Instant::now();
        load.mock_run();
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }

    #[test]
    fn query_actor_id() {
        let actor = Actor::new(17, AtomicBool::new(true));
        assert_eq!(actor.id(), 17);
    }

    #[test]
    fn receive_workload() {
        let actor = Actor::new(1, AtomicBool::new(true));
        let load = messages::DummyWorkload::new(16);
        let now = time::Instant::now();
        actor.receiveDummy(load);
        assert!(now.elapsed() >= time::Duration::from_millis(16));
    }
}
