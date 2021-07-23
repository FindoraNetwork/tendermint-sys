#!/bin/sh

CGO_ENABLED=1 go build -buildmode=c-archive -ldflags '-s -w --extldflags "-static -fpic"' -o $1

