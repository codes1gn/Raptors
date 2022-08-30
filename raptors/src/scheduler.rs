use log::{debug, info};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use uuid::Uuid;

use crate::mailbox::*;
use crate::messages::*;
use crate::system::*;
use crate::workloads::*;

// Util function that randomly allocates an executor to each workloads
// Take vector of workloads, produce vector of envelope
// pub fn pre_schedule(system: &ActorSystem, workloads: Vec<TypedMessage>) -> Vec<Envelope> {
//     let mut actor_ids = Vec::from_iter(system.actor_registry().keys());
//     let mut rng = rand::thread_rng();
//     let die = Uniform::from(0..system.actor_registry().keys().len());
//     workloads
//         .into_iter()
//         .map(|wkl| -> Envelope {
//             let index = die.sample(&mut rng);
//             debug!("receiver's actor index = {:#?}", index);
//             Envelope {
//                 msg: wkl,
//                 receiver: Address::new(*actor_ids[index]),
//             }
//         })
//         .collect::<Vec<Envelope>>()
// }
