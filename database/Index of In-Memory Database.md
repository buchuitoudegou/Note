# Index of In-Memory Database

## BW-Tree

Bw tree是微软提出的一种不需要加锁的索引结构，为SQL Server的存储引擎Hakaton提供高性能索引。传统的B+ tree索引在并行环境下会使用coupling locking的策略，或者versioned latch coupling（一种乐观锁，写者不会block读者），但在In-Memory的环境中，数据库系统的瓶颈从以往的I/O瓶颈变成了并行控制/cache missing等，尽管不同的锁的实现在数据库系统中的开销各不相同，但是，避免出现锁仍然是一种常见的优化策略。Bw tree就是一种无锁的并行索引结构。

在并行数据库系统中，我们需要解决的冲突是**读写冲突**和**写写冲突**。后者很好理解，即两个写者不能同时更新同一个节点的数据，在乐观并行控制协议中，防止这类冲突的做法比较简单：让先进行写的worker获胜，而另一个worker则需要abort，在写冲突并不严重的场景下，这样的策略能获得不错的效果；而这里的读写冲突，和事务层级上的读写冲突并不相同。在两个worker同时访问同一个索引的时候，一个读者从根开始（假设索引是传统的无锁B+ tree）遍历，寻找它需要的key，于此同时，一个写者则希望改写/删除这个key。这时，就有可能：读者得到一个叶子节点的指针时，这个叶子节点可能因为刚刚的删除操作而导致和别的节点合并，也就是说，读者得到的指针可能指向一个不合法的位置，从而导致错误。类似的错误可能还有很多。

为了解决这样的问题，Bw tree应运而生。

### 基础概念

Bw tree的基本结构和B+ tree相似，区别在于：

- Mapping Table
- Base Nodes and Delta Chains

先介绍Mapping Table。传统的B+ tree中，节点和节点之间用指针连接，这里的指针是物理指针，直接指向一个内存块。而在Bw tree中，节点之间存的是逻辑指针，即指向某个节点对应的page-id。而我们要访问这个节点，则需要在Mapping Table中找到这个page-id对应的物理位置，再进行寻址。这样做的好处在于，当我们产生一个新修改过的页时，它的父节点、兄弟节点都不需要进行指针的修改，只需要在Mapping Table中修改逻辑指针指向的新的具体物理位置即可。而这个操作，可以利用CaS（compare-and-swap）进行，这个命令是原子命令（atomic primitive）。

### 插入/删除/更新Key

如图，当一个写者进入，要插入一个Key时：

1. 创建一个delta log
2. 用CaS命令将Mapping Table中的指针指向的值，修改成新delta log的地址。

这样做的好处是，读写冲突时，不需要加锁，读者可以直接读到原来的base node。当两个写者同时进行CaS时，因为CaS的原子性，只有一个写者能比较成功，并进行替换，另一个写者则需要abort。

当一个读者找到Mapping Table中指针指向的值时，读者可以通过delta log中，某个meta-data来判断这是一个delta log还是一个base node。同时，在读取多个delta log时，读者需要维护一个Present集合$S_{present}$和一个Delete集合$S_{delete}$，来判断哪些键值已经被删除。因为delta log的读取顺序是从最近到最远，所以先读到的操作是有效的，即使它在后续的delta log中被撤销。具体流程：

1. 当找到一个没有出现在$S_{delete}$集合的插入值$V$，我们把它加入到$S_{present}$
2. 当找到一个没有出现在$S_{present}$集合的删除值$V$，我们把它加入到$S_{delete}$

最后对我们可见的键值集合：$S_{present} \cup (S_{base} - S_{delete})$。

### 分裂/合并

Bw tree和B+ tree一样，当某个节点已经满了，需要分裂。但这个过程并不是马上执行的，和插入数据一样，会在base node的delta chain的末尾插入一个**分裂**的delta log，这个log有一个逻辑指针（需要在Mapping Table找到物理位置）指向新分裂出的node。在父节点处我们也可以插入一个Separator的delta log帮助更快速的找到这个新的node。

而合并和分裂相似，插入**合并**的delta log，指向合并的node。需要注意的是，无论是无论是合并还是分裂，都不会马上进行**物理**上的合并和分裂，父节点的逻辑指针仍然指向原来的页。

### 合并delta chain

当某个节点的delta chain过长时，会给我们读/写这个节点造成很大的不便，因此，当delta chain超过一定阈值时，我们就要对其进行合并。合并的过程比较简单：

1. 创建新的节点
2. 从最远的（离base node最近）的delta log开始重新执行
3. 用CaS命令，将Mapping Table中原来的物理指针指向的节点指向新的node
4. 将原来的记录标识为垃圾，等待后台GC线程回收

这里需要提到，在合并delta chain的过程中，如果有线程写了这个节点，会导致CaS失败。这时候，我们可以选择重新执行整个合并的过程，也可以对此做一定的优化：从失败的地方开始找新的delta log。



