#!/bin/bash

LANGUAGE=$1

docker run --rm -i -t --read-only --tmpfs /tmp:rw,noexec,nosuid,size=65536k --tmpfs /home/glot:rw,exec,nosuid,uid=1000,gid=1000,size=131072k -u glot -w /home/glot glot/${LANGUAGE}:latest /bin/bash
