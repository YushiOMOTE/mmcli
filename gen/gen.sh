#!/bin/sh

set -uex

spec=mattermost-openapi-v4.yaml
ext=../ext/ref

pushd "$(dirname "${BASH_SOURCE[0]}")"

# Generate spec
pushd $ext
make build
popd

# Generate client
cp $ext/v4/html/static/$spec .
rm -rf out
docker run --rm -v $(pwd):/local openapitools/openapi-generator-cli generate -i /local/$spec -g rust -c /local/config.json -o /local/out

# Check if build passes
pushd out
cargo build
popd

# Update mmcli_raw
rm -rf ../mmcli_raw
mv out ../mmcli_raw
