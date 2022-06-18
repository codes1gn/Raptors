### Conceptual Model of the Actor System

TODO: current all parts are fused together, we shall split this doc Into paragraphs later on
* comparison between other concurrency models, the tradeoffs of picking this
* conceptual Model of actor model
* review of impl modeling of typical works
** actix
** bastion
** axioms
** Riker
** etc

Attempt to keep the core module as compact as possible and make implementation flatten and straight-forward.

We make the abstraction according to the following axioms assumed:
* Systems:
Riker: 
Systems are structs
ProtoSystem: config, log, execute
LoggingSystem: log, logger_guard
ActorSystem: 
  ProtoSystem
  LoggingSystem
  Executors
  Channels
  Timer
Mailbox with MessageQueue
Envelope: Message + SenderAddr + ReceiverAddr
Actor: trait for actor, that defines the actor trait behaviours
  pre_start
  post_start
  supervisor_strategy
  sys_recv: receive system message
  recv: receive actor/executor Message that impl Receive Trait to bound
Receive<MsgType>: trait for Message, that defines the behaviours when an actor receive this message, that do they do with it
  receive: the concrete logics
MailboxSchedule Traits
  set_scheduled
  is_scheduled

Axiom:
Status enum: result of processing a msg
  Done
  Skip
  Reset
  Stop
Sender enum: tx,rx, the sender side of a msg-passing channel
  Local
  Remote
ActorID struct:
  uuid
  uuid of the belonging system
  name
  sender: sender of the channel pair? why not express addr and mailbox
Context struct with immutable info, infos that required by actor to processing messages
  System
  AID
Processor trait: takes states, context and message as input and process it
ActorBuild struct
  System
Actor:
  Context
ActorStream: process message/results for actor while receiving msg
Executor struct: high-level scheduling of actors, when actor created, it wrapped by a task, then enqueue to the sleeping_queue
more like scheduler role, that queueing and ordering workloads, awaking actors, and 
Reactor: manage a set of actors, and poll the available actor for working


Bastion:
MessageHandler:
  on_ask
  on_tell
  on_broadcast
  try_into_ask
  ...
Actor, named after child and children:
  handle (Envelope)



* Actors:
* Messages:
Riker: use enum to wrap over structs and make it specific hierarchy; use Into trait to make enums can convert/promote to higher level;
System Msg
  ActorInit
  SysCmd
    Stop
    Restart
  SysEvent
    struct ActorCreated
    struct ActorRestarted
    struct ActorTerminated
  SysError: make errors with enums and with hierarchy
    xxxError -> string
    xxxError -> string

Axiom:
part 1: actor message
ActorMessage Trait
  to_bincode
  from_bincode
MessageData struct -> 1:1 mapping to actual data
  hash id: uuid
  MessageContent
Message: Arc<MessageData> -> N:1 mapping as ref to data; passing refs are fast and trackable
MessageContent enum
  Local with ActorMessage MsgType
  Remote with Vec<u8>

part 2: system message
SystemMsg
  start
  stopo
  stopped
WireMsg
  HelloAck
  ActorMsg

Error design, same as Riker

Bastion:
Error enum:
  sender + receiver
MsgInner enum:
  Bcst
  Tell
  Ask
Envelope | SignedEnvelope
Addr





* Supervisors:
* Scheduler:


Key Challenges:
1. async message passing, with Futures, Polls and related rules
2. communicating intra-dev, inter-dev.

