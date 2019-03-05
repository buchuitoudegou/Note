# Something about DOM

## 目录
[Node](##节点)
[Document类型](##Document类型)
[Element类型](##Element类型)

## DOM的概念
DOM是针对HTML和XML文档的一个API。它描绘了一个层次化的节点树。

## 节点
页面上所有的元素都可以被描述成节点（不仅是所有的HTML元素，还包括属性、文本等，这些都是DOM节点）。页面的根节点叫做文档节点（理解为document），并且它只能有一个子节点——< html>元素，它也叫文档元素。

在DOM中，实现了一个Node接口，所有的节点类型都是由它派生而来。并且，每种节点都有一个NodeType属性，用来表明节点的类型。在DOM中有12种DOM类型：
* Node.ELEMENT_NODE(1)
* Node.ATTRIBUTE_NODE(2)
* Node.TEXT_NODE(3)
* Node.CDATA_SECTION_NODE(4)
* Node.ENTITY_REFERENCE_NODE(5)
* Node.ENTITY_NODE
* Node.PROCESSING_INSTRUCTION_NODE(7)
* Node.COMMENT_NODE(8)
* Node.DOCUMENT_NODE(9)
* Node.DOCUMENT_TYPE_NODE(10)
* Node.DOCUMENT_FRAGMENT_NODE(11)
* Node.NOTATION_NODE(12)

在这些节点类型中，我们最常使用的是元素、文本和属性节点。每个节点都有nodeName和nodeValue属性，不同类型的节点，他们的nodeName和nodeValue的定义都不一样，在使用的时候最好先检测一下要访问的是什么类型的节点。

另外，所有的Node节点都有childNodes属性，记录了所有子节点。要注意的是，childNodes的类型是NodeList，它是一个会动态变化的对象，因此在遍历的时候要特别小心。当然，我们也可以用slice方法将它转化为一个数组，再进行访问：
```js
const arrayNodes = [].slice.call(aNode.childNodes, 0);
```
除了childNodes，每个节点也有parentNode，指向父元素；同理，还能用nextSibling和previousSibling来访问它邻近的兄弟。最后，还能用firstChild和lastChild来访问第一个和最后一个子节点。

*所有Node共用的属性*

* nodeType
* nodeName
* nodeValue
* childNodes
* parentNode
* nextSibling
* previousSibling
* firstChild
* lastChild

介绍完所有Node共同拥有的属性，现在来介绍一下所有Node共同拥有的方法。最常用的方法是appendChild，即将一个节点插入到某个节点子节点列表的最后。但是要注意的是：由于每个节点都只能有一个父节点，用appendChild插入到别的节点中之后，原来的父节点就会失去这个子节点。

如果想控制插入的位置，就可以用到insertBefore方法。它接受连个参数：新插入的节点，以及定位的节点。如果定位的节点是null，则插入到childNodes列表的最后（相当于appendChild）。

除了增加节点之外，DOM还支持用户移除、替换节点。replaceChild接受两个参数：新节点、被替换的节点；removeChild接受一个参数：要移除的节点。

最后还有复制节点cloneNode函数。它接受一个参数：是否深复制。深复制会复制其子节点，浅复制只会复制自身。

*所有Node共有的方法*

* appendChild
* removeChild
* replaceChild
* insertBefore
* cloneNode

## Document类型
在HTML页面中，Document类型的实例就是我们熟知的window.document，也可以直接用document进行访问。它的基本属性：
| nodeType | nodeName | nodeValue | parentNode | ownerDocument |
|:-----:|:-----:|:-----:|:-----:|:-----:|
| 9 | #document | null | null | null |
它的子节点必须是DocumentType（我们常用的DOCTYPE标签），ELEMENT（html），ProcessingInstruction，Comment。它有一个documentElement（文档元素）的属性，始终指向html标签元素。

在HTML页面中，Document类型通常会派生出一个HTMLDocument的类型，它还会有一下这些属性：
* title：title标签中的文本
* body：body标签
* URL：完整地址
* domain：域名
* anchors：所有的a标签
* images：所有的img标签
* links：所有包含href的a标签

一些常用的方法：
* getElementByid
* getElementsByTagName（返回一个HTMLCollection，跟NodeList一样是动态的）

## Element类型
最常用的类型，即我们熟悉的HTML页面元素。基本属性如下：
| nodeType | nodeName | nodeValue | parentNode|
|:-----:|:-----:|:-----:|:-----:|
| 1 | 标签名 | null | Document或者Element |
它的子节点可能是Text、Element、Comment、ProcessingInstruction、CDATASection、EntityReference