**using flex layout**
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="X-UA-Compatible" content="ie=edge">
  <title>Document</title>
  <style>
    #parent {
      width: 500px;
      height: 300px;
      display: flex;
      justify-content: flex-start;
    }
    #child-1 {
      width: 100px;
      background-color: red;
    }
    #child-2 {
      flex-grow: 1;
      background-color: green;
    }
  </style>
</head>
<body>
  <div id="parent">
    <p id="child-1">child-1</p>
    <p id="child-2">child-2</p>
  </div>
</body>
</html>
```
`flex-grow` is an attribute of a child element. It defines how parent element allocates the rest space to the child elements. If one of the child element sets a number to `flex-grow` and others don't, it will be allocated all the rest space to.