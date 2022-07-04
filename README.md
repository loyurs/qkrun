# 介绍
一个命令行工具
用来快速构建rust开发环境与编译环境
## 快速构建开发环境
code-server webui
vscode-remote client-connection
## 自动化编译
## 结合pulimi 与k8s快速构建
Infrastructure as Code

## 注入一个buildkit工具镜像
buildkit工具镜像，可以利用此工具镜像进行构建，然后输出构建完成的镜像
构建：
buildctl build --frontend gateway.v0 --opt source=docker/dockerfile --opt context=https://github.com/loyurs/qkrun.git#master:docker/rust_code_server/ --output type=tar,dest=out.tar
导入：
docker import
利用buildkit镜像进行构建并输出文件
docker run --rm -v /home/:/home/ buildctl build --frontend gateway.v0 --opt source=docker/dockerfile --opt context=https://github.com/loyurs/qkrun.git#master:docker/rust_code_server/ --output type=tar,dest=/home/geneout.tar

容器化一次性构建参考资料
https://github.com/moby/buildkit#output


# 功能指引
1. 构建并运行code server
2. 多语言开发环境
利用一个docker-compose 容器，嵌套  /var/run/docker.socks:/     来执行docker-compose


## 利用docker buildkit 进行构建
buildkit构建，带远程上下文进行构建，可以直接对github的代码上下文进行构建