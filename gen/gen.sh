#!/bin/bash

set -uex

pushd "$(dirname "${BASH_SOURCE[0]}")"

gen=openapi-generator-cli.jar
root=$(pwd)/..
spec=mattermost-openapi-v4.yaml
spec_path=$(pwd)/$spec
ext=$root/ext/ref
name=mmcli_raw

# Generate spec
pushd $ext
spec_src=v4/source
cat $spec_src/introduction.yaml > $spec_path
cat $spec_src/users.yaml >> $spec_path
cat $spec_src/status.yaml >> $spec_path
cat $spec_src/teams.yaml >> $spec_path
cat $spec_src/channels.yaml >> $spec_path
cat $spec_src/posts.yaml >> $spec_path
cat $spec_src/preferences.yaml >> $spec_path
cat $spec_src/jobs.yaml >> $spec_path
cat $spec_src/system.yaml >> $spec_path
cat $spec_src/emoji.yaml >> $spec_path
cat $spec_src/webhooks.yaml >> $spec_path
cat $spec_src/saml.yaml >> $spec_path
cat $spec_src/compliance.yaml >> $spec_path
cat $spec_src/cluster.yaml >> $spec_path
cat $spec_src/brand.yaml >> $spec_path
cat $spec_src/commands.yaml >> $spec_path
cat $spec_src/oauth.yaml >> $spec_path
cat $spec_src/elasticsearch.yaml >> $spec_path
cat $spec_src/bleve.yaml >> $spec_path
cat $spec_src/dataretention.yaml >> $spec_path
cat $spec_src/roles.yaml >> $spec_path
cat $spec_src/schemes.yaml >> $spec_path
cat $spec_src/service_terms.yaml >> $spec_path
cat $spec_src/opengraph.yaml >> $spec_path
cat $spec_src/reactions.yaml >> $spec_path
cat $spec_src/actions.yaml >> $spec_path
cat $spec_src/bots.yaml >> $spec_path
cat $spec_src/definitions.yaml >> $spec_path
popd

# Generate client
rm -rf out
java -jar $gen generate -i $spec -g rust -c config.json -o out

# Update output crate
rm -rf ../$name
mv out ../$name

# Apply patch
pushd $root
patch -p1 < gen/allow-warns.patch
popd
