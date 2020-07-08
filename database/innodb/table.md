# innodb中的表

## innodb的逻辑存储结构
innodb每个表都默认有一个表空间（.idb），里面又有段、区、页组成。

innodb有一个共享的表空间（idbdata1），如果innodb_file_per_table没有开启，所有的表都会放在这个共享表空间里。

开启之后，表空间里会存放每个表的数据、索引、插入缓冲，而且其他信息依旧放在共享表空间里。

## 区
区是连续的页组成的，每个区的大小都是1MB，每个页的大小默认是16KB，即一个区会有64个连续的页。

innodb在创建一个表的时候，会用碎片页（6页）组成一个区（96KB），对于一些小的表，足够用来存储。如果这几页全部写满，则会开始连续申请1一个区的空间。

## 页
innodb中有很多种页：
1. 数据页，也是b+树的叶子节点（innodb中数据即索引）
2. undo页
3. 系统页
4. 事务数据页
5. 插入缓冲Bitmap
6. 插入缓冲空闲列表页
7. blob页
8. compressed blob页

### 数据页
innodb的存储是行存储，有Compact，Redundant，Dynamic等行记录格式。

#### Compact格式
对于Compact记录格式：

|变长字段长度列表|NULL标志位|记录头信息（40位）|数据...|
|:---:|:---:|:---:|:---:|

变长字段长度列表记录了变长字段如：varchar的单个字符长度，而且每个字符长度不能超过2字节。如果变长列的最大长度小于255字节，则变长字段长度列表里用一个字节来标记；反之，用两个字节标记。另外，innodb中varchar类型的最大长度限制是65535字节。

在记录头信息（record header）中，特别注意有个n_owned的字段，用来表示在该记录中**含有的记录数**，在后面介绍page directory的时候会用到。

#### Redundant格式
|字段偏移列表|记录头信息（48位）|数据...|
|:---:|:---:|:---:|

第一个字段变成了变长字段偏移列表，本质和Compact相同，但是是逆序摆放，而且是相对于上一个列的偏移值（不但是变长列，所有列都在里面）。对于NULL值，不是直接将其记录在头部，而是用全0代替。

#### 行溢出数据
blob、text、varchar等都有可能造成行溢出，即这类行对象不会放在行记录中，而是用单独的页来对其进行存放。当数据页不能存放两个或以上的行数据时，溢出的部分将会存放到单独的页中。

Compact和Redundant行记录会存放该对象768个前缀字节，其余部分存放到溢出页中，并在行中保留指针。而Dynamic，Compressed行记录格式则会将溢出的数据全部存放到溢出页，只保留指针。

### innodb的数据页结构
|file header(38byte)|page header(56byte)|Infimun + Supremum|user records|free space|page directory|file trailer|
|:---:|:---:|:---:|:---:|:---:|:---:|:---:|

file header用来记录页在表空间中的一些信息，如页的偏移量，上一个页，下一个页，lsn，页类型等。

而page header则保存了页内部的一些信息，如slot数量，可重用空间的指针，最后插入的记录位置等。

Infimun和Supremum是虚拟的记录，分别保存了比该页中任何一个主键都要小的值以及都要大的值，相当于页的记录边界。

page directory记录的是存放记录的相对位置。这些记录指针称为slot，这些slot是稀疏目录，即一个slot里面可能有几个记录。每个slot中的记录都是按照主键大小顺序排列的，每个slot中一般有[4, 8]个记录。当我们对某个槽进行插入/删除操作时，需要对这槽里的记录进行一些额外维护操作。我们在寻找某一个主键记录时，会先在page directory中寻找它的槽，找到这个槽的第一个记录后，我们再根据这条记录的next_record信息进行查找。提高了在页中的查找效率。

file trailer用来检测该页的完整性，判断其是否损坏。

## 约束
innodb提供了几种数据约束：
1. primary key
2. unique key
3. foreign key
4. default
5. not null

同时，还有几种约束机制：
1. 数据类型
2. 外键约束
3. 触发器

在对错误数据进行插入时，如将null插入到not null的列中，数据库不会报错，而是抛出一个warning。将sql_mode调整到STRICT_TRANS_TABLES之后会发生报错。

通过enum的约束也一样。

### 触发器
```sql
CREATE TRIGGER [trigger_name]
BEFORE | AFTER
INSERT | UPDATE | DELETE
ON [table_name]
FOR EACH ROW [trigger_stmt]
```

### 外键约束
在有外键约束的情况下，被引用的表叫做父表，引用的表叫做子表。当父表发生update或者delete的操作时，有4种子表的操作模式：
1. cascade，对子表进行相同的操作
2. set null，将该约束设置成null（not null报错）
3. no action，抛出错误
4. restrict，抛出错误，不允许此类操作（默认）

在创建外键约束时，可以通过on update/delete语句设置这几种操作。

## 视图
innodb的视图是虚表，即实际上只是根据视图定义来进行查询或更新操作。

在视图中定义with check option可以防止对视图的值进行错误更新。

### 物化视图
实际存在的表，一般有on demand和on commit两种更新方式。第一种是在用户查询时再更新视图；第二种则在原表更新的时候更新。

而更新方式也分为：fast、complete、force、never。fast是增量刷新，complete是完全刷新。

利用触发器，可以模拟物化视图的效果。

## 分区表
innodb支持水平分区。四种分区类型：
1. range：按照范围分区
2. list：离散范围分区，超出list里的范围会报错
3. hash：利用哈希函数进行分区
4. key：根据mysql提供的哈希函数分区

```sql
CREATE TABLE [table_name]
(
  paramlist
)
PARTITION by [PARTITION METHOD]
PARTITION p1 VALUE ...
...
```
分区列必须是唯一索引或者主键的一部分。

### 子分区
每个分区的子分区必须相同，子分区的名字不能重复。
```sql
CREATE TABLE [table_name]
(
  paramlist
)
PARTITION by [PARTITION METHOD]
PARTITION p1 VALUE ...
(
  SUBPARTITION s1,
  SUBPARTITION s2,
  ...
)
...
```
