# js的继承
## 原型链继承
```js
function parent() {
	this.name = 'aaa';
}

parent.prototype.sayName = function() {
	return this.name;
}

function child(age) {
	this.age = age;
}

child.prototype = new parent

const ins = new child(10);
console.log(ins.name); // aaa
ins.name = 'bbb';
const ins2 = new child(11);
console.log(ins.name); // bbb
```
两个问题：
1. 不能向父类传递参数
2. 子类原型指向同一个对象

## 借用构造函数
```js
function parent(name) {
	this.name = name;
}

parent.prototype.sayName = function() {
	return this.name;
}

function child(name, age) {
	parent.call(this, name);
	this.age = age;
}

const ins = new child('aa', 10);
console.log(ins.name, is.age); // aa 10
console.log(ins.sayName()); // throw error
```
不能继承父类的方法。父类的方法在构造函数的原型中，因此，子类用call的方式，无法得到它们。

## 组合继承
```js
function parent(name) {
	this.name = name;
}

parent.prototype.sayName = function() {
	return this.name;
}

function child(name, age) {
	parent.call(this, name);
	this.age = age;
}

child.prototype = new parent();
child.prototype.constructor = child;
child.prototype.sayAge = function() {
	return this.age;
}
const ins = new child('aaa', 11);
const ins2 = new child('bbb', 12);
ins.name = 'ccc';
console.log(ins.name, ins2.name); // ccc bbb
```
组合继承组合了原型链和借用构造的优点:
1. 继承父类的属性和方法（原型链）
2. 可以向父类传递参数（借用构造函数）
3. 子类实例的属性修改不会影响其他实例

缺点：
1. 多次调用父类的构造函数

## 寄生式继承
```js
function createNewObject(origin) {
	let obj = new Object(origin);
	obj.sayYeah = function() {
		console.log(this.a);
	}
	return obj;
}

let obj = createNewObject({ a: 1 });
obj.sayYeah(); // 1
```
跟工厂模式类似。传入一个对象，并增强这个对象。

## 寄生组合式继承
```js
function parent(name) {
	this.name = name;
}

parent.prototype.sayName = function() {
	return this.name;
}

function child(name, age) {
	// 继承父类的属性
	parent.call(this, name);
	this.age = age;
}
// 继承父类的原型方法
child.prototype = Object.assign({}, parent.prototype);
child.prototype.constructor = child;
// 子类的方法
child.prototype.sayAge = function() {
	return this.age;
}

const ins = new child('aaa', 11);
const ins2 = new child('bbb', 12);

console.log(ins.sayName(), ins2.sayName()); // aaa bbb
console.log(ins.sayAge(), ins2.sayAge()); // 11 12
```
开发中最理想的继承方式。
