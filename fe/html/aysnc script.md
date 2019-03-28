# script标签中的defer和async

## defer属性
和普通标签一样，在加载到head的时候就下载，定义了defer="defer"的script标签会在页面加载完之后执行（先于contentLoaded事件，但不一定）。原则上有先后顺序。

## async属性
告诉浏览器立即下载该文件，同时加载页面。建议不在async中有DOM操作。异步脚本一定会在load事件前执行，但contentLoaded则不一定。