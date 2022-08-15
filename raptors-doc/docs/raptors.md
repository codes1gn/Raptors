# Raptors原型系统  ![raptors icon](/img/rapicon.jpg "Raptors Icon" ){: style="height:50px;width:60px"}

`Raptors`将为`AI并行语义与执行模型研究项目`提供高性能网络支持，运行时系统调度支持，自动并行，以及最终将集成至`Chopper框架`。

## 1. Why Actor Model?   

* 集群间通过消息传递进行操作，对于集群内的Shared memory，也可以通过message-passing提升抽象层次，从而达到抽象的统一。而且Actor model能够和MPMD的模式有很好的结合。  

* Actor model有着较高的系统容错性以及较低的纠错难度。可以构建专用的纠错策略、生成用于系统监控、检测的专用actor node。

* Actor model适合大规模分布部署，对于水平扩容支持高，适应当下超大规模模型训练的潮流。

---

## 2. 其他开源项目
* [Riker](https://github.com/riker-rs/riker)
    * Actor supervision to isolate and recover from failures
    * Concurrency built on ```futures::execution::ThreadPool```
    * Publish/Subscribe messaging via actor channels
    * Message scheduling
    * Out-of-the-box, configurable, non-blocking logging

* [Actix](https://github.com/actix/actix)
    * Async and sync actors
    * Actor communication in a local/thread context
    * Uses [futures](https://crates.io/crates/futures) for asynchronous message handling
    * Actor supervision
    * Typed messages (No ```Any``` type)

* [Axiom](https://github.com/rsimmonsjr/axiom)
    * Async and sync actors
    * Actor messages distinguished into ```local/remote```
    * Location agnostic

* [Xactor](https://github.com/sunli829/xactor)
    * Async actors
    * Actor communication in a local context
    * Using [futures](https://crates.io/crates/futures) for asynchronous message handling
    * Typed messages (No ```Any``` type), generic messages allowed

* **What we learned:**
    * Actor supervision, important to isolate and recover from failues
    * Async and sync actors
    * Actor commuication in a local context
    * Typed messages (No ```Any``` type)
    * Existing network frameworks are too heavy for us. We need to build our own dedicated network framework
---

## 3. Raptors中消息类型系统

网络中的消息被区分为两类：`SystemMessage` 以及 `ActorMessage`。

* SystemMessage: 
    1. SystemActionMsg：用来控制系统的启动/停止，actor的创建/销毁等。
    2. SystemErrorMsg：System层级的错误消息。

* ActorMessage: 
    1. ActorActionMsg：actors间的消息，例如用来传递数据与代码等。
    2. ActorErrorMsg：Actors层级的错误消息。

| ![message_types](/img/messages.png "AMessage Types") |   
|:--:|  
| Message Definition In Actor Model | 
---
## 4. Raptors中网络模型设计  

| ![actor_model](/img/actor_model.png "Actor Model Design") |   
|:--:|  
| Actor Model Design in Raptors |  

整个系统分别System层和Actors层。System层有着单独的，具有特化作用的Actor: ```Profiler``` & ```Logger```。  

* `System`负责系统的启动/停止，以及管理其他所有特化/非特化actors。

* `Profiler`在运行时采样性能数据，负责代价模型的更新。

* `Logger`记录整个系统的运行时日志。

* `Actors`被创建负责具体计算，内部间互相通过`Message`来传递数据与消息。

---

## 5. Context的设计

---

## 6. Profiling的颗粒度

```Profiler```需要其他Actors向它传递消息表明每次计算操作的运行时间，内存占用等信息。但是收集这样的信息，需要Kernel以及硬件方面的支持。因为Actors只能得到```workload```层级的信息。

---

## 7. 自动并行: 基于Workload层级优化

现在```Profiler```只能得到workload层级的计算信息来更新损失模型，所以我们的并行策略也将在workload这个维度进行计算与优化。
