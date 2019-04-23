# 用webpack创建一个React应用

## 创建项目
```
npm init
```

## 安装webpack和webpack-cli
```
npm install webpack --save-dev
npm install webpack-cli --save-dev
```
添加webpack指令
```json
"script": {
  "build": "webpack --mode prodution"
}
```

## 使用Babel转换ES6代码
由于React大多数语法是JS ES6标准的，因此需要Babel对其进行转义。这时候就要用webpack的loader将这些ES6模块转换为ES5模块。因此，先安装Babel的loader
```
npm install @babel/core babel-loader @babel/preset-env @babel/preset-react --save-dev
```
然后添加一个Babel的配置文件`.babelrc`
```json
{
  "preset": ["@babel/preset-env", "@babel/preset-react"]
}
```
安装完成之后就可以开始编写webpack的配置文件了。
```js
const path = require('path');

module.exports = {
  entry: './src/index.js',
  output: {
    filename: '[main].bundle.js',
    path: path.resolve(__dirname, 'dist'),
    publicPath: '/'
  },

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader"
        }
      }
    ]
  }
};
```
这一步只是规定了webpack打包的入口，输出文件的名字、目录，并用Babel编译。在编写完React应用之后，运行`npm run build`就可以生成一个`main.bundle.js`的文件。

*这里我们假设编写的React应用的入口是index.js，即用了ReactDOM.render渲染的节点的文件。* 

## 在html中引用bundle
最简单的方法就是创建一个`index.html`，并在里面引用`main.bundle.js`。但是，这样也会有别的问题：如果我重新构建之后生成的新js文件名字并不是`main.bundle.js`，则又要打开`index.html`修改里面的引用。这显然是不人性化的。

因此这时候我们就要用到`HtmlWebpackPlugin`模块。他会将生成的bundle全部自动添加到html中。也可以给这个html指定模板。

安装方法：
```
npm i html-webpack-plugin html-loader --save-dev
```
```js
plugins: [
  new HtmlWebPackPlugin({
    title: 'New Project',
    template: './public/index.html'
  })
]
```

## 加载图片
由于webpack不能自动处理其他格式的模块，因此，我们需要安装`file-loader`或者`url-loader`等loader模块，对他们进行处理。
```js
{
  test: /\.(png|jpg|gif)$/,
  use: [
    {
      loader: 'file-loader',
      options: {}
    }
  ]
}
```
使用了loader之后，我们就可以在文件中用`import`语句，将图片资源加载进来（相当于一个JS的模块）。

## 使用webpack-server调试
`webpack-dev-server`也是webpack中十分重要的功能。它可以给我们提供调试的环境，这样就不用每次都要跑一次编译，只要修改保存代码就可以预览到新效果。而且，`webpack-dev-server`不会写入磁盘，它编译的文件会读到内存中。
```
npm install webpack-dev-server --save-dev
```
给`package.json`添加script
```json
"scripts": {
  "test": "echo \"Error: no test specified\" && exit 1",
  "start": "webpack-dev-server --open --mode development",
  "build": "webpack --mode production"
}
```
这样只需要运行`npm start`即可

## 文件管理器打开编译出来的文件
按照我们以往对webpack的理解，必须给dist文件夹添加一个静态http服务器才能查看具体内容，而直接打开`index.html`会看到无法加载一些资源。无法加载的原因是*路径错误*。也就是说，只要把路径纠正，就能直接打开`index.html`预览。具体操作只要：
```js
output: {
  filename: 'js/[name].bundle.js',
  path: path.resolve(__dirname, 'dist'),
  publicPath: './'
},
```
将`publicPath`默认值从'/'改成'./'，这样我们就能直接打开`index.html`预览啦。

## reference
* https://imweb.io/topic/5be0159bb5bbd42b053d0458
* https://www.webpackjs.com/
