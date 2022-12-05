FROM gitlab.com:443/aivero/dependency_proxy/containers/alpine:3.15
RUN apk add --no-cache openssl
