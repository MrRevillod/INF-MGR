#!/bin/bash

user="$TUNNEL_SSH_USER"
ip="$TUNNEL_IP_ADDRESS"
port="$TUNNEL_PORT"

ssh -L "$port:$ip:$port" "$user@pillan.inf.uct.cl"