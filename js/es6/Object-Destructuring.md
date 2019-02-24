```js
/*对象解构赋值*/
let { a, b } = { a: 1, b: 2 }; // <=> let {a:a, b:b} = {a:1, b:2};
console.log(a, b);// 1 2
let { a: a1, b: b1 } = { a: 1, b: 2 }; // a和b是模式，并不是变量。模式名必须和属性名相匹配才能解构
console.log(a1, b1); // 1 2

// 嵌套解构
let obj = {
	p: ['Hello', { y: 'World' }]
};
let { p: [x, { y }] } = obj;
console.log(x, y); // Hello World

// 嵌套解构
const node = {
  loc: {
    start: {
      line: 1,
      column: 5
    }
  }
};
let { loc, loc: { start }, loc: { start: { line }} } = node;
console.log(line) // 1
console.log(loc)  // Object {start: Object}
console.log(start) // Object {line: 1, column: 5}

//解构默认值，和数组解构一样，当===undefined的时候才会触发
let { x: x1 = 'aaa' } = { x: null };
console.log(x1); // null
let { x: x2 = 'aaa' } = { x: undefined };
console.log(x2); // aaa
let { x: x3 = 'aaa' } = { y: null };
console.log(x3); // aaa
```