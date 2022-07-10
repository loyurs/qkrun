# 容器构建方式

## 1. 利用buildkit进行构建->buildkit_build/

i>通过tcp+TLS进行构建
ii>通过k8s进行构建
https://github.com/moby/buildkit/tree/master/examples/kubernetes
## 2. 利用kaniko进行构建->kaniko_build/
i>通过k8s进行构建


# 运行pod
## 2. 基于kubernetes的statefulset
1. k8s启动一个持久化，web code-server 监听8022端口，可以通过vscode remote连接，也可以web ip:8443直接访问
2. 将/config 映射到主机目录，持久化