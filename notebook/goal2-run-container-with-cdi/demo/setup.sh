#!/bin/bash

set -eu

docker pull python:slim
docker create --name python-slim-container python:slim
mkdir rootfs
cd rootfs
docker export python-slim-container | tar -C ./ -xvf -
docker rm python-slim-container
