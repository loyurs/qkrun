#code-server + ssh + vscode_rust_dev_extensions
FROM registry.cn-hangzhou.aliyuncs.com/clouddevs/vscode-extensions:rust AS builder
FROM registry.cn-hangzhou.aliyuncs.com/clouddevs/code-server:ubuntu20-ssh
COPY --from=builder  /opt/extensions /config/extensions
RUN mkdir -p /root/.vscode-server && ln -s /config/extensions /root/.vscode-server/extensions