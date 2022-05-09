#!/usr/bin/env bash

commit_hash=`rustc -vV | grep -oP '[0-9a-z]{40}'`
sysroot=`rustc --print sysroot`

sed -i "s@\(\"\/rustc\/\)[0-9a-z]\{40\}\(\": \"\).*\(\/lib\/rustlib\/src\/rust\"\)@\1$commit_hash\2${sysroot//\\//}\3@g" ./.vscode/launch.json
sed -i "s@\(\"sysroot_src\": \"\).*\(\/lib\/rustlib\/src\/rust\/library\"\)@\1${sysroot//\\//}\2@g" ./.vscode/settings.json
