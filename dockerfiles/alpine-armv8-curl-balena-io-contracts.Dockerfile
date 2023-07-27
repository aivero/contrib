FROM --platform=linux/arm64 gitlab.com:443/aivero/dependency_proxy/containers/arm64v8/alpine:3.15
RUN apk add --no-cache curl
WORKDIR /contracts
RUN curl -LJO https://github.com/aivero/balena-io-contracts/archive/refs/heads/master.zip