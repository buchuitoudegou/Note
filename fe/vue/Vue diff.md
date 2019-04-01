# Vue的diff算法

在Vue中，实现了VNode节点，以减少渲染次数，并实现节点的复用。当一个值改变的时候，Vue并不会马上进行重新渲染，而是先比对新生成的VNode和之前的VNode之间的区别，然后再进行页面中的dom操作。

当生成了一个新的VNode时，diff算法会进行如下的操作：

## 比较OldNode和NewNode是否相同
Vue在内部实现了一个sameNode的函数，用来比较两个VNode是不是同一个VNode。sameNode会对它们的tagName，key，attr等一系列属性进行比对，如果二者是同一个元素，diff算法就要发挥作用了。而如果它们不是同一个元素（e.g. div元素变成了span），Vue就会直接将新元素替换掉旧元素。

## 如果OldNode和NewNode是同一个节点
如果二者是同一个节点，那么我们就可以考虑节点复用的事情了，这也是diff算法的核心。

首先判断两个Node的之间的文本节点是否相同，如果不同则将其变得相同。再对它们的子节点进行判断：
1. 如果NewNode没有子节点而OldNode有，那么移除OldNode的子节点
2. 如果NewNode有子节点而OldNode没有，那么给OldNode新增这些子节点
3. 如果二者都有子节点，那么就要进行下一步的比较了

## 比较OldNode和NewNode的子节点
首先将它们的子节点抽出来，放在数组里。这里要注意到，子节点是指这个Node的直属子节点，子节点的子节点并不算在内。

现在将OldNode的子节点称为OldChild，而NewNode的子节点称为NewChild。分别给他们设置startIdx和endIdx。并开始比较，四种情况满足一种即可：
1. 比较NewStartIdx和OldStartIdx是否相同，相同则将OldStartIdx的元素移到NewStartIdx的位置，NewStartIdx和OldStartIdx往前走一格
2. 比较NewStartIdx和OldEndIdx是否相同，相同则将OldEndIdx的元素移到NewStartIdx的位置，二者往前走一格
3. 比较NewEndIdx和OldEndIdx是否相同，相同则将OldEndIdx的元素移到NewEndIdx的位置，二者往前走一格
4. 比较NewEndIdx和OldStartIdx是否相同，相同则将OldStartIdx的元素移到NewEndIdx的位置，二者往前走一格

如果四次比较中，都没有出现相同的结果，那么，我们会根据OldChild的key值生成一个哈希表，在表中找与NewStartIdx对应元素的key值匹配的OldChild：
1. 如果找不到，将NewStartIdx对应的元素当成新元素，并插入到NewStartIdx的位置
2. 如果找到了key值相同的OldChild：
   1. 若节点选择器不同，直接当成新节点插入
   2. 反之，深度比较二者的子节点（再进行一次diff算法），然后将找到的这个元素插入到NewStartIdx的位置
3. NewStartIdx往前走一格

在某一个节点（NewNode/OldNode）的全部子节点比较完后，若另一个节点还有自己未能匹配：
1. 如果是OldNode的话，则删除这些多出来的节点
2. 如果是NewNode的话，在OldStartIdx开始插入新的节点