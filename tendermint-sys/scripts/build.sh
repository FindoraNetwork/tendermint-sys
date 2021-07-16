#!/bin/sh

CGO_ENABLED=1 CGO_LDFLAGS="-static" go build -buildmode=c-archive -ldflags '-s -w --extldflags "-static -fpic"' -o $1

