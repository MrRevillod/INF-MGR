#!/usr/bin/env bash

privatekey="$WIREGUARD_PRIVATE_KEY"
presharedkey="$WIREGUARD_PRESHARED_KEY"
wireguard_ip="$WIREGUARD_IP"
wireguard_peer="$WIREGUARD_PEER"
wireguard_endpoint="$WIREGUARD_ENDPOINT"
kubernetes_endpoint="$KUBERNETES_ENDPOINT"

echo "Setting up WireGuard VPN..."
echo "WireGuard IP: $wireguard_ip"

echo "$privatekey" > privatekey
echo "$presharedkey" > presharedkey

ip link add dev wg0 type wireguard

ip address add dev wg0 "$wireguard_ip" peer "$wireguard_peer"

wg set wg0 private-key privatekey \
 peer UBi3x7Cjv4lPABuWcbv7yTOgiDyb2ElLN+39J1gHqnU= preshared-key \
 presharedkey endpoint "$wireguard_endpoint":51820 \
 allowed-ips "$kubernetes_endpoint"/32,"$wireguard_peer"/32

ip link set up dev wg0

ip route add "$kubernetes_endpoint"/32 via "$wireguard_peer" dev wg0