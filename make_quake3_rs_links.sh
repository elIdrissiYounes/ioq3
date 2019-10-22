#!/bin/bash

set -e

if [ ! -d "$1" -o ! -d "$2" ]; then
    echo -e "Usage: $0 <path/to/quake3-rs/target/release> <path/to/paks>"
    exit 1
fi

TARGET_DIR=`readlink -f $1`
PAKS_DIR=`readlink -f $2`

ln -s $TARGET_DIR/ioquake3
ln -s $TARGET_DIR/librenderer_opengl1_x86_64.so renderer_opengl1_x86_64.so

mkdir -p baseq3
cd baseq3
ln -s $TARGET_DIR/libcgamex86_64.so cgamex86_64.so
ln -s $TARGET_DIR/libqagamex86_64.so qagamex86_64.so
ln -s $TARGET_DIR/libuix86_64.so uix86_64.so
for pak in $PAKS_DIR/*.pk3; do
    ln -s $pak
done
