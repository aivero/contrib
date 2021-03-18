# docker build -f bionic-armv8-bootstrap.Dockerfile .
FROM arm64v8/ubuntu:bionic AS builder
RUN apt update && \
    apt install --no-install-recommends -y python3-pip python3-setuptools gcc libpython3.6-dev
RUN CC=/usr/bin/gcc pip3 install --upgrade conan

FROM arm64v8/ubuntu:bionic
COPY --from=builder /usr/local/bin/conan /usr/local/bin/conan
COPY --from=builder /usr/local/lib/python3.6/dist-packages /usr/local/lib/python3.6/dist-packages
RUN apt update && \
  apt install --no-install-recommends -y libc6-dev libatomic1 python3-minimal python3-pkg-resources && \
  rm -rf /var/lib/apt/lists/*
RUN conan config install https://codeload.github.com/aivero/conan-config/zip/master -sf conan-config-master
RUN conan config set general.default_profile=linux-armv8
RUN conan install git/2.30.0@ -g tools -if /usr/local/bin
RUN conan install rust/1.0.0@
RUN chmod -R 777 /root
