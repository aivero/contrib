FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/amd64/alpine:3.15
RUN apk add --no-cache openssl
