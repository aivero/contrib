# docker build -f bionic-armv8-bootstrap.Dockerfile .
FROM lukechannings/deno:v1.14.3 AS deno
FROM arm64v8/ubuntu:bionic AS builder
RUN apt update && \
    apt install --no-install-recommends -y python3-pip python3-setuptools gcc libpython3.6-dev
RUN CC=/usr/bin/gcc pip3 install --upgrade conan MarkupSafe==1.1.1

FROM arm64v8/ubuntu:bionic
COPY --from=builder /usr/local/bin/conan /usr/local/bin/conan
COPY --from=builder /usr/local/lib/python3.6/dist-packages /usr/local/lib/python3.6/dist-packages
COPY --from=deno /bin/deno /usr/bin/deno
RUN apt update && apt install --no-install-recommends -y software-properties-common && add-apt-repository ppa:git-core/ppa && \
  apt update && apt install --no-install-recommends -y make gcc-7 g++-7 libc6-dev cmake git gawk bison rsync python3-minimal python3-pkg-resources python3-distutils && \
  apt remove -y software-properties-common && apt autoremove -y && rm -rf /var/lib/apt/lists/*
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-7 10
RUN update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-7 10
