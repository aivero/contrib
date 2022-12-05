FROM gitlab.com:443/aivero/dependency_proxy/containers/alpine:3.15
RUN apk add --no-cache chrony=4.1-r0

RUN mkdir -p /var/lib/chrony/drift && chown -R chrony:chrony /var/lib/chrony/drift

EXPOSE 123/udp

# let docker know how to test container health
HEALTHCHECK CMD chronyc tracking || exit 1
# See https://chrony.tuxfamily.org/doc/4.2/chronyd.html
ENTRYPOINT [ "/usr/sbin/chronyd", "-u", "chrony", "-d"]
