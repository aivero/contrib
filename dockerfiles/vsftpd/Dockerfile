# docker build --build-arg ALPINE_VERSION=latest --build-arg USER_NAME=deepcore -t registry.gitlab.com/aivero/open-source/contrib/alpine-vsftpd/linux-armv8:3.8 -f vsftpd/Dockerfile .

ARG ALPINE_VERSION=3.8
FROM --platform=linux/arm64 gitlab.com:443/aivero/dependency_proxy/containers/arm64v8/alpine:$ALPINE_VERSION
LABEL org.opencontainers.image.source https://registry.gitlab.com/aivero/open-source/contrib/docker-alpine-vsftpd
RUN apk add --no-cache vsftpd

ARG USER_NAME=deepcore
ENV USER_NAME=${USER_NAME}

RUN adduser -h /./${USER_NAME} -s /bin/false -D ${USER_NAME}

RUN echo "local_enable=YES" >> /etc/vsftpd/vsftpd.conf \
  && echo "chroot_local_user=YES" >> /etc/vsftpd/vsftpd.conf \
  && echo "allow_writeable_chroot=YES" >> /etc/vsftpd/vsftpd.conf \
  && echo "write_enable=YES" >> /etc/vsftpd/vsftpd.conf \
  && echo "local_umask=022" >> /etc/vsftpd/vsftpd.conf \
  && echo "passwd_chroot_enable=yes" >> /etc/vsftpd/vsftpd.conf \
  && echo 'seccomp_sandbox=NO' >> /etc/vsftpd/vsftpd.conf \
  && echo 'ftp_data_port=2121' >> /etc/vsftpd/vsftpd.conf \
  && echo 'pasv_enable=YES' >> /etc/vsftpd/vsftpd.conf \
  && echo 'pasv_max_port=30100' >> /etc/vsftpd/vsftpd.conf \
  && echo 'pasv_min_port=30090' >> /etc/vsftpd/vsftpd.conf \
  && sed -i "s/anonymous_enable=YES/anonymous_enable=NO/" /etc/vsftpd/vsftpd.conf

ADD vsftpd/docker-entrypoint.sh /
RUN chown $USER_NAME:$USER_NAME /docker-entrypoint.sh
RUN chmod +x /docker-entrypoint.sh
VOLUME /./${USER_NAME}

EXPOSE 2121 21 30090-30100

ENTRYPOINT /docker-entrypoint.sh
