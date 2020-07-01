# mysql中的文件
1. 参数文件
2. 日志文件
3. socket文件
4. pid文件
5. 表结构文件
6. 存储引擎文件

## 参数文件
`mysql --help | grep my.cnf`查看参数文件路径。如果找不到参数文件，mysql按照默认参数启动（编译时）。

在mysql实例中可以设置动态参数（静态参数无法设置）。利用`set`语句可以动态改变参数值。

`set @@session.xxx`设置当前会话的参数；`set @@global.xxx`设置当前实例的全局参数，但重新启动后会重新加载参数文件中的参数。

## 日志文件
`mysql.local.err`文件记录了mysql的错误信息，包括warning信息。

`binlog`是二进制日志文件，记录了mysql的修改记录，`select`和`show`语句不会记录在这里。直接打开是一个二进制文件，无法读取。可以在mysql的会话中：
```
show binlog events in 'binlog'\G;
```
查看binlog日志的事件。

也可以利用`mysqlbinlog`命令，将binlog日志打印出来：`mysqlbinlog -vv binlog -u root -p`。这里的`-vv`代表显示二进制记录的细节，否则仍是显示二进制内容。

另外，mysql还可以设置`binlog_format`参数：
- `statement`代表保存查询的sql语句
- `row`为默认格式，即上述提到的格式

## socket文件
`mysql.sock`，可以用来连接一个Unix下的mysql实例。

## pid文件
记录了mysql在系统中运行的pid

## 表结构定义文件
在innodb8.0以前，每个表空间都有一个`.frm`和`.idb`文件，前者保存了表的结构。

## 存储引擎文件

### 表空间文件
`ibdata1`，是一个自动增长的文件。在参数`innodb_file_per_table`设置为`ON`之后，每个表都会保存在分开的目录下。

### redo log
`ib_logfile1`和`ib_logfile2`，1写满之后切换到2，循环往复。可以开启`innodb_mirrored_log_groups`，使用日志镜像，提高可用性。`innodb_log_file_size`不能设置过小，否则很容易引起错误。