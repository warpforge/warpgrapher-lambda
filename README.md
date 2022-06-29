# Warpgrapher + AWS Lambda
[![Build Status](https://github.com/warpforge/warpgrapher-lambda/workflows/Test/badge.svg)](https://github.com/warpforge/warpgrapher-lambda/actions?query=workflow%3A%22Test%22+branch%3Amaster)

This project demonstrates how to run a [warpgrapher](https://github.com/warpforge/warpgrapher) service on an [AWS Lambda](https://aws.amazon.com/lambda) serverless function. 

## External Requirements 

- [OpenSSL lib for MUSL](https://qiita.com/liubin/items/6c94f0b61f746c08b74c)
- [Runing CosmosDB database](https://azure.microsoft.com/en-us/services/cosmos-db/#featured) 
- [AWS Lambda Function](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/) with CosmosDB environment variables:

```bash
export WG_CYPHER_HOST=127.0.0.1
export WG_CYPHER_READ_REPLICAS=127.0.0.1
export WG_CYPHER_PORT=443
export WG_CYPHER_USER=
export WG_CYPHER_PASS=
```

## Dependencies

Rust MUSL toolchain:

```bash
rustup target add x86_64-unknown-linux-musl
```

## Build

```bash
export FUNC_NAME=my-lambda-func
export BOOTSTRAP_ZIP=bootstrap.zip
```

Build the AWS Lambda serverless bootstrap binary and package it in a zip:

```bash
cargo build --release --target x86_64-unknown-linux-musl 
zip -j ${BOOTSTRAP_ZIP} ./target/x86_64-unknown-linux-musl/release/bootstrap
```

## Deploy

Deploy packaged function to AWS lambda:

```bash
aws lambda update-function-code \
    --function-name ${FUNC_NAME} \
    --zip-file fileb://${BOOTSTRAP_ZIP}
```
