#!/bin/sh
PKG_DIR=$(pwd)
VERSION=v$1
TARGET=x86_64-apple-darwin
RELEASE_DIR=$PKG_DIR/release/$VERSION

cargo build --release --target $TARGET && \
mkdir -p $RELEASE_DIR && \
cd $PKG_DIR/target/$TARGET/release && \
tar -zcvf silqfmt-$VERSION-$TARGET.tar.gz silqfmt && \
mv silqfmt-$VERSION-$TARGET.tar.gz $RELEASE_DIR

TARGET=x86_64-pc-windows-gnu
cross build --target $TARGET --release && \
cd $PKG_DIR/target/$TARGET/release && \
zip silqfmt-$VERSION-$TARGET silqfmt.exe && \
mv silqfmt-$VERSION-$TARGET.zip $RELEASE_DIR

TARGET=x86_64-unknown-linux-gnu
cross build --target $TARGET --release && \
cd $PKG_DIR/target/$TARGET/release && \
tar -zcvf silqfmt-$VERSION-$TARGET.tar.gz silqfmt && \
mv silqfmt-$VERSION-$TARGET.tar.gz $RELEASE_DIR