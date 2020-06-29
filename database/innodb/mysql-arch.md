# mysql innodb 基本架构

## 多线程模型
1. Master Thread：负责将缓冲池中的数据异步刷新到磁盘；合并插入缓冲（Insert Buffer）；回收Undo页（MVCC）。
2. IO Thread：负责异步IO。用innodb_read_io_threads和innodb_write_io_threads进行设置。
3. Purge Thread：回收undo页。用innodb_purge_threads=1来启动Purge Thread。
4. Page Cleaner Thread：刷新脏页。

## 内存

### 缓冲池
1. innodb_buffer_pool_size设置缓冲池的大小
2. 缓冲池中有：索引、数据、undo、insert buffer、自适应哈希索引、锁信息、数据字典新消息...
3. innodb 1.0x之后可以拥有多个缓冲池，对每个数据页进行哈希分配到不同的缓冲池中。通过innodb_buffer_pool_instances配置。

### LRU LIST、Free LIST、Flush LIST
1. 缓冲池通过LRU算法管理，每个页的大小默认为16KB。和普通的LRU的区别在于，新页被pin时，插入到LRU列表的中间（可以用innodb_old_blocks_pct控制位置）。
2. 如果用普通的LRU算法，对整张表进行扫描的时候很容易将热点的页清理出缓冲池，而扫描的页并不是热点页。innodb利用innodb_old_blocks_time参数判断一个页放在中间之后需要等待多久才能放到LRU LIST的热端。
3. 当数据库刚启动，LRU LIST是空的，需要读取数据的时候，首先去Free LIST寻找是否有空闲的页；如果没有，则根据LRU算法将列表末尾的页删掉。
4. Flush LIST管理脏页。脏页可能既存在LRU LIST也存在Flush LIST。
5. select * from information.INNODB_BUFFER_POOL_STATS查看缓冲池状态
6. innodb1.0.x之后支持压缩页功能。16KB的页可以压缩成8、4、2、1KB。对于非16KB的页利用unzip_LRU来管理。

### 重做日志缓冲
redo log buffer。按照一定频率写入到重做日志文件。一般不需要设置很大，每一秒都会将重做日志缓冲写入到日志文件，只需要保证1秒钟产生的日志小于缓冲。用innodb_log_buffer_size控制。

1. Master Thread每秒钟都会写入重做日志文件。
2. 每个事务提交成功之前都要把重做日志写入到重做日志文件。
3. 当重做日志缓冲池的剩余空间小于1/2时，强制刷新重做日志文件。

## checkpoint
1. 缩短数据库的恢复时间
2. 缓冲池不够用时将脏页刷新到磁盘
3. 重做日志不可用刷新脏页

### Sharp checkpoint
在数据库关闭的时候，将所有的脏页刷回磁盘（innodb_fast_shutdown=1）。

### Fuzzy checkpoint
数据库运行时，将一部分脏页刷回磁盘。
1. Master Thread checkpoint：每秒/每10秒将一部分脏页刷回磁盘。
2. FLUSH_LRU_LIST checkpoint：innodb引擎需要保证LRU中有至少100个空闲页，在1.2.x版本中，有Page Cleaner线程专门检查。用innodb_lru_scan_depth来控制至少的空闲页。
3. Async/Sync Flush checkpoint：重做日志文件不可用，需要强制将一些页刷回磁盘。
4. Dirty Page too mush checkpoint：脏页太多，需要强制进行checkpoint。用innodb_max_dirty_pages_pct控制。

## Master Thread
由4个循环组成：loop、background loop、flush loop和suspend loop。Master Thread会根据数据库运行的状态在这4个循环之间切换。

### loop
主循环。分为两部分，每秒钟操作一次和每10秒操作一次。每秒钟的操作：
1. 日志缓冲刷新到磁盘
2. 合并插入缓冲操作（如果前一秒的IO次数少于5次）
3. 刷新100个脏页到磁盘（判断buf_get_modified_ratio_pct，超过innodb_max_dirty_pages_pct则会写100个脏页，也就是Dirty Page too much checkpoint）

每10秒的操作：
1. 刷新100个脏页（如果上一个10秒的io次数少于200次）
2. 合并5个插入缓冲
3. 日志缓冲刷新到磁盘
4. full purge（删除没用的undo页）
5. 刷新100个或者10个脏页到磁盘（buf_get_modified_ratio_pct大于70则要写100个）

### background loop
如果没有用户操作发生，数据库会切换到这个模式。
1. 删除undo
2. 合并20个插入缓冲
3. 跳到flush loop

### flush loop
1. 刷100个脏页到磁盘
2. 如果buf_get_modified_ratio_pct超过innodb_max_dirty_pages_pct，继续执行这个循环。

### suspend loop
挂起，停止循环，直到有用户操作。

### 1.2.x之后的innodb
由于SSD的出现，硬盘的IO性能提高，在一些写入密集的场景，写入100个脏页，合并20个插入缓冲可能会浪费磁盘的IO性能，这时候我们可以设置一个innodb_io_capacity的参数，代表硬盘的io性能。而将之前写死的参数100，10改成：
1. 100% innodb_io_capacity
2. 10% innodb_io_capacity

IO次数的判断也可以改成innodb_io_capacity相关的参数。

而innodb_max_dirty_pages_pct在一开始1.0.x版本的默认值是90，这会让系统里的脏页非常多，让恢复性能变差。并且引入innodb_adaptive_flushing，引擎会通过buf_flush_get_desired_flush_rate函数来判断应该刷新的脏页数量，并且在脏页比例小于max_dirty_pages_pct时也会刷一定数量的脏页。

在1.2.x中还引入了innodb_purge_batch_size来控制purge线程每次回收的undo页数量。

## innodb的关键特性
1. 插入缓冲
2. 两次写
3. 自适应哈希
4. 异步IO
5. 刷新邻接页

### 插入缓冲
innodb的主键索引是聚簇索引，插入时是顺序索引，而辅助索引是非聚簇索引，在插入的时候会造成大量的随机IO。innodb提出插入缓冲的机制，让插入的辅助索引键值插入到内存，等到一个特定的时候再进行合并：
1. 这个辅助索引页进入缓冲池
2. Insert Buffer Bitmap追踪到辅助索引页的空间不足，强制合并
3. Master Thread每秒或10秒

Insert Buffer在内部是一棵B+树，它的搜索键是辅助索引对应的表的space id和这个辅助索引页的offset。另外innodb维护了Insert Buffer Bitmap来追踪每个辅助索引页的状态，每个索引页在里面占4bit：
1. IBUF_BITMAP_FREE：2bit，代表剩余空间
2. IBUF_BITMAP_BUFFERED：1bit，辅助索引页是否有插入缓冲
3. IBUF_BITMAP_IBUF：这一页是否是Insert Buffer B+树

### 两次写
在操作系统崩溃时，如果数据库刚好在将脏页写入文件系统，则有可能会导致写入一半的情况而导致文件损坏，无法用恢复算法将数据库恢复到一致的状态。

因此innodb引入两次写策略：
1. 在内存维护一个2MB的doublewrite buffer
2. 在磁盘维护一个2MB的共享空间
3. 在写脏页的时候先将脏页用memcpy写到doublewrite buffer
4. 然后分两次从buffer顺序写入到共享空间里
5. 调用fsync函数同步共享空间的数据和表数据

这时候，如果操作系统发生崩溃，导致文件损坏，则可以在共享空间中找到它的副本。

### 自适应哈希
innodb给热点页和查询模式建立一个哈希表。这里的查询模式必须是固定的，且重复多次。哈希查询不支持顺序查询。

### 异步IO
同步IO会阻塞系统，而异步IO则可以同时开启，并且，Linux和Windows提供原生的AIO支持，在多个IO发生时，可以合并。

MacOS上，innodb会模拟异步IO的过程。

### 刷新邻接页
在刷某个脏页时，检测同一个分区的其他页是不是脏页，如果是则一起刷新。

### 启动、关闭、恢复
innodb_fast_shutdown影响innodb的关闭，默认值为1：
1. 0表示在数据库关闭的时候，要完成full purge和merge insert buffer，并且将所有的脏页刷回硬盘。
2. 1表示在数据库关闭的时候不需要full purge和merge insert buffer，但还是要刷脏页。
3. 2表示在数据库关闭的时候不需要full purge和merge insert buffer，也不刷脏页，只需要将日志写回硬盘。

innodb_force_recovery影响innodb的恢复，默认值为0，表示如果需要恢复则进行所有的恢复操作。当它大于0时有6个取值：
1. 1：忽略所有的corrupt页
2. 2：阻止Master Thread
3. 3：不进行事务的回滚
4. 4：不进行插入缓冲的合并
5. 5：不查看undo log
6. 6：不进行前滚操作