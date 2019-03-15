# cp
usage: cp src_file target_file

* open(filename, how)
* creat(filename, mode)
* read(fd, buffer, amt) // amt是buffer的大小
* write(fd, buffer, amt)
* close(fd)
* perror(string)在标准错误显示信息中显示出相应的错误信息

# ls
usage:
```
ls /tmp 列出tmp文件夹的所有文件和目录
ls -l docs 列出docs目录各文件的属性
ls *.c 显示与*.c相匹配的文件

ls -a 列出包括.开头的文件
ls -lu 显示最后访问的时间
ls -s 显示以块为单位的文件大小
ls -t 按时间排序
ls -F 显示文件类型
```
* DIR* 目录指针
* struct dirent 目录项目结构体
* opendir(dirname) 打开一个项目
* readdir(DIR*) 读一个项目，返回值是struct dirent最后一个返回NULL
* closedir(DIR*) 关闭目录

```
struct stat类型
st_mode 文件类型和许可权限（16位的二进制数）
st_uid 用户所有者ID
st_gid 所属组ID
st_size 所占字节数
st_nlink 文件链接数
st_mtime 文件最后修改时间
st_atime 文件最后访问时间
st_ctime 文件属性最后改变时间
```

# chmod
修改文件模式.
```
usage:
chmod(char* path, mode_t mode)
path文件名，mode新的许可权限和特殊属性
返回-1错误，0成功
```

# chown
修改文件所有者和组
```
usage:
int chown(char* path, uid_t uid, gid_t gid)
path文件名，uid新的用户id，gid新的组id
返回-1错误，返回0成功
```

# rename
```
usage:
int rename(char* old, char* new)
old旧文件名，new新文件名
返回-1错误，返回0成功
```

# mkdir
```
usage: 
int mkdir(char* path, mode_t mode)
path新目录名 mode权限位掩码
-1错误，0成功
```

# rmdir
```
usage:
int rmdir(const char* path)
-1错误，0成功
```

# rm
```
usage:
int unlink(const char* path)
path需要删除的链接名
-1错误，0成功
```

# ln
创建一个链接（快捷方式）
```
usage:
int link(const char* orig, const char* new)
-1错误，0成功
```

# mv
```
usage:
int rename(const char* from, const char* to)
from原始链接的名字，to新建链接的名字
-1错误，0成功
```

# cd
```
usage:
int chdir(const char* path)
改变当前目录
-1错误，0成功
* stat(name, ptr) 第一个参数是文件名，第二个是buffer指针，这个指针指向一个struct stat类型。
```

# pwd
指令pwd的实现。pwd指令可以显示当前目录的完整路径。在linux的文件系统中，无论是目录还是文件，都是一个inode节点，利用系统调用get_inode可以得到某个文件的inode编号。

同时，"."和".."是每个目录中都有的节点。在普通目录中，".."指向自己的上一层目录，而根目录的".."则指向自己。

既然有这样的规定，我们只需要在当前目录，不断往前找就可以找到指向自己的目录，即根目录。这里还需要用到进入上一层目录的函数：chdir("..")
```
usage:
int chdir(const char* path)
进入目录
-1错误，0成功
```
因此，pwd的流程大概是：

找出当前目录的ino(利用stat函数)->进入上一层文件夹->利用这个ino找到这个文件夹的名字，并记录下来->并重复这样的操作->找到根目录之后，开始打印文件夹的名字.