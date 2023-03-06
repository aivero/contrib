FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/amd64/alpine:latest
RUN apk add --no-cache py3-pip
RUN pip3 install --ignore-installed conan==1.59.0
