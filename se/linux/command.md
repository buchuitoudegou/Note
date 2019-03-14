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
* stat(name, ptr) 第一个参数是文件名，第二个是buffer指针，这个指针指向一个struct stat类型。
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