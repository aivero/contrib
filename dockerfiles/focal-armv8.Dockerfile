FROM lukechannings/deno:v1.27.0 AS deno
FROM arm64v8/ubuntu:focal AS builder
RUN apt update && \
  apt install --no-install-recommends -y python3-pip python3-setuptools && \
  pip3 install --upgrade conan
FROM arm64v8/ubuntu:focal
COPY --from=builder /usr/local/bin/conan /usr/local/bin/conan
COPY --from=builder /usr/local/lib/python3.8/dist-packages /usr/local/lib/python3.8/dist-packages
COPY --from=deno /bin/deno /usr/bin/deno
RUN apt update && \
  apt install --no-install-recommends -y libc6-dev libatomic1 python3-minimal python3-pkg-resources ca-certificates git git-lfs curl && \
  rm -rf /var/lib/apt/lists/*
