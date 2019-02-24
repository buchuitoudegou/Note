## Set
```js
/* 数据结构Set */
// 构造方法-逐个添加
const a = new Set;
[1, 2, 3, 4].forEach(x => a.add(x));
console.log(...a); // 1 2 3 4

// 构造方法-构造函数可接收一个有Iterable接口的结构
const a1 = new Set([1, 2, 3, 4, 4]);
console.log(...a1); // 1 2 3 4
const a2 = new Set('abcd');
console.log(...a2); // a b c d

// 四个操作方法: add, delete, has, clear
const a3 = new Set(['[object Object]', {}]);
a3.delete({});
console.log(a3.size); // 2。delete方法的判断条件是===
console.log(a3.has({})); // false。has方法的判断条件是===

// 四个遍历方法: keys, values, entries, forEach
const a4 = new Set(['a', 'b', 3, 4]);
// keys方法和values方法返回值一样
console.log(...a4.keys()); // a b 3 4
console.log(...a4.values()); // a b 3 4
// entries方法返回键名和键值，set没有键名只有键值
console.log(...a4.entries()); // [ 'a', 'a' ] [ 'b', 'b' ] [ 3, 3 ] [ 4, 4 ]

```

## WeakSet
```js
/* 数据结构WeakSet */
// WeakSet的成员只能是对象，而不能是其他类型的值
// 而且WeakSet里面的对象是不计入垃圾回收机制的，如果外部引用消失，则WeakSet中的引用也会消失
// 也因为这个特点，WeakSet是不可被遍历的，没有forEach方法，没有size属性

// 构造
const arr = [{1: 2}, [3, 4]];
const a = new WeakSet();
const a1 = new WeakSet(arr);
console.log(a1.has(arr[0])); // true
const obj = {a: 88};
a1.add(obj);
console.log(a1.has(obj)); // true
a1.delete(obj);
console.log(a1.has(obj)); // false
```