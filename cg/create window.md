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