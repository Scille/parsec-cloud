#!/bin/bash
set -eu

AWS_ACCESS_KEY_ID="${RUSTFS_ACCESS_KEY}"
AWS_SECRET_ACCESS_KEY="${RUSTFS_SECRET_KEY}"

export AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY

AWS_CLI_SHARED_OPTS=(
    --bucket "$BUCKET_NAME"
    --region "$RUSTFS_REGION"
    --endpoint-url "https://$RUSTFS_ADDRESS"
    --no-cli-pager
    --no-cli-auto-prompt
)

set -x

aws s3api create-bucket \
    "${AWS_CLI_SHARED_OPTS[@]}"

aws s3api put-bucket-versioning \
    "${AWS_CLI_SHARED_OPTS[@]}" \
    --versioning-configuration Status=Enabled

aws s3api list-buckets \
    "${AWS_CLI_SHARED_OPTS[@]}"
