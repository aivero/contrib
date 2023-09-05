FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/denoland/deno:bin-1.14.3 AS deno
FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/amd64/ubuntu:focal AS builder
RUN apt update && \
  apt install --no-install-recommends -y python3-pip python3-setuptools curl jq && \
  pip3 install --upgrade conan==1.59.0 && \
  VERSION=$(curl -s "https://api.github.com/repos/google/go-containerregistry/releases/latest" | jq -r '.tag_name') && \
  OS=Linux && \
  ARCH=x86_64 && \
  curl -sL "https://github.com/google/go-containerregistry/releases/download/${VERSION}/go-containerregistry_${OS}_${ARCH}.tar.gz" > go-containerregistry.tar.gz && \
  tar -zxvf go-containerregistry.tar.gz -C /usr/local/bin/ crane
FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/amd64/ubuntu:focal
COPY --from=builder /usr/local/bin/conan /usr/local/bin/conan 
COPY --from=builder /usr/local/bin/crane /usr/local/bin/crane
COPY --from=builder /usr/local/lib/python3.8/dist-packages /usr/local/lib/python3.8/dist-packages
COPY --from=deno /deno /usr/bin/deno
RUN apt update && \
  apt install --no-install-recommends -y libc6-dev libatomic1 python3-minimal python3-pkg-resources ca-certificates git git-lfs curl && \
  rm -rf /var/lib/apt/lists/*
 