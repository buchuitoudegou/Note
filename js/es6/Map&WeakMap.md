## Map
```js
/* 数据结构Map */
// 任何类型的值都可以当作键
// 构造
const obj = { 'a': 'aaa' };
const a = new Map;
const a1 = new Map([
  ['name', 'abc'],
  [obj, 1]
]);
console.log(a1.has(obj)); // true

// 操作方法
a.set(obj, 10);
console.log(a.get(obj)); // 10
console.log(a.get({ 'a': 'aaa' })); // undefined
a.delete(obj);
console.log(a.has(obj)); // false

// 遍历方法: keys, values, entries, forEach
// Map的遍历顺序就是插入顺序
const a2 = new Map([['a', 1], ['b', 2]]);
console.log(...a2.keys()); // a b
console.log(...a2.values()); // 1 2
console.log(...a2.entries()); // ['a', 1] ['b', 2]
a2.forEach((val, key) => { console.log(val, key); });
// 1 'a'
// 2 'b'
```