#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

. "$DIR/.env"

sudo groupadd nocturne
USERNAME="$(whoami)"
echo "$USERNAME"
sudo usermod -aG nocturne "$USERNAME"

sudo mkdir -p "$LOG_ROOT_DIR/backend"  "$LOG_ROOT_DIR/frontend"
sudo chown -R :nocturne "$LOG_ROOT_DIR"
sudo chmod -R 775 "$LOG_ROOT_DIR"

