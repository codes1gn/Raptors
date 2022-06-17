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
Riker: Systems are structs
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

Traits:
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





* Supervisors:
* Scheduler:

