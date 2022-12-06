FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/amd64/alpine:latest
RUN apk add --no-cache py3-pip make gcc g++ binutils binutils-gold zlib-dev zlib-static cmake git gawk bison rsync
RUN pip3 install --ignore-installed conan
