#!/bin/sh

if ! [ -x "$(command -v docker)" ]; then
    echo "Please install docker and docker-compose: https://docs.docker.com/get-docker/"
    exit 1
fi

if [[ `uname -m` == 'arm64' ]]; then
  docker compose --file dev.m1.yml up -d --build
else 
  docker compose --file dev.yml up -d --build
fi