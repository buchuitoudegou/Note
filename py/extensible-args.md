# 函数可扩展参数列表

## *args
表示可以在函数后面加入任意数量个参数。

```python
def argsFunc(a, *args):
	print a
	print args
	
>>> argsFunc(1, 2, 3, 4)
1
(2, 3, 4)
```
在函数中以元组形式（tuple）使用

## **kwargs
形参名前加两个 * 表示，参数在函数内部将被存放在以形式名为标识符的 dictionary 中，这时调用函数的方法则需要采用 arg1=value1,arg2=value2 这样的形式。

```python
import mysql.connector  

db_conf = {
	user='xx',
	password='yy', 
	host='xxx.xxx.xxx.xxx',
	database='zz'
}

cnx = mysql.connector.connect(
	user=db_conf['user'],
	password=db_conf['password'], 
	host=db_conf['host'],
	database=db_conf['database']
	)
# 可以转化为下面这种写法
db_conf = {
	user='xx',
	password='yy', 
	host='xxx.xxx.xxx.xxx',
	database='zz'
}

cnx = mysql.connector.connect(**db_conf)
```