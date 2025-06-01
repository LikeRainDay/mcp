FROM rust:1.77-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig cmake make

WORKDIR /app
COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates jq

RUN wget https://aliyuncli.alicdn.com/aliyun-cli-linux-latest-amd64.tgz \
    && tar -xvzf aliyun-cli-linux-latest-amd64.tgz \
    && rm aliyun-cli-linux-latest-amd64.tgz \
    && mv aliyun /usr/local/bin/ \
    # 兼容 musl libc 的动态链接
    && mkdir -p /lib64 \
    && ln -s /lib/libc.musl-x86_64.so.1 /lib64/ld-linux-x86-64.so.2

COPY --from=builder /app/target/release/rig-mcp-server /usr/local/bin/rig-mcp-server

COPY entrypoint.sh /entrypoint.sh
COPY config.yml /config.yml

RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
