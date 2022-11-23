#!/usr/bin/env bash

root_path=$(dirname $(realpath $0))
root_name=$(basename $root_path)

source $root_path/prepare.sh

dir_src=$root_path/src
dir_out=$root_path/target

function color_msg() {
    local color=${1:?'(r)ed or (g)reen (b)lue (y)ellow (p)urple (c)yan'}

    if (( 2 > $# )); then
        return
    fi

    if [[ 'r' == $color ]]; then
        echo -e '\033[31m'${@:2}'\033[0m' # red
    elif [[ 'g' == $color ]]; then
        echo -e '\033[32m'${@:2}'\033[0m' # green
    elif [[ 'b' == $color ]]; then
        echo -e '\033[34m'${@:2}'\033[0m' # blue
    elif [[ 'y' == $color ]]; then
        echo -e '\033[33m'${@:2}'\033[0m' # yellow
    elif [[ 'p' == $color ]]; then
        echo -e '\033[35m'${@:2}'\033[0m' # purple
    elif [[ 'c' == $color ]]; then
        echo -e '\033[36m'${@:2}'\033[0m' # cyan
    else
        echo -e '\033[37m'${@:2}'\033[0m' # white
    fi
}

function exec_rustc() {
    local input_args=${1:?''}
    local rustc_args=(${@:2})

    if (( 1 == $input_args )); then
        color_msg y "preparing: rustc ${rustc_args[*]}"

        read -p "input rustc ext args: " ext_rustc_args

        for ext_arg in ${ext_rustc_args[@]}; do
            rustc_args+=($ext_arg)
        done
    fi

    color_msg g "executing: rustc ${rustc_args[*]}"

    rustc ${rustc_args[@]}

    return $?
}

function get_id_name() {
    local source_file=${1:?''}

    local source_file_name=`basename $source_file`

    local id_name=${source_file_name%.*}

    if [[ 'main' == $id_name || 'lib' == $id_name || 'bin' == $id_name ]]; then
        local dir=`dirname $source_file`
        while true; do
            id_name=`basename $dir`
            if [[ 'src' != $id_name ]]; then
                break
            fi
            dir=`dirname $dir`
        done
    fi

    echo $id_name
}

function get_group_name() {
    local source_file=${1:?''}

    local group_name=`echo $source_file | grep -oP "(?<=$root_path/)[^\s]+?(?=/)"`

    if [[ 'src' == $group_name || 'bin' == $group_name ]]; then
        echo ''
    else
        echo $group_name
    fi
}

function build_bin() {
    local file_in=${1:?''}
    local file_out=${2:?''}
    local dir_lib=${3:?''}

    exec_rustc 0 -g -o $file_out --edition 2021 -C instrument-coverage -L $dir_lib --extern purs $file_in

    if (( 0 != $? )); then
        echo 'build bin error'
        exit 0
    fi
}

function build_lib() {
    local file_in=${1:?''}
    local dir_out=${2:?''}

    exec_rustc 0 -g --out-dir $dir_out --edition 2021 -C instrument-coverage -L $dir_out -l purc $file_in

    if (( 0 != $? )); then
        echo 'build lib error'
        exit 0
    fi
}

function select_target_file() {
    local root_path_to_build=${1:?''}

    local main_path=$root_path_to_build/src
    local ext_paths=(
        $root_path_to_build/src/bin
        $root_path_to_build/examples
        $root_path_to_build/tests
        $root_path_to_build/benches
    )

    local builds=()

    for build in `find $main_path -maxdepth 1 -type f | grep -P 'main.rs$'`; do
        builds+=($build)
    done

    for ext_path in ${ext_paths[@]}; do
        if [[ -d $ext_path ]]; then
            for build in `find $ext_path -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
                builds+=($build)
            done
            for build in `find $ext_path -type f | grep -P 'main.rs$'`; do
                builds+=($build)
            done
        fi
    done

    PS3="select(1-${#builds[@]}): "
    select label in ${builds[@]}; do
        local index=$(($REPLY-1))
        if (( 0 <= $index && $index < ${#builds[@]} )); then
            echo ${builds[$index]}
        else
            echo "Illegal Selection: $REPLY" 1>&2
            exit 1
        fi
        break
    done
}

function select_target_group() {
    local root_path_to_build=${1:?''}

    local groups=(
        $root_path_to_build/src
        $root_path_to_build/src/bin
        $root_path_to_build/examples
        $root_path_to_build/tests
        $root_path_to_build/benches
    )
    local group_names=(
        $root_name
        'bins'
        'examples'
        'tests'
        'benches'
    )

    PS3="select(1-${#group_names[@]}): "
    select label in ${group_names[@]}; do
        local index=$(($REPLY-1))
        if (( 0 <= $index && $index < ${#group_names[@]} )); then
            echo ${groups[$index]}
        else
            echo "Illegal Selection: $REPLY" 1>&2
            exit 1
        fi
        break
    done
}

function run_single() {
    local target_file=${1:?''}

    color_msg g "running $target_file"

    local id_name=`get_id_name $target_file`
    local group_name=`get_group_name $target_file`
    
    build_lib $dir_src/lib.rs $dir_out

    local bin_file=$dir_out/$id_name
    local profraw_file=$dir_out/$id_name.profraw
    if [[ -n $group_name ]]; then
        mkdir -p $dir_out${group_name:+/$group_name}
        bin_file=$dir_out${group_name:+/$group_name}/$id_name
        profraw_file=$dir_out${group_name:+/$group_name}/$id_name.profraw
    fi

    build_bin $target_file $bin_file $dir_out

    LLVM_PROFILE_FILE=$profraw_file $bin_file
}

function run_group() {
    local target_group=${1:?''}

    local builds=()

    local key=`basename $target_group`

    if [[ 'src' == $key ]]; then
        for build in `find $target_group -maxdepth 1 -type f | grep -P 'main.rs$'`; do
            builds+=($build)
        done
    else
        if [[ -d $target_group ]]; then
            for build in `find $target_group -maxdepth 1 -type f | grep -P '[^\s]+?\.rs$'`; do
                builds+=($build)
            done
            for build in `find $target_group -type f | grep -P 'main.rs$'`; do
                builds+=($build)
            done
        fi
    fi

    for build in ${builds[@]}; do
        run_single $build
    done
}

function act_run_single() {
    local target_file=`select_target_file $root_path`

    if [[ -n $target_file ]]; then
        run_single $target_file
    fi
}

function act_run_group() {
    local target_group=`select_target_group $root_path`

    if [[ -n $target_group ]]; then
        run_group $target_group
    fi
}

function act_codecov() {
    local groups=(
        $root_path/src
        $root_path/src/bin
        $root_path/examples
        $root_path/tests
        $root_path/benches
    )

    for group in ${groups[@]}; do
        run_group $group
    done

    local profraws=()
    for profraw in `find $dir_out -type f | grep -P '[^\s]+?\.profraw$'`; do
        profraws+=($profraw)
    done
    local profraws_args=$(printf " %s" "${profraws[@]}")
    profraws_args=${profraws_args:1}

    local bins=()
    for bin in `find $dir_out -type f | grep -oP '[^\s]+?(?=\.profraw)'`; do
        bins+=($bin)
    done
    local bins_args=$(printf " -object %s" "${bins[@]}")
    bins_args=${bins_args:1}

    local profdata_file=$dir_out/$root_name.profdata

    $sysroot/lib/rustlib/$target_triple/bin/llvm-profdata merge \
    -sparse \
    $profraws_args \
    -o $profdata_file

    $sysroot/lib/rustlib/$target_triple/bin/llvm-cov show \
    -format=html \
    -output-dir=$dir_out/cov \
    -Xdemangler=rustfilt \
    $bins_args \
    -instr-profile=$profdata_file \
    -show-line-counts-or-regions \
    -show-instantiations

    $sysroot/lib/rustlib/$target_triple/bin/llvm-cov export \
    -format=lcov \
    $bins_args \
    -instr-profile=$profdata_file > $dir_out/cov/codecov.info
}

function act_upload_codecov() {
    local count=`git status --porcelain | grep -coP '^.+$'`
	local hashl=`git rev-parse @`
	local hashr=`git rev-parse @{u}`

    if (( 0 < $count )) || [[ $hashl != $hashr ]]; then
        echo 'you need commit and push at first'
        return 1
    fi

    if [[ ! -x `type codecov | grep -oP '[^\s]+$'` ]]; then
        echo 'you need get codecov'
        return 1
    fi

    if [[ ! -n $CODECOV_TOKEN ]]; then
        echo 'you need setup CODECOV_TOKEN env var'
        return 1
    fi

    codecov -t $CODECOV_TOKEN
}

function act_doc() {
    rustdoc --html-in-header ./html/katex_header.html ./src/lib.rs
}

function main() {
    local actions=(
        act_run_single
        act_run_group
        act_codecov
        act_upload_codecov
        act_doc
        'exit 0'
    )
    local labels=(
        'run_single'
        'run_group'
        'codecov'
        'upload_codecov'
        'doc'
        'quit'
    )

    PS3="select(1-${#labels[@]}): "
    select label in ${labels[@]}; do
        local index=$(($REPLY-1))
        if (( 0 <= $index && $index < ${#labels[@]} )); then
            ${actions[$index]}
        else
            echo "Illegal Selection: $REPLY" 1>&2
            exit 1
        fi
        break
    done
}
main
