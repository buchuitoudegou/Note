# 在macos配置glfw

## 安装cmake
```
brew install
```

## 安装glfw
下载glfw的[源码](http://www.glfw.org/download.html)，解压，在命令行进入这个文件夹。

执行：`mkdir build/ & cmake . -B build/`

完成后，进入glfw/build，执行`make install`，这时候，如果/usr/local/lib有了glfw的静态链接文件和/usr/local/include有了glfw文件夹，就说明安装成功了。

## 安装glad
下载glad的[源码](http://glad.dav1d.de/)：
1. 将语言设置为 C/C++，在 API 选项中，选择 3.3 及以上的 OpenGL 版本。
2. 将模式 Profile 设置为 Core ，保证生成加载器 Generate a loader 选项是选中的。
3. 先暂时忽略拓展 Extensions 中内容。
4. 点击生成 Generate 。
5. 下载生成的 zip 包（包含 glad.c、glad.h 和 khrplatform.h），解压后 include 目录下的 glad 和 KHR 文件夹复制到 /usr/local/include，glad.c 添加到项目中。

完成这两项工作之后，glfw就可以用了。

## makefile
```makefile
CC = g++
CFLAGS = -std=c++11 -w -g
bin = bin
build = build

mkdir -p $(bin)
mkdir -p $(build)

$(bin)/test : $(build)/glad.o $(build)/test-opengl.o
	g++ $(build)/glad.o $(build)/test-opengl.o -lglfw $(CFLAGS) -o $@

$(build)/test-opengl.o : test-opengl.cpp
	g++ -c -o $@ $< $(CFLAGS)

$(build)/glad.o : glad.c
	g++ -c -o $@ $< $(CFLAGS)

run:
	$(bin)/test
clean:
	rm -rf build/
	mkdir build/
	rm -rf bin/
	mkdir bin/
```