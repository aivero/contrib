# docker build -t registry.gitlab.com/aivero/open-source/contrib/focal-x86_64-balena/linux-x86_64 -f focal-x86_64-balena.Dockerfile .
FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/amd64/ubuntu:focal

RUN apt-get update && apt-get install --no-install-recommends -y pigz wget unzip ca-certificates curl gnupg lsb-release jq && \
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null && \
  apt-get update && apt-get install -y --no-install-recommends docker-ce docker-ce-cli containerd.io && rm -rf /var/lib/apt/lists/*
RUN mkdir -p ~/.docker/cli-plugins/ && curl -SL https://github.com/docker/compose/releases/download/v2.2.3/docker-compose-linux-x86_64 -o ~/.docker/cli-plugins/docker-compose && chmod +x ~/.docker/cli-plugins/docker-compose

ARG BALENA_CLI_VERSION=v15.0.0
WORKDIR /balena
RUN wget https://github.com/balena-io/balena-cli/releases/download/${BALENA_CLI_VERSION}/balena-cli-${BALENA_CLI_VERSION}-linux-x64-standalone.zip -O balena.zip && unzip balena.zip && mv balena-cli/balena . && chmod +x balena && rm -rf balena-cli balena.zip
ENV PATH=$PATH:/balena
