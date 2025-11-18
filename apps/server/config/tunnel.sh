#!/bin/bash

user="$TUNNEL_SSH_USER"
ip="$TUNNEL_IP_ADDRESS"
port="$TUNNEL_PORT"

chmod 600 /app/.ssh/pillan_tunnel

ssh -i "/app/.ssh/pillan_tunnel" -L "$port:$ip:$port" "$user@pillan.inf.uct.cl"