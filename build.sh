#!/bin/sh
PKG_DIR=$(pwd)
VERSION=v$1
RELEASE_DIR=$PKG_DIR/release/$VERSION

cargo build --release --target $RUST_BUILD_TARGET && \
mkdir -p $RELEASE_DIR && \
cd $PKG_DIR/target/$RUST_BUILD_TARGET/release && \
tar -zcvf silqfmt-$VERSION-$RUST_BUILD_TARGET.tar.gz silqfmt && \
mv silqfmt-$VERSION-$RUST_BUILD_TARGET.tar.gz $RELEASE_DIR