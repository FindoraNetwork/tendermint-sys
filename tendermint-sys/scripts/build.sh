#!/bin/sh

CGO_ENABLED=1 go build -buildmode=c-archive -tags cleveldb -ldflags '-s -w --extldflags "-static -fpic"' -o $1

