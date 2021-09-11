#!/bin/sh

if [ "$1" = "cleveldb" ]; then
    GOROOT=$3 CGO_ENABLED=1 go build -buildmode=c-archive -tags cleveldb -ldflags '-s -w --extldflags "-static -fpic"' -o $2
else
    GOROOT=$3 CGO_ENABLED=1 go build -buildmode=c-archive -ldflags '-s -w --extldflags "-static -fpic"' -o $2
fi
