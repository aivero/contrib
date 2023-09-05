FROM --platform=linux/arm64 gitlab.com:443/aivero/dependency_proxy/containers/arm64v8/ubuntu:focal AS builder
RUN apt update && \
  apt install --no-install-recommends -y python3-pip python3-setuptools && \
  pip3 install --upgrade conan==1.59.0
FROM --platform=linux/arm64 gitlab.com:443/aivero/dependency_proxy/containers/arm64v8/ubuntu:focal
COPY --from=builder /usr/local/bin/conan /usr/local/bin/conan
COPY --from=builder /usr/local/lib/python3.8/dist-packages /usr/local/lib/python3.8/dist-packages
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && \
  apt install --no-install-recommends -y make clang gcc libc6-dev cmake git gawk bison flex rsync valac python3-minimal python3-pkg-resources python3-distutils && \
  rm -rf /var/lib/apt/lists/*
