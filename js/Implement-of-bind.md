```js
Function.prototype.Bind = function(obj) {
  const self = this;
  return function() {
    self.call(obj, ...arguments);
  }
}

function demo(cc, ll) {
  console.log(this.a);
  console.log(cc);
  console.log(ll);
}
let e = demo.Bind({ a: 1 });
e('cc', 'll');
```
expected out: 
```
1
cc
ll
```