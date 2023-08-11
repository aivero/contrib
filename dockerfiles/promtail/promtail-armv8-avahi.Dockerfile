FROM --platform=linux/arm64 gitlab.com:443/aivero/dependency_proxy/containers/grafana/promtail:2.8.3-arm64
RUN apt-get update && apt-get install -yq --no-install-recommends \
    avahi-utils \
    libnss-mdns \
    jq && apt-get clean && rm -rf /var/lib/apt/lists/*

ADD promtail/start_promtail_with_avahi_discovery.sh /start_promtail_with_avahi_discovery.sh
ADD promtail/promtail.yaml /etc/promtail/promtail.yaml

ENTRYPOINT ["bash", "/start_promtail_with_avahi_discovery.sh"]
