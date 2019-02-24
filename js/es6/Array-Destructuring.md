```js
/* 数组解构赋值 */

// ,占位符
let [ , , t1] = [1, 2, 3];
console.log(t1); // 3

// ...展开数组
let [a1, ...t2] = [1, 2, 3];
console.log(a1, t2); // 1 [2, 3]

// 解构失败，返回undefined
let [a2, t3] = [1];
console.log(a2, t3); // 1 undefined

// 右边不是数组，报错。原因是右表达式不具有Iterator接口，如果有，依然可以解构
// let [t4] = 200;
// console.log(t4);

// 解构默认值。当元素值===undefined时才会触发默认值
let [t5 = true] = [];
console.log(t5); // true
let [t6 = 1, t7 = 2] = [null, undefined];
console.log(t6, t7); // null 2
let [t8, t9 = () => { console.log('abc'); return 'ccc'; }] = [1, 2]; // no output
console.log(t8, t9); // 1 2
```