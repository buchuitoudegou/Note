```js
/* 数组的扩展 */
// 用扩展运算符代替apply
const a = [1, 2, 3, 4]
Math.max(...a); // <=> Math.max.apply(null, a);
// 扩展运算符复制数组
const a2 = [...a];
const [...a3] = a; // <=> const a3 = a.concat();
// 扩展运算符合并数组
const a4 = [...a, ...a2, ...a3]; // <=> a4 = a.concat(a2, a3);

// Array.of转化为数组
console.log(Array.of(3, 11, 8)); // [3, 11, 8]
console.log(Array.of(3)); // [3]
console.log(Array(3)); // [,,,]

// find，找出第一个符合的成员，并返回这个成员，否则返回undefined
console.log([1, 2, 3, 4].find((x) => x == 4)); // 4

// findIndex返回第一个符合条件的数组成员的下标，没有则返回-1
console.log([3, 4, 5, -10].findIndex((x) => x < 0)); // 3
// findIndex和indexOf的区别：可以找到NaN
[NaN].indexOf(NaN); // -1
[NaN].findIndex(x => Object.is(NaN, x)); // 0

// fill填充数组
console.log(new Array(3).fill(7)); // [7, 7, 7]
// fill填充对象的时候，是浅复制
const a5 = new Array(3).fill({ "a": 1 });
a5[0]['a'] = 'aaa';
console.log(a5); // [ { a: 'aaa' }, { a: 'aaa' }, { a: 'aaa' } ]

// includes
console.log([1, 2, 3].includes(3)); // true
// includes 的第二个参数为起始位置
console.log([1, 2, 3].includes(1, 1)); // false
```