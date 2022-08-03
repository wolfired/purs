#!/usr/bin/env bash

root_path=$(dirname $(realpath $0))
root_name=$(basename $root_path)

source $root_path/prepare.sh

src=$root_path/src
target=$root_path/target

name=$root_name
profraw_file=$target/$name.profraw
profdata_file=$target/$name.profdata

function act_run() {
    rustc -g --out-dir $target --edition 2021 -C instrument-coverage -L $target -l purc $src/lib.rs

    rustc -g -o $target/$name --edition 2021 -C instrument-coverage -L $target --extern purs $src/main.rs

    LLVM_PROFILE_FILE=$profraw_file $target/$name
    $sysroot/lib/rustlib/$target_triple/bin/llvm-profdata merge -sparse $profraw_file -o $profdata_file
    $sysroot/lib/rustlib/$target_triple/bin/llvm-cov show -format=html -output-dir=$target/cov -Xdemangler=rustfilt $target/$name -instr-profile=$profdata_file -show-line-counts-or-regions -show-instantiations
    $sysroot/lib/rustlib/$target_triple/bin/llvm-cov export -format=lcov $target/$name -instr-profile=$profdata_file > $target/cov/codecov.info
}

function act_upload_codecov() {
    if [[ ! -x `type codecov | grep -oP '[^\s]+$'` ]]; then
        echo 'you need get codecov'
        return 0
    fi

    if [[ ! -n $CODECOV_TOKEN ]]; then
        echo 'you need setup CODECOV_TOKEN env var'
        return 0
    fi

    codecov -t $CODECOV_TOKEN
}

function main() {
    local actions=(
        act_run
        act_upload_codecov
        'exit 0'
    )
    local labels=(
        'run'
        'codecov'
        'quit'
    )

    PS3="select(1-${#labels[@]}): "
    select label in ${labels[@]}; do
        local index=$(($REPLY-1))
        if (( 0 <= $index && $index < ${#labels[@]} )); then
            ${actions[$index]}
        else
            echo "Illegal selection: $REPLY" 1>&2
            exit 1
        fi
        break
    done
}
main