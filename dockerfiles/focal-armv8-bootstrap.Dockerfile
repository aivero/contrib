FROM lukechannings/deno:v1.14.3 AS deno
FROM arm64v8/ubuntu:focal AS builder
RUN apt update && \
  apt install --no-install-recommends -y python3-pip python3-setuptools && \
  pip3 install --upgrade conan
FROM arm64v8/ubuntu:focal
COPY --from=builder /usr/local/bin/conan /usr/local/bin/conan
COPY --from=builder /usr/local/lib/python3.8/dist-packages /usr/local/lib/python3.8/dist-packages
COPY --from=deno /bin/deno /usr/bin/deno
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && \
  apt install --no-install-recommends -y make clang libc6-dev cmake git gawk bison flex rsync valac python3-minimal python3-pkg-resources python3-distutils && \
  apt remove -y gcc g++ && \
  rm -rf /var/lib/apt/lists/*
