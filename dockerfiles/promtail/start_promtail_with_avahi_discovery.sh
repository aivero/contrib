#!/bin/sh
avahi_discovery() {
    # Avahi discover deepserver
    printf "Performing avahi discovery\n"
    while true; do
        sleep 1
        if (avahi-resolve-host-name aivero.local | awk '{print $2}' >/dev/null); then
            DS_IP=$(avahi-resolve-host-name aivero.local | awk '{print $2}')
            echo "Resolved Deepserver address to $DS_IP"
            break #Abandon the loop.
        fi
    done
}

CONFIG_FILE=/etc/promtail/promtail.yaml

echo "Starting promtail from from start_promtail_with_avahi_discovery.sh"
export DS_IP
# Check if FTP copied config file exists
if [ "$LOKI_URL" ]; then
    printf "We have a LOKI_URL set, will send logs here: %s\n" "$LOKI_URL"

elif [ "$AIVERO_DCD_CFG_BALENA" ]; then
    printf "We have a AIVERO_DCD_CFG_BALENA set, will try to construct a loki URL based on the dns_resolve_method: %s\nNote that we cannot configure credentials this way\n" "$LOKI_URL"

    DNS_RESOLVE_METHOD=$(echo "$AIVERO_DCD_CFG_BALENA" | jq .dns_resolve_method)
    printf "DNS_RESOLVE_METHOD: %s\n" "$DNS_RESOLVE_METHOD"
    if [ "$DNS_RESOLVE_METHOD" = "\"Avahi\"" ]; then
        avahi_discovery
    elif [ "$DNS_RESOLVE_METHOD" = "\"EtcHostsEntry\"" ]; then
        DS_IP=$(echo "$AIVERO_DCD_CFG_BALENA" | jq .ip )
    elif [ "$DNS_RESOLVE_METHOD" = "\"None\"" ]; then
        GRAPHQL_ENDPOINT=$(echo "$AIVERO_DCD_CFG_BALENA" | jq .graphql.endpoint)
        # GRAPHQL_ENDPOINT looks like this "wss://ds.aivero.lan/v1/graphql", but we only want ds.aivero.lan
        DS_IP=$(echo "$GRAPHQL_ENDPOINT" | cut -d'/' -f3 | cut -d':' -f1)
    else
        printf "No valid DNS_RESOLVE_METHOD set\n"
        exit 1
    fi
    if [ -n "$DS_IP" ] && [ "$DS_IP" != "null" ]; then
        LOKI_URL="http://$DS_IP:3100/loki/api/v1/push"
    else
        printf "Could not resolve DS_IP from AIVERO_DCD_CFG_BALENA\n"
        exit 1
    fi
else
    # Just performing avahi discovery
    export LOKI_URL
    avahi_discovery
    if [ -n "$DS_IP" ] && [ "$DS_IP" != "null" ]; then
        LOKI_URL="http://$DS_IP:3100/loki/api/v1/push"
    else
        printf "Could not find DS_IP from Avahi\n"
        exit 1
    fi

fi
echo "LOKI_URL: $LOKI_URL"

echo "Config file content:"
cat "$CONFIG_FILE"
promtail -config.file="$CONFIG_FILE" -config.expand-env=true -print-config-stderr
