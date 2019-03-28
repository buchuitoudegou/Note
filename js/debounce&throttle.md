# js的防抖和节流

## 防抖
在事件被触发n秒后才会调用回调函数，而如果事件在这n秒内再次触发，则会重新开始记时。

例子：输入框的回调函数。在回调函数中若有ajax操作，我们不能频繁触发。

实现：
```js
function debounce(fn, wait) {
  let timer = null;
  return function() {
    const context = this;
    let args = arguments;
    if (timer) {
      clearTimeout(timer);
      timer = null;
    }
    timer = setTimeout(function() {
      fn.call(context, ...args);
    }, wait);
  }
}

const fn = function() {
  console.log('boom');
}

const dFn = debounce(fn, 1000);

setInterval(function() {
  dFn();
}, 500); // 不会输出；因为fn的触发时限是1秒，而现在触发频率是0.5s，每次都会将计时器重置
```

## 节流
在一段时间内，若一个事件被多次触发，则只会触发一次回调。

实现：
```js
function throttle(fn, wait) {
  let lastTime = null;
  return function() {
    let newTime = new Date();
    if (!lastTime || newTime - lastTime > wait) {
      fn(arguments);
      lastTime = newTime;
    }
  }
}

const fn = function() {
  console.log('boom');
}

const tFn = throttle(fn, 1000);

setInterval(function() {
  tFn();
}, 10); // 每秒输出一次boom
```