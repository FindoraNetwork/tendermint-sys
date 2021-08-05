#!/bin/sh

export GOROOT="$(go env GOROOT)"

echo "GOROOT = ${GOROOT}"

GO_ROOTFS=$1
PATCH_FILE=$2

echo "Use patch file ${PATCH_FILE}"
echo "Build Rootfs on ${GO_ROOTFS}"

mkdir -p $GO_ROOTFS

cd $GO_ROOTFS

ln -s $GOROOT/bin .
ln -s $GOROOT/api .
ln -s $GOROOT/doc .
ln -s $GOROOT/lib .
ln -s $GOROOT/pkg .
ln -s $GOROOT/misc .
ln -s $GOROOT/test .

cp -r $GOROOT/src $GO_ROOTFS

patch -p2 -d ./src < $PATCH_FILE

