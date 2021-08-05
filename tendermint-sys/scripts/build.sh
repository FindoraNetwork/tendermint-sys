#!/bin/sh

if [ "$1" = "cleveldb" ]; then
    CGO_ENABLED=1 go build -buildmode=c-archive -tags cleveldb -gcflags '-+' -ldflags '-s -w --extldflags "-static -fpic"' -o $2
else
    CGO_ENABLED=1 go build -buildmode=c-archive -gcflags '-+' -ldflags '-s -w --extldflags "-static -fpic"' -o $2
fi
