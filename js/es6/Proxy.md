## definition
`Proxy` is a new API in ES6, which is used to redefine some basic operation of `Object`.
## usage
`Proxy(target, handlers)`, `target` is the target object while `handlers` is an object whose properties are functions that define the user's behaviors when performing operations. Those operations are also called `traps`.
```js
let obj = new Proxy({}, {
  get(obj, prop) {
    return (obj[prop]) ? obj[prop] : 'undefined property';
  },
  set(obj, prop, val) {
    if (prop in obj) {
      throw 'duplicate definition';
    } else {
      obj[prop] = val;
      return true;
    }
  }
});
console.log(obj.a); // undefined property
obj.a = 100;
console.log(obj.a); // 100
obj.a = 1000; // throw error
```