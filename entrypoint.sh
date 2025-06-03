#!/bin/sh
set -e

if [ -z "$ALIYUN_ACCESS_KEY_ID" ] || [ -z "$ALIYUN_ACCESS_KEY_SECRET" ] || [ -z "$ALIYUN_REGION_ID" ]; then
  echo "请设置 ALIYUN_ACCESS_KEY_ID, ALIYUN_ACCESS_KEY_SECRET, ALIYUN_REGION_ID 环境变量"
  exit 1
fi

aliyun configure set \
  --profile AkProfile \
  --mode AK \
  --access-key-id "$ALIYUN_ACCESS_KEY_ID" \
  --access-key-secret "$ALIYUN_ACCESS_KEY_SECRET" \
  --region "$ALIYUN_REGION_ID"

aliyunlog configure "$ALIYUN_ACCESS_KEY_ID" "$ALIYUN_ACCESS_KEY_SECRET" "$ALIYUN_REGION_ID.log.aliyuncs.com"

exec /usr/local/bin/mcp-server
