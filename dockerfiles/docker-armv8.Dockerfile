FROM --platform=linux/arm64 gitlab.com:443/aivero/dependency_proxy/containers/arm64v8/docker:20
RUN apk add --no-cache py3-pip
RUN pip3 install --ignore-installed conan==1.59.0
