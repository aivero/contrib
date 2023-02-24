FROM --platform=linux/arm64 nvcr.io/nvidia/l4t-base:r32.7.1
RUN apt-get update && apt-get install --no-install-recommends -y ca-certificates git python3-pip && pip3 install setuptools && pip3 install conan==1.59.0 && rm -rf /var/lib/apt/lists/*
ADD --chown=root:root https://repo.download.nvidia.com/jetson/jetson-ota-public.asc /etc/apt/trusted.gpg.d/jetson-ota-public.asc
RUN chmod 644 /etc/apt/trusted.gpg.d/jetson-ota-public.asc
RUN echo 'deb https://repo.download.nvidia.com/jetson/common r32.7 main' >> /etc/apt/sources.list.d/nvidia-l4t-apt-source.list && echo 'deb https://repo.download.nvidia.com/jetson/t210 r32.7 main' >> /etc/apt/sources.list.d/nvidia-l4t-apt-source.list
RUN apt-get update && apt-get install -y --no-install-recommends cuda-compiler-10-2 && rm -rf /var/lib/apt/lists/*
RUN echo "deb http://ports.ubuntu.com/ubuntu-ports focal main restricted" >> /etc/apt/sources.list && apt-get update && apt-get install -y libc6 udev && rm -rf /var/lib/apt/lists/*
