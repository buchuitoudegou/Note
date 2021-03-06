# http连接管理

## 并行的http事务
通过多条tcp连接发起并行的http请求。速度不一定更快，要考虑客户端到服务端的链路带宽。而且开过多的连接会消耗大量的内存和端口。现代浏览器一般只允许4条并行连接。

## 持久连接
connection：keep-alive的http头表示，客户端希望和服务端发起持久连接。如果服务端返回的头中也包含这个字段信息，则说明可以发起持久连接（tcp连接不关闭）。connection字段的另一个取值可能是close，说明这个连接在该事务完成后关闭。

keep-alive字段。可能有max=？或者timeout=？max说明服务器还可以为多少个事务保持连接状态；tiemout表示连接空闲多少秒后自动关闭。

### 持久连接的盲中继问题
代理服务器在转发请求的时候不会检查请求中的connection字段，它在转发完成后将该连接关闭。而服务器在收到keep-alive的请求后，表示可以keep-alive，这个代理服务器同样将该请求返回给客户端。现在客户端和服务端都认为只要向同一个tcp连接发送数据即可，而实际上，代理服务器已经将该连接关闭。

netscape曾经使用扩展字段：proxy-connection来鉴别哪些是聪明代理（会识别keep-alive，并将proxy-connection变成connection），但是只要在转发过程中遇到一个盲代理，还是有之前的问题。

在http/1.1中，keep-alive变成默认true。也就是说，只要不在connection中设置false，就不会关闭连接。使用http/1.1的代理服务器不会遇到之前的问题。但是如果http/1.1的报文遇到使用http/1.0的服务器还是会出现问题（这种情况比较常见）。

## 管道化连接
即使用pipeline的方式发送请求。在响应到达前就可以将队列中的后续请求继续发送。但是如果发生错误必须有重新请求的策略；另外，post是非幂等性方法，在出错时很难安全地进行重试。