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