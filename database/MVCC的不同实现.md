# MVCC的不同实现

- hyper MVCC
- SAP HANA
- CMU CICADA

## hyper MVCC

1. 列存储
2. multi-version storage：delta storage

### Precision Locking

1. first writer wins: 只有一个写者线程，后来的写者冲突abort
2. 在验证阶段：正在验证的事务和这个**事务开始之后committed的事务**日志验证（读写冲突）
   1. **precision locking**：只需要存这个事务的读谓词而不是整个读集
   2. 检查读谓词是否和修改日志相交
   3. 如果相交则说明正在验证的事务读到的版本违反串行化标准（read stability，读到的版本在事务结束时不可见）；快照隔离级别不需要验证（快照隔离允许write skew anomaly）

### Version Synposes

表（也可能是每个block）的meta-data中存了一个Version Synopsis的值，表示存在多版本的元组的开始位置和结束位置。除了这个范围的其他元组都可以连续读取；这个范围里的元组则需要检测是否存在多个版本，然后找到合适的版本。



## SAP HANA

1. 主表中存最旧的版本
2. 行存储+列存储
3. 主表中每个元组有一位存储是否存在新版本
4. 版本存储：哈希表+N2O
5. 每个事务存储了修改的内容，全局有一个group commit context，存储group commit的txn的id



## CMU CICADA

### BEST - EFFORT INLINING

1. 在表的meta-data中存每个元组最常访问的版本

### Contention-aware Validation (optional)

在验证的时候首先验证最近被修改的元组。在高冲突场景下才开启。

### Early Consistency Check (optional)

在写操作发生的时候先进行验证（而不是在验证阶段）。在高冲突场景下才开启。

### Incremental Version Search

从上次访问过的版本开始继续搜索（找不到之后再从头搜索）

### Index Node Table

将b+树索引存在一个表结构里，利用指针指向表的不同项。叶子节点存每个到元组的指针（RID）。普通的b+树存在堆中，可能在不同的页里，提高访问索引的读取速度（疑惑）。但修改索引的时候可能会导致大量的拷贝/移动带来的损耗。