# docker build --build-arg=OS_VERSION=focal --build-arg=RGBD_TOOLKIT_VERSION=master -t aivero-rgbd-toolkit .

ARG OS_VERSION=focal

FROM ubuntu:$OS_VERSION

RUN apt-get update && \
  apt-get install --no-install-recommends -y python3-pip python3-setuptools python3-dev python3-venv python3-wheel git && \
  pip3 install --upgrade conan

RUN conan config install https://gitlab.com/aivero/open-source/conan-config/-/archive/master/conan-config-master.tar.gz -sf conan-config-master
RUN conan config set general.default_profile=linux-x86_64

WORKDIR /WORKDIR

ARG RGBD_TOOLKIT_VERSION=master
ENV CI_COMMIT_REF_SLUG=${RGBD_TOOLKIT_VERSION}

# Alternatively, COPY the directory from your host machine
RUN git clone https://gitlab.com/aivero/open-source/contrib.git -b ${RGBD_TOOLKIT_VERSION}

WORKDIR /WORKDIR/contrib

# You may skip this step if you just want to download a version from the CICD server
RUN conan create aivero-rgbd-toolkit
RUN conan install aivero-rgbd-toolkit/${RGBD_TOOLKIT_VERSION}@ -if /opt/aivero/rgbd_toolkit

WORKDIR /opt/aivero/rgbd_toolkit
COPY entry.sh .

CMD [ "bash", "entry.sh" ]
