#!/bin/bash

cp scripts/release/header.include specs/header.include
rm specs/index.html

make

rm specs/header.include

YEAR=$(date +"%Y")

if ! [ -d "TR/$YEAR" ]; then
  mkdir TR/$YEAR
fi

DATE=$(date +"%Y%m%d")

RELEASE_DIR=TR/$YEAR/ileap-extension-$DATE

mkdir -p $RELEASE_DIR
cp -r specs/diagrams $RELEASE_DIR/diagrams
cp specs/index.html $RELEASE_DIR/index.html
