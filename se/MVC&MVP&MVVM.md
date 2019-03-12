# MVC、MVP、MVVM的区别

[参考链接](https://juejin.im/post/593021272f301e0058273468)

## MVC
MVC模式中，M代表的model储存了应用数据，而V代表的view负责展示model中的数据。M和V之间采用观察者模式，在M的数据改变之后，通知V自动变更。C代表controller，负责响应view层面的事件。

缺点：主要逻辑在Controller，当每个事件都经过Controller的时候，这层会变得十分臃肿。而且，每个view都要绑定一个controller，代表一个组件，这就使得controller难以复用。

## MVP
这里的P代表的是presenter。这种模式中，view是不能直接访问model的，它只能通过presenter提供接口，更新Model，进而用观察者模式更新view。也就是说，这种模式解除了view和model的绑定，让presenter负责所有的业务逻辑。

但缺点是显而易见的：这样presenter会变得很重，维护起来非常困难。而且由于没有数据绑定，如果presenter对视图的渲染需求增加，它不得不关注特定的视图，一旦视图需求发生改变，presenter也要改变。

## MVVM
跟MVP的区别在于，view和model的同步不用再手动操作，而是交给框架提供的数据绑定功能自动完成。同时，视图发生改变之后，viewModel在自身改变的同时，也会自动改变存储在model中的值。（参考Vue，template可以看作一个view，Vue.component是viewModel，data是model）

这个模式比前两个优秀的地方在于，封装了所有的数据绑定操作，viewModel的编写不需要关心view的情况，model也不需要察觉到view的存在，同理view也不需要知道model，只要关心viewModel给他提供的接口即可。