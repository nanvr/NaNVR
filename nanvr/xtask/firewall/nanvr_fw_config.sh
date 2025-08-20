#!/usr/bin/env bash
# Basic script to add / remove firewall configuration for NaNVR
# Usage: ./nanvr_fw_config.sh add|remove
# Exit codes:
# 1 - Invalid command
# 2 - Invalid action
# 3 - Failed to copy UFW configuration
# 99 - Firewall not found
# 126 - pkexec failed - Request dismissed
# todo: port script to rust

firewalld_cfg() {
    # Iterate around each active zone
    for zone in $(firewall-cmd --get-active-zones | grep -P '^\w+'); do
        if [ "${1}" == 'add' ]; then
            # If running or permanent nanvr service is missing, add it
            if ! firewall-cmd --zone="${zone}" --list-services | grep 'nanvr' >/dev/null 2>&1; then
                firewall-cmd --zone="${zone}" --add-service='nanvr'
            fi
            if ! firewall-cmd --zone="${zone}" --list-services --permanent | grep 'nanvr' >/dev/null 2>&1; then
                firewall-cmd --zone="${zone}" --add-service='nanvr' --permanent
            fi
        elif [ "${1}" == 'remove' ]; then
            # If running or persistent nanvr service exists, remove it
            if firewall-cmd --zone="${zone}" --list-services | grep 'nanvr' >/dev/null 2>&1; then
                firewall-cmd --zone="${zone}" --remove-service='nanvr'
            fi
            if firewall-cmd --zone="${zone}" --list-services --permanent | grep 'nanvr' >/dev/null 2>&1; then
                firewall-cmd --zone="${zone}" --remove-service='nanvr' --permanent
            fi
        else
            exit 2
        fi
    done
}

ufw_cfg() {
    # Try and install the application file
    if ! ufw app info 'nanvr'; then
        # Pull application file from local build first if the script lives inside it
        if [ -f "$(dirname "$(realpath "${0}")")/ufw-nanvr" ]; then
            cp "$(dirname "$(realpath "${0}")")/ufw-nanvr" '/etc/ufw/applications.d/'
        elif [ -f '/usr/share/nanvr/ufw-nanvr' ]; then
            cp '/usr/share/nanvr/ufw-nanvr' '/etc/ufw/applications.d/'
        else
            exit 3
        fi
    fi

    if [ "${1}" == 'add' ] && ! ufw status | grep 'nanvr' >/dev/null 2>&1; then
        ufw allow 'nanvr'
    elif [ "${1}" == 'remove' ] && ufw status | grep 'nanvr' >/dev/null 2>&1; then
        ufw delete allow 'nanvr'
    else
        exit 2
    fi
}

iptables_cfg() {
    first_port_match_count=$(iptables -S | grep -c '9945')
    second_port_match_count=$(iptables -S | grep -c '9946')
    if [ "${1}" == 'add' ]; then
        if [ "$first_port_match_count" == "0" ] || [ "$second_port_match_count" == "0" ]; then
            if [ ! -d '/etc/iptables' ]; then
                mkdir '/etc/iptables'
            fi

            iptables -I OUTPUT -p tcp --sport 9945 -j ACCEPT
            iptables -I INPUT -p tcp --dport 9945 -j ACCEPT
            iptables -I OUTPUT -p udp --sport 9945 -j ACCEPT
            iptables -I INPUT -p udp --dport 9945 -j ACCEPT
            iptables -I OUTPUT -p tcp --sport 9946 -j ACCEPT
            iptables -I INPUT -p tcp --dport 9946 -j ACCEPT
            iptables -I OUTPUT -p udp --sport 9946 -j ACCEPT
            iptables -I INPUT -p udp --dport 9946 -j ACCEPT
            iptables-save >/etc/iptables/rules.v4
        fi
    elif [ "${1}" == 'remove' ]; then
        if [ "$first_port_match_count" == "4" ] || [ "$second_port_match_count" == "4" ]; then
            iptables -D OUTPUT -p tcp --sport 9945 -j ACCEPT
            iptables -D INPUT -p tcp --dport 9945 -j ACCEPT
            iptables -D OUTPUT -p udp --sport 9945 -j ACCEPT
            iptables -D INPUT -p udp --dport 9945 -j ACCEPT
            iptables -D OUTPUT -p tcp --sport 9946 -j ACCEPT
            iptables -D INPUT -p tcp --dport 9946 -j ACCEPT
            iptables -D OUTPUT -p udp --sport 9946 -j ACCEPT
            iptables -D INPUT -p udp --dport 9946 -j ACCEPT
            iptables-save >/etc/iptables/rules.v4
        fi
    else
        exit 2
    fi
}

main() {
    # If we're not root use pkexec for GUI prompt
    if [ "${USER}" == 'root' ]; then
        # Check if firewall-cmd exists and firewalld is running
        if which firewall-cmd >/dev/null 2>&1 && firewall-cmd --state >/dev/null 2>&1; then
            firewalld_cfg "${1,,}"
        # Check if ufw exists and is running
        elif which ufw >/dev/null 2>&1 && ! ufw status | grep 'Status: inactive' >/dev/null 2>&1; then
            ufw_cfg "${1,,}"
        elif which iptables >/dev/null 2>&1; then
            iptables_cfg "${1,,}"
        else
            exit 99
        fi
    else
        pkexec "$(realpath "${0}")" "${@}"
    fi
}

main "${@}"
