FROM ubuntu:20.04
RUN apt-get update && \
  apt-get install --no-install-recommends -y openssl && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /opt/webrtc-signalling
COPY ./webrtcsink/bin/server ./server

# OPTIONS:
#     -c, --cert <CERT>                      TLS certificate to use
#         --cert-password <CERT_PASSWORD>    password to TLS certificate
#     -h, --host <HOST>                      Address to listen on [default: 0.0.0.0]
#         --help                             Print help information
#     -p, --port <PORT>                      Port to listen on [default: 8443]
#     -V, --version                          Print version information

ENTRYPOINT ./server