FROM rust:1.87-alpine AS builder

RUN apk add --no-cache musl-dev build-base openssl-dev openssl-libs-static pkgconfig cmake make

WORKDIR /app
COPY . .

RUN cargo build --release

FROM ghcr.io/likerainday/mcp-base:latest

ARG ALIYUN_REGION_ID
ARG ALIYUN_ACCESS_KEY_ID
ARG ALIYUN_ACCESS_KEY_SECRET
ARG NACOS_SERVER_ADDR
ARG NACOS_NAMESPACE
ARG NACOS_DATA_ID
ARG NACOS_GROUP
ARG OFFLINE_MODE
ARG CONFIG_FILE

ENV ALIYUN_REGION_ID=${ALIYUN_REGION_ID}
ENV ALIYUN_ACCESS_KEY_ID=${ALIYUN_SLS_ACCESS_KEY_ID}
ENV ALIYUN_ACCESS_KEY_SECRET=${ALIYUN_SLS_ACCESS_KEY_SECRET}
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

EXPOSE 3001

ENTRYPOINT ["/entrypoint.sh"]
