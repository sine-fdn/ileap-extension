#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <Consultation Draft|Release>"
    exit 1
fi

set -euo pipefail

pushd specs

STATUS="$1"
RELEASE_DATE=$(date +%Y%m%d)
PREV_VERSION=$(jq .TR < computed-metadata.include)
YEAR=$(date +%Y)


## create a new computed-metadata.include file
cat <<EOF > computed-metadata.include
{
  "Previous Version": ${PREV_VERSION},
  "TR": "https://sine-fdn.github.io/ileap-extension/TR/2024/ileap-extension-${RELEASE_DATE}/",
  "Text Macro": "STATUS ${STATUS}"
}
EOF

popd

YEAR=$(date +"%Y")

if ! [ -d "TR/$YEAR" ]; then
  mkdir TR/$YEAR
fi

DATE=$(date +"%Y%m%d")

RELEASE_DIR=TR/$YEAR/ileap-extension-$DATE

mkdir -p $RELEASE_DIR
cp -r specs/diagrams $RELEASE_DIR/diagrams
cp specs/index.html $RELEASE_DIR/index.html

make
