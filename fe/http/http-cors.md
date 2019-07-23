# http的跨域请求

## 简单请求不会触发CORS预检
满足一下条件的请求是简单请求：
1. 使用GET/HEAD/POST方法
2. HTTP首部只有以下允许字段：Accept/Accept-Language/Content-Type
3. Content-Type是以下三种之一：text/plain、multipart/form-data、application/x-www-form-urlencoded

## 其他请求会触发预检
首先使用OPTIONS方法发起一个预检请求，询问是否允许该请求的发生。

预检请求中，带有如下字段，分别告诉服务器请求源、请求方法、请求的自定义首部
```
Origin：...
Access-Control-Request-Method：POST
Access-Control-Request-Headers：...
```
预检返回：
```
Access-Control-Allow-Origin: http://foo.example
Access-Control-Allow-Methods: POST, GET, OPTIONS
Access-Control-Allow-Headers: ...
```
Allow-Origin表明该资源允许哪些域访问（可以是 * ）

如果客户端一开始发送的请求中有cookie，服务端也会有可能发送一个Access-Control-Allow-Credentials，表明是否允许cookie发送。如果没有的话，该请求可能失败。

另外，如果设置了cookie的请求跨域访问，Acces-Control-Allow-Origin不可以设 * 。