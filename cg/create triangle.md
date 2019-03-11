# 创建一个三角形

* 顶点数组对象：Vertex Array Object，VAO
* 顶点缓冲对象：Vertex Buffer Object，VBO
* 索引缓冲对象：Element Buffer Object，EBO或Index Buffer Object，IBO

## 图形渲染管线
两个部分：将3D坐标转换为2D坐标；把2D坐标转变为实际的有颜色的图像。

我们用数组的形式传递3个3D坐标作为图形渲染管线的输入，用来表示一个三角形，这个数组叫做顶点数据；每一个元素都代表一个顶点，而每个顶点都是一个对象，由各种顶点属性表示。

首先经过的是*顶点着色器*。它把一个单独的顶点作为输入，顶点着色器主要的目的是把3D坐标转为另一种3D坐标，同时顶点着色器允许我们对顶点属性进行一些基本操作。

然后是*图元装配*，它将顶点着色器输出的所有顶点作为输入，并将所有点装配成指定图元的形状。比如这次要装配成三角形。

图元装配的输出要传递给*几何着色器*。它把图元形式的一系列顶点的集合作为输入，它可以通过产生新顶点构造出图元来生成其他形状。

几何着色器的输出会传入到*光栅化阶段*，这里它会把图元映射为最终屏幕上相应的像素，生成供片段着色器使用的片段。

*片段着色器*主要目的是计算一个像素的最终颜色，这是所有OpenGL高级效果产生的地方。通常片段着色器包含3D场景的数据（光照、阴影、光的颜色），这些数据用来计算最终像素的颜色。

在所有颜色值确定之后，最终的对象会被传到最后一个阶段，我们叫做*Alpha测试*和*混合*阶段，检测片段的深度，用来判断这个像素是其他物体的前面还是后面，决定是否应该舍弃。这个阶段还会检查*alpha*值（透明度），并对物体进行混合。

虽然管线非常复杂，但在大多数情况下我们只需要配置顶点和片段着色器就行了。几何着色器是可选的，一般选用默认的着色器即可。

## 输入顶点
openGL指定的所有坐标都是3D坐标。但是它不能简单地将3D坐标变换为屏幕上的2D像素；OpenGL只有在三个坐标都在-1到1的范围内才能处理它。
```cpp
float vertices[] = {
  -0.5f, -0.5f, 0.0f,
  0.5f, -0.5f, 0.0f,
  0.0f,  0.5f, 0.0f
};
```
定义完这些顶点数据之后，我们可以把它作为输入发送给图形渲染管线的第一个处理阶段：顶点着色器。它会在GPU上创建内存用于储存我们的顶点数据，还要配置OpenGL如何解释这些内存，并且指定其如何发送给显卡。顶点着色器会处理我们在内存中指定数量的顶点。

我们通过*顶点缓冲对象（VBO）*来管理这个内存，他会在GPU内存（即显存）中储存大量顶点。使用这些缓冲对象的好处是我们可以一次性的发送一大批数据到显卡上，而不是每个顶点发送一次。从CPU把数据发送到显卡很慢，我们希望一次性发送尽可能多的数据。

和OpenGL的其他对象一样，这个缓冲有一个独一无二的ID，所以我们可以使用glGenBuffers函数和一个缓冲ID生成一个VBO对象。
```cpp
// 返回1个缓冲对象
unsigned int VBO;
glGenBuffers(1, &VBO);
```
顶点对象缓冲的类型是GL_ARRAY_BUFFER，OpenGL允许我们同时绑定多个缓冲，只要它们是不同的缓冲类型。我们可以用glBindBuffer函数把新创建的缓冲绑定到GL_ARRAY_BUFFER上：
```cpp
// 指定当前活动缓冲区的对象
glBindBuffer(GL_ARRAY_BUFFER, VBO);
```
这样，我们使用任何（在GL_BUFFER_BUFFER目标上的）缓冲调用都会用来配置当前绑定的缓冲。然后我们调用glBufferData函数，他会把之前定义的顶点数据复制到缓冲的内存中：
```cpp
glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), GL_STATIC_DRAW);
```
glBufferData是一个专门用来把用户定义的数据复制到当前绑定缓冲的函数。它的第一个参数是目标缓冲类型：顶点缓冲对象的类型是GL_ARRAY_BUFFER。第二个参数是指定传输数据的大小（单位是字节，用sizeof即可）。第三个是我们实际要发送的数据。最后一个是我们希望显卡如何管理我们给的数据，有三种方式：
* GL_STATIC_DRAW：数据不会发生改变
* GL_DYNAMIC_DRAW：数据会被改变很多
* GL_STREAM_DRAW：数据每次绘制时都要改版
由于我们的三角形时不会动的，所以用第一个参数即可。这样就将顶点数据存储在显卡的内存中，用VBO这个顶点缓冲对象管理。

总结一下，创建VBO的流程：
1. 创建一个缓冲对象
2. 绑定活动的缓冲对象
3. 将数据复制到显存中

## 顶点着色器
我们要做的第一件事是用着色器语言GLSL编写顶点着色器，然后编译这个着色器，这样我们就可以在程序中使用它了。
```c
#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
  gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
```
GLSL看起来很像c语言。每个着色器都起始于一个版本生命。OpenGL3.3以及更高的版本中，GLSL版本号和OpenGL版本是匹配的。

下一步，使用in关键字，在顶点着色器中声明所有的输入顶点属性。现在我们只关心位置数据，所以我们只需要一个顶点属性。GLSL有一个向量数据类型，它包含1到4个float分量，包含的数量可以从它的后缀数字看出来。由于每个顶点都有一个3D坐标，我们就创建一个vec3输入向量aPos。我们也通过layout(location = 0)设定了输入变量的位置值。

在设置顶点着色器输出是，我们必须把位置数据赋值给预定义的gl_Position变量，它在幕后是vec4类型的。在main函数的最后，我们将gl_Position设置的值会成为该顶点着色器的输出。由于我们的输入是一个3分量的向量，我们必须把它转换为4分量的。我们可以把vec3的数据作为vec4构造器的参数，同时把w分量设置为1

这样顶点着色器的定义就基本完成了。

### 编译着色器
现在我们写好了一个顶点着色器源码（储存在一个c的字符串中），但是为了能够让OpenGL能使用它，我们必须在运行时动态编译它的源码。

首先创建一个着色器对象，跟之前的缓冲对象一样，我们要给它一个ID
```cpp
unsigned int vertexShader;
vertexShader = glCreateShader(GL_VERTEX_SHADER);
```
glCreateShader可以创建不同的着色器，GL_VERTEX_SHADER代表顶点着色器。

接下来把着色器源码附加到着色器对象上，然后编译
```cpp
glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
glCompileShader(vertexShader);
```
glShaderSource的第一个参数是着色器对象ID，第二个参数是要传递的源码字符串数量，第三个参数是顶点着色器真正的源码，最后一个暂且设置为NULL

## 片段着色器
片段着色器做的是在计算像素最后的颜色输出。

首先跟之前一样，先编写着色器的源码。
```c
#version 330 core
out vec4 FragColor;

void main()
{
  FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
```
计算机中颜色的表示是一个4元的向量：红色、绿色、蓝色和透明度分量。着色器需要一个输出变量，这个变量就是一个4个分量的向量。在上面的源码中我们可以看到，我们用out关键字声明输出的变量。

接下来和顶点着色器一样，创建一个着色器，然后编译它。
```cpp
unsigned int fragmentShader;
fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
glCompileShader(fragmentShader);
```
这里和之前的区别是，我们创建着色器的时候用的是GL_FRAGMENT_SHADER表示这是一个片段着色器。

## 着色器程序
现在两个着色器对象已经创建和编译完毕了。但是要使用他们两个，我们还需要把他们链接成为一个着色器程序对象，然后再渲染对象的时候激活这个着色器程序。已激活的着色器程序的着色器将再我们发送渲染调用的时候被使用。

*这里的步骤和程序的编译执行有一点相似*

当链接一个着色器到一个程序的时候，它会把每个着色器的输出链接到下个着色器的输入。当输出和输入不匹配的时候会得到一个链接错误。

现在我们需要创建一个程序对象。
```cpp
unsigned int shaderProgram;
shaderProgram = glCreateProgram();
```
调用glCreateProgram函数，同样会返回一个唯一的ID。现在可以将着色器附加到程序对象上，并用glLinkProgram链接它们：
```cpp
glAttachShader(shaderProgram, vertexShader);
glAttachShader(shaderProgram, fragmentShader);
glLinkProgram(shaderProgram);
```
最后用glUseProgram激活这个程序对象
```cpp
glUseProgram(shaderProgram);
```
激活成功之后，每个着色器调用和渲染调用都会使用这个程序对象。

最后还有一个步骤，删除之前的着色器对象——因为我们已经将它们链接到了程序中，因此不会再用到它们了。

## 链接顶点属性
利用glVertexAttribPointer来告诉OpenGL如何解析顶点数据：
```cpp
glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void*)0);
glEnableVertexAttribArray(0);
```
* 第一个参数是我们要配置的顶点属性。由于我们的顶点着色器中layout(location=0)定义了顶点属性position的位置值，我们希望数据传递到这一个顶点属性中，所以我们传入0.
* 第二个参数指定属性大小。顶点属性是一个3元的坐标，由3个值组成。
* 第三个参数是数据类型，GL_FLOAT代表浮点值
* 下个参数定义我们是否希望数据被标准化。
* 第五个参数叫做步长，它告诉我们再连续的顶点属性组之间的间隔。由于我们传入的数据都是3元的坐标值，因此设为sizeof(float) * 3，单位是字节
* 最后的参数类型是void*，所以要进行这种位置转换。它表示数据再缓冲中的起始位置的偏移量。

## 顶点数组对象VAO
一个储存了顶点属性配置和应使用的VBO的顶点数组对象。一般打算绘制多个物体时，要生成/配置所有VAO，然后储存他们供后面使用。

它可以像VBO那样被绑定，然后所有的*顶点属性调用都会储存再这个VAO中*。这样的好处时，当配置顶点属性指针时，只需要将这些调用执行一次，之后再绘制物体的时候只需要绑定相应的VAO就行了。