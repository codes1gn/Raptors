# AI并行语义与执行模型研究  
该文档记录`AI并行语义与执行模型研究项目`从背调、立项、开发、最终至结题的完整流程。 

由 ERA(燧原科技研究院) 负责归纳，撰写，更新，与发布。 

![enflame icon](/img/enflame_logo.png "Enflame Icon" ){: style="height:50px;width:100px"}  ![era icon](/img/era_logo.png "ERA Icon" ){: style="height:50px;width:60px"}


## 第一部分: 并发并行技术分析

### 1.1 常见并行并发技术

* `Threads + locks` : pessimistic transactions.
* `Transactional memory` : optimistic transactions.
* `Functional programming` : shared immutable state.
* `Actor model` : unshared mutable state.
* `CSP` : unshared mutable state.
* `SPMD` : unshared mutable state.
* `Map-reduce` : shared immutable state.
* `Event-driven model` : unshared mutable state.
* `Grid computing` : pessimistic model + optimistic transactional    

| ![concurrency_model](/img/concurrency_model.PNG "Concurrency Model Analysis") |   
|:--:|  
| 并行并发技术总结 |

在 Shared Mutable State下，多线程一定会破坏原子性。又因为Shared State往往会导致较差的模块性，以及应对data race的防御手段。所以我们选择属于Unshared Mutable State类的技术。

---

## 第二部分: 典型框架分析

### 2.1 主流方案
当前的主流方案往往是：并行训练库 + AI框架的结合。

* 外层： 并行训练库  
* 内层： 计算框架  

| ![old_flowchart](/img/old_flowchart.png "两阶段分布式训练系统典型结构") |    
|:--:|  
| 两阶段分布式训练系统典型结构 |

* 优点：
    1. 实现简单，特别是基于SPMD形式对于现有计算框架进行并行扩展，基于MPI与CCL集成通信库即可实现。  
    2. 对于实现数据并行这种基本模式有着天然加成，对于并行训练库以及现有计算框架兼容性要求较低。
* 缺点：
    1. 扩展能力差或受限于计算框架特性。针对非数据并行的其他复杂模式，并行训练库能否实现的强依赖是计算框架本身的性质。
    2. 独立的软件模块（两阶段）虽然带来了实现的易用性，但是也造成了优化空间、优化visibility的缺失。
    3. 兼容性：上述的主流方案 是由互相配合的两个软件模块组成。因此不同的模块之间的接口兼容性带来了额外的技术挑战与维护成本。
    4. 自动化困难：两个阶段，两个只依赖接口对其的系统，实现自动化存在着原生的工程挑战。

---

### 2.2 前沿工作

#### i. Oneflow 
`Oneflow` 基于SBP语义 + Actor model 

* 优点：   
    * 贴近目前用户写model workloads的表达形式，可以expose到DSL层。
* 缺点：   
    * 需要比较复杂的dataflow analysis。
    * 需要用户手动标注。

| ![oneflow](/img/oneflow.png "OneFlow的SBP语义") |    
|:--:|  
| Oneflow的SBP语义 |

####  ii. Tenstorrent
芯片数据的传输会被抽象为纯的`消息传递`的方式。芯片的核之间以及芯片之间对于编译器与其他软件模块的抽象形式一致，大大降低了软件的构建复杂度。  

| ![tenstorrent](/img/tenstorrent.png "Tenstorrent") |    
|:--:|  
| Tenstorrent |

#### iii. Pathways

* 整体形式上仍然遵循两阶段的模式，第一阶段的并行训练库构建了完整的`scheduler`和`executor`的模式，支持不同的并行程序分片的调度和执行。而不是简单的基于MPI进行扩展。  
* SPMD模式到MPMD模式的转变。  
* Dynamic routing 动态路由的支持。  

| ![pathways](/img/pathways.jpg "Pathways") |    
|:--:|  
| Pathways结构图 |

---

### 2.3 自动并行

| ![autoparallel](/img/autoparallel.png "Autoparallel") |    
|:--:|  
| 自动并行相关工作总结 |

* `Cost-model Style`:  
基于`cost model`作为先验知识，尝试采用数学规划、集群优化等偏静态的优化算法计算出可能的策略。  
    * 优点：
        1. 每次提出的策略的质量相对较高。
        2. 优化策略需要的收敛轮数较少，甚至可以用one-off的方式不做循环搜索。
    * 缺点：
        1. 效果极其严重依赖于代价模型的质量，很多工作中的结论不具有可迁移性，主要原因就是代价模型本身的质量不可控。
        2. 相比于`on-the-fly`的方式，代价模型的质量与精度普遍较低。
        3. 数学规划类的算法优化效果较好，但单次求解耗时长。

* `On-the-fly Style`:
将运行系统当作封闭的黑盒，在实际运行中，基于统计规律将策略与性能采样之间建立关联。   
    * 优点：
        1. 通常配合启发式搜索算法，如随即搜索、网格搜索、贝叶斯优化等，单次策略循环求解很快。
        2. 数据完全实测，准确率高于建模形式的代价模型。
        3. 不需要太多的先验知识，将系统黑盒化本质上是一种层次特别高的抽象处理方式。
    * 缺点：
        1. 单次优化求解都有一次编译/增量编译的时间开销。
        2. 将性能模型当作黑盒来看待，过于舍弃先验知识，造成往往只能使用总迭代数较多的启发式优化算法。

---
## 第三部分: 经验吸取

1. 两阶段系统不利于支持任意的并行模式，倾向于in-system形式的设计，将并行的功能融合到编译流程和Runtime中。
2. 一阶段的设计模式拥有更好的兼容性、优化空间以及更通用。
3. 统一抽象的关键在于shared state的抽象形式统一。
4. Shared memory的形式在大颗粒度下很难做到，但message-passing的形式可以。
5. MPMD的模式是必要特性。
6. 尽可能考虑对dynamic routing的支持、或者留下对应的扩展接口/扩展可能性。
7. 结合`cost-model style`以及`on-the-fly style`，充分利用先验知识降低总迭代数，提高收敛速度；同时在运行时进行性能采样更新代价模型，保证策略的质量，以及更好的通用性。

