# js数组的方法

## map方法
```js
arr.map(function callback(curVal, curIdx, curArr) {
    ...
})
```
返回一个新数组。

## forEach方法
```js
arr.forEach(function callback(curVal, curIdx, curArr)) {
    
}
```
forEach循环无法终止。如果需要判断每个元素是否符合某个规则，可以使用every方法。

## filter
```js
const newArr = arr.filter((ele, idx, arr) => {
    
})
```
返回通过了测试的元素数组。

## shift和unshift
shift移除头元素（pop移除尾元素）

unshift(ele, ...)添加N个元素到头

## splice
参数一：start，表示开始修改的位置
参数二：表示删除的元素个数
参数三...n：要从start开始添加进数组的元素（可以不填，这样就只会删除元素）