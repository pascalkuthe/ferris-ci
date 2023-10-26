#! /bin/bash

ssh remote-docker <<EOF
  sudo bash -c 'echo "{\"experimental\": true}" > /etc/docker/daemon.json'
  sudo systemctl restart docker
EOF