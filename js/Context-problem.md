```js
function func1() {
	console.log(this.a);
}

function func2() {
	this.a = 'a';
	func1();
}

func2();
console.log(this.a)
```
output:
```
a
undefined
```
why `func1` and `func2` share the same context?

`context` probably means the value of `this` in the same scope. If a function defined in a scope, the value of `this` will be `global` when it's called directly. In other word, the `context` of the function is `global`  unless `call`, `apply` or `new` are used or called by other object (e.g. `obj.func()`). 

Furthermore, if `call`, `apply` or `new` are used to call a function, the `context` of the function will be changed while the `context` of the functions defined in it will not.
```js
function a() {
	console.log(this.b);
	const that = this;
	function b() {
		console.log(this === that);
	}
	b();
}
a.call({ b: 'b' })
a();
```
output:
```
b
false
undefined
true
```

When a function is called, 3 steps are executed:
1. create variables and activate objects
2. create scope of the function
3. set the value of `this`