# 如何使用自动构建工具Jenkins
Jenkins是一款用于自动构建/自动集成的工具，可以通过Github web hook来获取github上的仓库信息，然后在push到特定分支之后，自动进行持续集成和构建。

## Ubuntu下安装Jenkins
```
wget -q -O - https://pkg.jenkins.io/debian/jenkins.io.key | sudo apt-key add -
sudo sh -c 'echo deb http://pkg.jenkins.io/debian-stable binary/ > /etc/apt/sources.list.d/jenkins.list'
sudo apt-get update
sudo apt-get install jenkins
```
安装完成后，可以通过`service start jenkins`启动。