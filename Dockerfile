#测试通过git进行构建并运行
#DOCKER_BUILDKIT=1 docker build http://115.159.115.244:3000/liloyu/macro_study.git#master -t litong:vk

FROM rust:latest AS builder
ADD . /opt
#设置tuna镜像源加速rust构建
RUN mv /opt/tuna_mirrors/config /usr/local/cargo/
RUN cd /opt && cargo build --release
FROM alpine
COPY --from=builder /opt/src /home/
#