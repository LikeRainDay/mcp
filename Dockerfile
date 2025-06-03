FROM rust:1.77-alpine AS builder

# openssl-dev 和 pkgconfig 仅在编译 Rust 项目时需要，若你的 Rust 项目依赖 openssl（如 reqwest、openssl crate），则必须保留。pkgconfig 用于构建时查找 openssl 等库。musl-dev 也是编译静态链接二进制时常用。
# 如果你的 Rust 项目不依赖 openssl，可以去掉 openssl-dev 和 pkgconfig，但大多数网络相关项目都需要。
RUN apk add --no-cache musl-dev openssl-dev pkgconfig cmake make

WORKDIR /app
COPY . .

RUN cargo build --release

FROM rig-mcp-base:latest

ARG ALIYUN_SLS_ENDPOINT
ARG ALIYUN_SLS_ACCESS_KEY_ID
ARG ALIYUN_SLS_ACCESS_KEY_SECRET
ARG NACOS_SERVER_ADDR
ARG NACOS_NAMESPACE
ARG NACOS_DATA_ID
ARG NACOS_GROUP
ARG OFFLINE_MODE
ARG CONFIG_FILE

ENV ALIYUN_SLS_ENDPOINT=${ALIYUN_SLS_ENDPOINT}
ENV ALIYUN_SLS_ACCESS_KEY_ID=${ALIYUN_SLS_ACCESS_KEY_ID}
ENV ALIYUN_SLS_ACCESS_KEY_SECRET=${ALIYUN_SLS_ACCESS_KEY_SECRET}
ENV NACOS_SERVER_ADDR=${NACOS_SERVER_ADDR}
ENV NACOS_NAMESPACE=${NACOS_NAMESPACE}
ENV NACOS_DATA_ID=${NACOS_DATA_ID}
ENV NACOS_GROUP=${NACOS_GROUP}
ENV OFFLINE_MODE=${OFFLINE_MODE}
ENV CONFIG_FILE=${CONFIG_FILE}

COPY --from=builder /app/target/release/mcp-server /usr/local/bin/mcp-server
COPY entrypoint.sh /entrypoint.sh
COPY config.yml /config.yml

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
