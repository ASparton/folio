#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <target_version>"
    exit 1
fi

target_version=$1

sed -i "s/version = \"[0-9.]\+\"/version = \"$target_version\"/" Cargo.toml
sed -i 's/"version": "[0-9.]\+"/"version": "'"$target_version"'"/' package.json

echo "Version in Cargo.toml and package.json files updated to $target_version"
exit 0
