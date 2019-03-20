# XMLHttpRequest对象
现代浏览器基本都内置了XMLHttpRequest对象，因此，创建的方法就比较简单了：
```js
var xhr = new XMLHttpRequest();
```
## XHR的用法
1. open方法：open(method, url, isAsync)method代表请求方法，url是请求的url（绝对路径或者相对路径），isAsync表示是否使用异步发送。
2. send方法：send(body)可以发送一个请求主体，如果不需要发送，必须传入null
3. abort方法：xhr对象停止触发事件
4. setRequestHeader(key, val)设置请求头

### readyState属性
有五个状态码：
1. 0：未初始化
2. 1：启动，已经调用了open方法
3. 2：发送，已经调用了send方法
4. 3：接收，已经接收到部分响应信息
5. 4：完成，接收到全部响应数据，而且已经可以在客户端使用

我们可以用onreadystatechange来处理返回的信息

```js
xhr.onreadstatechange = function() {
  if (xhr.readyState === 4) {
    if (xhr.status === 200) {
      ...
    } else {
      ...
    }
  }
}
```

### post请求
用之前提过的send方法来发送内容。但是发送的内容必须是字符串而且满足这样的格式:
```
?key1=val1&key2=val2...
```
我们可以手动处理对象里面的属性，并将它们拼接成这样的字符串。也可以用FormData对象：
```js
var data = new FormData();
for (const key of Object.keys(postData)) {
  data.append(key, myData[key]);
}
xhr.send(data);
```
### 超时设置
```js
xhr.timeout = 1000; // 1000ms
xhr.ontimeout = function() {
  ...
}
```

### 进度事件
用于前端制作进度条，XHR有6个进度事件。
1. loadstart：在接收第一个字节时触发
2. progress：在接收响应期间不断触发
3. error：在请求错误时触发
4. abort：在调用abort时触发
5. load：在接收到完整的数据时触发
6. loadend：在通信完成或者触发error，abort，load事件后触发

onload事件会接收到一个event对象，event.target指向这个XHR对象

onprogress事件非常有用，可以用来制作进度条。它也会接收到一个event对象，event.target指向这个XHR对象。而且，event还会有三个附加属性：
1. lengthComputable：进度信息是否可用
2. position：表示已经接收的字节数
3. totalSize：表示响应头部确定的预期字节数

## 跨域资源共享（CORS)
IE对它的实现比较复杂。直接看别的浏览器。别的浏览器不需要特殊操作，XHR对CORS原生支持。但会有一些限制，比如：不能使用setRequestHeader，不能发送和接收cookie，getAllResponseHeaders方法返回的总是空字符串。

可以通过设置withCredentials属性来带上cookie等凭据。

## Comet
Ajax是客户端向服务端请求资源的技术，而Comet是服务端向页面推送数据的技术。有两种实现Comet的方式
### 长轮询
页面向服务器发起一个请求，然后服务器一直保持连接打开，知道有数据可以发送。发送完数据之后，浏览器关闭连接，然后又发起一个到服务器的新请求。这一个过程在页面打开期间一直持续不断。

### HTTP流
浏览器向服务器发起一个HTTP请求，而服务器保持及连接打开，周期性地向浏览器发送数据。浏览器通过侦听readystate来处理数据。

### SSE API
即将之前谈到的方法封装成了一个API，直接使用即可。
```js
var src = new EventSource(url);
src.onmessage = function(event) {
  event.data ...
  ...
}
src.close(); 
```
使用这个接口，服务器必须把MIME设置成text/event-stream

## WebSocket
```js
var socket = new WebSocket(url);
socket.send(JSON.stringify(obj));
socket.onmessage = function(event) {
  event.data...
}
socket.close();
```