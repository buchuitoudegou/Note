# DOM2和DOM3的一些常用属性

## style对象
可以访问各种css属性。xxx.style.backgroudColor: string

* 内联样式：cssText
* removeProperty(propertyName)
* setProperty(propertyName, value, priority) priority是一个字符串''或'importan'

## 取得样式表计算属性
```js
var div = document.getElementById('...');
var st = document.defaultView.getComputedStyle(div, null);
console.log(st.width);
```

## 元素大小

### 偏移量
1. offsetHeight（content+padding+border）
2. offsetWidth（content+padding+border）
3. offetLeft外边框到包含元素的左内边框
4. offsetTop同left

### 客户区大小
1. clientHeight：padding+content
2. clientWidth：同上

### 滚动大小
1. scrollHeight：没有滚动条的情况下，内容高度（即内容实际占屏幕的大小）
2. scrollWidth：同上
3. scrollLeft：被隐藏在内容区域左侧的像素（可用于设置滚动条的位置）
4. scrollTop同上

### 遍历

### NodeIterator
创建：document.createNodeIterator(root, whatToShow, filter, entityReferenceExpansion)
```
root：想要作为搜索起点的树的节点
whatToShow：访问节点的相关掩码
filter：NodeFilter是个函数
entityReferenceExpansion：boolean是否要扩展实体引用

whatToShow的常用类型：
1. NodeFilter.SHOW_ALL显示所有类型节点
2. NodeFilter.SHOW_ELEMENT显示元素类型的节点
3. NodeFilter.SHOW_TEXT显示文本节点
4. NodeFilter.SHOW_ATTRIBUTE显示属性节点

```
```js
// filter是一个函数
function filter(node) {
  return node.tagName.toLowerCase() === 'p' ?
    NodeFilter.FILTER_ACCEPT :
    NodeFilter.FILTER_SKIP;
}
```
创建一个NodeIterator
```js
var itr = document.createNodeIterator(document, NodeFilter.SHOW_ALL, filter, flase);
```
用itr.nextNode()或者itr.previousNode()访问上一个或者下一个节点。

### TreeWalker
和NodeIterator类似。但遍历方法比它多。
* parentNode()根节点
* firstChild()第一个孩子
* lastChild()最后一个孩子
* nextSibling()下一个兄弟
* previousSibling()前一个兄弟