# Create a Window

## glfwInit初始化GLFW
用glfwWindowHint函数来配置GLFW。它的第一个参数代表选项的名称，可以在GLFW_开头的枚举值中选择；第二个参数接受一个整型，用来设置这个选项的值。
```cpp
glfwInit();
glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
//glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);

```
## 创建一个窗口
使用glfwCreateWindow函数，返回一个GLFWwindow类型的指针。第一个和第二个参数代表窗口的宽和高，第三个参数是窗口的标题，最后两个参数暂时忽略。

## GLAD
GLAD是用来管理OpenGL的函数指针的，所以在调用任何OpenGL的函数之前我们要初始化GLAD。
```cpp
if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) o{
  std::cout << "Failed to initialize GLAD" << std::endl;
  return -1;
}
```
给GLAD传入了用来加载系统相关对的OpenGL函数指针地址的函数。GLFW给我们的是glfwGetProcAddress，它根据我们的编译系统定义了正确的函数。

## 视口（ViewPort）
告诉OpenGL窗口的大小和位置。
```cpp
glViewPort(0, 0, 800, 600);
```
第一个和第二个参数是窗口左下角位置，第三第四个是窗口大小。

当用户改变窗口大小的时候，视口大小也要被调整，因此，这时候我们就需要一个回调函数，在窗口大小被调整时被调用。
```cpp
void framebuffer_size_callback(GLFWwindow* window, int width, int height)
{
  glViewport(0, 0, width, height);
}
glfwSetFramebufferSizeCallback(window, frame_size_callback);
```

## 循环
在用户关闭窗口之前，我们希望它能够不断绘制图像并能够接受用户输入。因此，我们需要在程序中添加一个while循环。
```cpp
while (!glfwWindowShouldClose(window)) { // 检查GLFW是否被要求退出
  glfwSwapBuffers(window); // 交换眼色缓冲，显示图像
  glfwPollEvents(); // 检查是否有触发时间、更新窗口状态，调用对应的回调函数
}
glfwTerminate();
```
渲染循环结束，利用glfwTerminate释放/删除分配的所有资源。

## 用户输入
使用glfwGetKey函数，它需要一个窗口以及一个按键作为输入。这个函数返回这个键是否被按下。
```cpp
void processInput(GLFWwindow* window) {
  if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
    glfwSetWindowShouldClose(window, true);
  }
}
// main
while (!glfwWindowShouldClose(window)) {
  processInput(window);

  glfwSwapBuffers(window);
  glfwPollEvents();
}
```
## 渲染
在渲染循环中，我们要把所有的渲染操作放进去。
```cpp
// 渲染循环
while(!glfwWindowShouldClose(window)) {
  // 输入
  processInput(window);

  // 渲染指令
  glClearColor(0.2f, 0.3f, 0.3f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT);

  // 检查并调用事件，交换缓冲
  glfwPollEvents();
  glfwSwapBuffers(window);
}
```