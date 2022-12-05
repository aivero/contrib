FROM gitlab.com:443/aivero/dependency_proxy/containers/arm64v8/alpine:latest
RUN apk add --no-cache py3-pip make gcc g++ binutils binutils-gold zlib-dev zlib-static cmake git gawk bison rsync
RUN pip3 install --ignore-installed conan