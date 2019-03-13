# Extension of DOM

## 选择符API
* querySelector(string)，接收一个css选择符字符长，返回匹配的第一个元素。
* querySelectorAll(string)，和querySelector类似，但返回全部匹配的元素，返回类型是NodeList
* matchesSelector(string)，判断某个元素是否满足这个css选择符

## 元素遍历
DOM的扩展中为HTMLElement新增了几个专门用于遍历元素的API
* childElementCount
* firstElementChild
* lastElementChild
* previousElementSibling
* nextElementSibling
这些API在Node的API中都有对应，但这里的API返回值都是Element

## HTML5的相关扩展

### 元素class
* 新增getElementsbyClassName方法，返回一个NodeList
* 每个元素新增classList属性，是DOMTokenList类型。classList有三种操作：
  * add，新增一个class
  * contains，是否包含某个类
  * remove，移除一个类
  * toggle，存在则移除，不存在则新增

### 焦点管理
* 新增document.activeElement属性，这个属性始终引用DOM中获得了焦点的元素。元素获取焦点的方式有页面加载、用户输入和在代码中调用focus方法。e.g. btn.focus()
* 新增document.hasFocus()判断文档是否获得了焦点

### HTMLDocument的变化
document新增了readyState属性，它有两个可能的值：'loading'和'complete'。

### 自定义数据属性
在元素中可以添加data-xxx属性，然后我们可以通过ele.dataset.xxx来访问它的值。

### 插入标记
innerHTML属性：返回该元素所有子节点（包括文本节点、注释节点等）对应的HTML标记。给它设置一个值，可以替换原来的DOM。

outerHTML属性：返回包括元素自己在内的所有子节点对应的HTML标记。设置值时，会把自己替换成新的值。

insertAdjacentHTML方法：接收两个参数，第一个表示插入位置，第二个是元素。第一个参数值有四个选项
* beforebegin：作为自己的前一个同辈插入
* afterbegin：作为自己的第一个子元素插入
* beforeend：作为自己的最后一个子元素插入
* afterend：作为自己的后一个同辈插入

### scrollIntoView
让某个元素进入视口。它可以在所有HTML元素上调用。可以传入参数true/false。true（默认）表示调用元素的顶部和视口平齐；false表示底部和视口平齐。

##专有扩展

### children属性
返回元素的所有子元素，而且类型是HTMLCollection，其他地方和childNodes属性相似。

### contains方法
判断某个元素是不是另一个元素的后代。

### 插入文本
innerText可以操作元素中包含的所有文本内容，包括子文档树的文本。在读取值的时候，它会按照*由浅入深的顺序，将子文档树的所有文本拼接起来*。在通过innerText写入值的时候，结果会删除元素的所有子节点，插入包含响应文本值的文本节点。

outerText：用文本节点代替自己

### 滚动
* scrollByLines(lineCount)：元素内容滚动指定的行高
* scrollByPages(pageCount)：元素内容滚动指定的页高