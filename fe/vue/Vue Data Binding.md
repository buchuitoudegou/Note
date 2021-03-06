# Vue data binding
相关源码：https://github.com/buchuitoudegou/Data-Binding-demo
![preview](https://github.com/buchuitoudegou/Note/raw/master/snapshot/Data%2Bbinding.jpg)

Vue的数据绑定机制利用了观察者设计模式，利用侦听器动态更新DOM元素中的值，以下是Vue在编译时绑定数据的过程。

## 绑定setter和getter
index.mjs是编译的入口。从SelfVue的构造函数我们可以看到，对于options中的data，我们会先对它进行一个observe的操作，即绑定getter和setter，这也是数据响应模式构造的开始。
```js
// index.mjs
export function SelfVue(options) {
  this.data = options.data;
  observe(this.data);
  Object.keys(this.data).forEach((key) => {
    Object.defineProperty(this, key, {
      enumerable: true,
      configurable: true,
      get: () => {
        return this.data[key];
      },
      set: (newVal) => {
        this.data[key] = newVal;
        // console.log('set index');
      }
    });
  });

  new Compile(options.el, this);

  return this;
}
// Observable.mjs
export function observe(data) {
  if (!data || !(typeof(data) === 'object')) {
    return;
  }
  Object.keys(data).forEach((key) => {
    defineReactive(data, key, data[key]);
  });
}

function defineReactive(data, key, val) {
  observe(val);
  const dep =  new Dep;
  Object.defineProperty(data, key, {
    enumerable: true,
    configurable: true,
    get: () => {
      if (Dep.target) {
        dep.addSub(Dep.target);
      }
      return val;
    },
    set: (newVal) => {
      // console.log(newVal);
      val = newVal;
      dep.notify();
    }
  });
}
```
defineReactive函数递归给data对象的每个属性（如果该属性也是对象，则递归执行）绑定getter和setter。

getter是这样定义的：如果这个属性被订阅了，则将它的侦听器加入到订阅数组中，最后返回这个属性的值。

```js
// Dep.mjs
export class Dep {
  constructor() {
    this.subs = [];
  }

  static target = null;

  addSub(sub) {
    this.subs.push(sub);
  }
  
  notify() {
    this.subs.forEach(sub => sub.update());
  }
}
```
这里提到了两个概念侦听器（Watcher）和订阅（Dep）数组。侦听器接下来会提到，先简单介绍一下订阅的概念。订阅器（Dep）是一个实例，每个data中的对象（Object）都会有一个Dep实例（data自己也会有一个），它里面定义了订阅数组，存储了所有订阅某个属性的侦听器。订阅器有一个notify方法，用于通知订阅数组里面所有的侦听器，某个值发生了改变（并不会判断该侦听器是否侦听了这个值）。另外，订阅器类（Dep）有一个静态属性target，用于存储当前正在进行订阅操作的属性（相当于订阅器的办事窗口，而且只有一个窗口）。

介绍完订阅器的概念之后，继续之前的内容，setter的定义应该就可以猜到了：setter在调用时，除了更新这个属性的值，还会调用订阅器的notify。

上面的步骤是Vue的root实例在定义的时候做的第一步工作。由于这个时候还不知道哪些属性会绑定到DOM上，因此，所有订阅器的订阅数组都是空的。

## 编译DOM节点
```js
// Compile.mjs
export class Compile {
  constructor(el, vm) {
    this.vm = vm;
    this.el = document.querySelector(el);
    this.fragment = null;
    this.init();
  }

  init() {
    if (this.el) {
      this.fragment = document.createDocumentFragment();
      let child = this.el.firstChild;
      while (child) {
        this.fragment.appendChild(child);
        child = this.el.firstChild;
      }
      [].slice.call(this.fragment.childNodes).forEach(node => {
        this.compileElement(node);
      });
      this.el.appendChild(this.fragment);
    }
  }

  compileElement(ele) {
    const reg = /\{\{(.*)\}\}/;
    if (ele.nodeType === 3) {
      if (reg.test(ele.nodeValue)) {
        let temp = RegExp.$1;
        temp = temp.trim();
        ele.nodeValue = getValue(this.vm, temp.split('.'));
        new Watcher(this.vm, temp.split('.'), (value) => {
          ele.nodeValue = value;
        });
      }
    } else if (ele.nodeType === 1) {
      const attr = ele.getAttribute('vmodel');
      if (attr) {
        ele.value = getValue(this.vm, attr.split('.'));
        new Watcher(this.vm, attr.split('.'), (value) => {
          ele.value = value;
        });
        ele.addEventListener('input', (e) => {
          this.vm[attr] = e.target.value;
        });
      }
    }
  }
}
```

这个步骤里，我们会知道哪些节点（HTMLElement或者文本节点）会和Vue的data有互动。给这些有互动的节点设置侦听器（Watcher）。接下来介绍一下Watcher。

## Watcher设置
```js
// Watcher.mjs
export function getValue(data, exp) {
  if (exp.length === 0) {
    return null;
  }
  let temp = data;
  exp.forEach((key) => {
    temp = temp[key];
  });
  return temp;
}

export class Watcher {
  constructor(vm, exp, cb) {
    this.vm = vm;
    this.exp = exp;
    this.cb = cb;
    this.value = this.get();
  }

  update() {
    const value = getValue(this.vm.data, this.exp);
    const oldVal = this.value;
    if (value !== oldVal) {
      this.value = value;
      this.cb.call(this.vm, value, oldVal);
    }
  }

  get() {
    Dep.target = this;
    const value = getValue(this.vm.data, this.exp);
    Dep.target = null;
    return value;
  }
}
```
侦听器，顾名思义用于侦听某个属性的值是否发生了变化。我们在给某个属性构造侦听器的时候需要传入一个回调函数，这个回调函数用于更新对应DOM节点的值。至于哪个节点对应哪个属性值我们在编译的时候就可以得知。

Watcher在构造的时候，会强行调用这个属性值的getter。这是为什么呢？之前我们也提到过，getter中定义了，若某个值被订阅，则它的侦听器会加入到订阅数组中。因此，给某个值构造Watcher时，先让Dep的订阅窗口（Dep.target）指向这个Watcher，说明这个值需要被订阅，然后再调用这个值的getter，这样子Dep的订阅数组中就会把这个Watcher放进去。最后将订阅窗口（Dep.target）指向null，表示订阅完成，可以订阅下一个属性。

每个Watcher都会有一个update方法，用于调用之前提到过的回调函数。当Dep调用notify方法的时候，就会触发每个在订阅数组中的Watcher的update方法。update方法中确认自己的侦听的值是否发生了变化，若发生了变化则调用构造的时候传入的回调函数，更新DOM元素中的值。

## 当一个值发生变化时
当data中一个值发生了变化时，setter会调用对应Dep实例的notify函数，notify函数调用实例中存储的订阅数组中所有侦听器的update方法。update方法中，侦听器会判断自己侦听的值是否发生了变化。若发生了变化，则调用回调函数，更新DOM节点中的值。
