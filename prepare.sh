#!/usr/bin/env bash

root_path=${root_path:-$(dirname $(realpath $0))}

commit_hash=`rustc -vV | grep -oP '[0-9a-z]{40}'`
sysroot=`rustc --print sysroot`
target_triple=`basename $sysroot | grep -oP '(?<=-).*'`

sed -i "s@\(\"\/rustc\/\)[0-9a-z]\{40\}\(\": \"\).*\(\/lib\/rustlib\/src\/rust\"\)@\1$commit_hash\2${sysroot//\\//}\3@g" $root_path/.vscode/launch.json
sed -i "s@\(\"sysroot_src\": \"\).*\(\/lib\/rustlib\/src\/rust\/library\"\)@\1${sysroot//\\//}\2@g" $root_path/.vscode/rust-project.json
sed -i "s@\(\"command\": \"\).*\(\/lib\/rustlib\/\).*\(\/bin\)@\1${sysroot//\\//}\2$target_triple\3@g" $root_path/.vscode/tasks.json
