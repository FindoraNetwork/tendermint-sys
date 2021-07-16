#!/bin/sh

go build -buildmode=c-archive -ldflags '-s -w --extldflags "-static -fpic"' -o $1

