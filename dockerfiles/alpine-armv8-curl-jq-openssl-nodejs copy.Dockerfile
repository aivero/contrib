FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/arm64v8/alpine:3.15
RUN apk add --no-cache curl jq openssl nodejs