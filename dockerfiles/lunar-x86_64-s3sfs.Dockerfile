# docker build -f ./contrib/dockerfiles/lunar-x86_64-s3sfs.Dockerfile  -t registry.gitlab.com/aivero/open-source/contrib/lunar-s3sfs/linux-x86_64:latest  ./contrib/dockerfiles/
FROM --platform=linux/arm64 gitlab.com:443/aivero/dependency_proxy/containers/amd64/rust:1.72-slim-buster as builder

WORKDIR /s3sfs
RUN cargo install --root /s3sfs s3s-fs --features binary --target-dir .
FROM --platform=linux/amd64 gitlab.com:443/aivero/dependency_proxy/containers/ubuntu:23.04 as runner
WORKDIR /s3sfs/bin
WORKDIR /s3sfs
COPY --from=builder /s3sfs/bin/s3s-fs /s3sfs/bin/s3s-fs
ENTRYPOINT [ "/s3sfs/bin/s3s-fs" ]
# See https://github.com/Nugine/s3s/blob/main/scripts/s3s-fs.sh for reference