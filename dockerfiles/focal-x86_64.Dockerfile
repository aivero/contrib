FROM denoland/deno:bin-1.14.3 AS deno
FROM --platform=linux/amd64 ubuntu:focal AS builder
RUN apt update && \
  apt install --no-install-recommends -y python3-pip python3-setuptools && \
  pip3 install --upgrade conan
FROM --platform=linux/amd64 ubuntu:focal
COPY --from=builder /usr/local/bin/conan /usr/local/bin/conan
COPY --from=builder /usr/local/lib/python3.8/dist-packages /usr/local/lib/python3.8/dist-packages
COPY --from=deno /deno /usr/bin/deno
RUN apt update && \
  apt install --no-install-recommends -y libc6-dev libatomic1 python3-minimal python3-pkg-resources ca-certificates git git-lfs curl && \
  rm -rf /var/lib/apt/lists/*
RUN deno run --unstable --allow-all --import-map \
  https://gitlab.com/aivero/open-source/cicd/-/raw/stable/import_map.json \
  https://gitlab.com/aivero/open-source/cicd/-/raw/stable/lib/es6/src/GenerateConfig.js || true

