# 如何使用docker构建项目

## 如何安装docker

### win10

直接上官网下载docker-desktop，然后打开win10的Hyper-V设置，以及cpu的虚拟硬件功能。

### Ubuntu（16.04）

```
$ sudo apt-get update

$ sudo apt-get install \
    linux-image-extra-$(uname -r) \
    linux-image-extra-virtual
$ sudo apt-get update

$ sudo apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    software-properties-common
$ sudo add-apt-repository \
    "deb [arch=amd64] https://mirrors.ustc.edu.cn/docker-ce/linux/ubuntu \
    $(lsb_release -cs) \
    stable"
$ sudo apt-get update

$ sudo apt-get install docker-ce
$ sudo systemctl enable docker
$ sudo systemctl start docker
```

## 镜像

docker是根据镜像来构建容器的。可以用`docker image ls`来查看本地有什么镜像。如果啥都没有，可以上dockerhub，找到合适的镜像，再用`docker image pull username/repo:tagname`拉一个镜像。

### 镜像换源

docker的国外源速度比较慢，我们可以用国内的源。win10换源比较简单，找到可以用的源（清华大学的docker源或者用阿里云的镜像加速器），然后打开docker desktop的setting页面，找到Daemon选项，在registry mirrors中加入你的源。

ubuntu换源稍微复杂一点：

编辑`vi /etc/docker/daemon.json`，然后加上：

```
{
"registry-mirrors": ["example.com"]
}
```

## 构建一个容器

```
docker container run --name container-name -d -p port1:port2 -it image-name:tagname
```

利用这个命令构建一个容器，如果容器已经存在的话，可以先用`docker container rm ...`将其删除。而port1和port2是端口映射的部分，port1是宿主机的端口，port2是容器的端口，我们可以在宿主机上通过访问port1的方式访问port2。

用`docker container ls -a`可以列出所有存在的容器（包括没有在运行的），这时候我们就可以看到我们的容器id了。

用`docker exec -it container-name bash`可以进入这个容器的终端。

用`docker cp from cotainer-name:to`可以将宿主机的文件复制到容器。

## 将容器变成镜像

先用`docker ps`或者`docker container ls`找出要打包的container 的id。

然后用`docker commit -m "comment" -a "" container-id [new image-name]`将其打包成镜像。

我们还可以用`docker image tag image-name username/repo:tag`的方式给这个镜像打上tag。

重新构建一下：`docker image build -t username/repo:tag`。

最后发布：`docker image push username/repo:tag`.



---

*reference*:

[阮一峰老师的博客](<http://www.ruanyifeng.com/blog/2018/02/docker-tutorial.html>)

[将docker容器打包成镜像](https://blog.csdn.net/medivhq/article/details/76176801)

[修改ubuntu的docker源](https://blog.csdn.net/skh2015java/article/details/82631633)

[在ubuntu安装docker](https://yeasy.gitbooks.io/docker_practice/install/ubuntu.html)