source ./prepare.sh

rustc -g --out-dir ./out --edition 2021 -C instrument-coverage -L ./out -l purc ./src/lib.rs

if [[ -d ./src/bin ]]; then
    find ./src/bin -maxdepth 1 -mindepth 1 -type d | while read line; do
        name=`basename $line`
        rustc -g -o ./out/$name --edition 2021 -C instrument-coverage -L ./out --extern purs $line/main.rs
        LLVM_PROFILE_FILE=./out/$name.profraw ./out/$name
        $sysroot/lib/rustlib/$target_triple/bin/llvm-profdata merge -sparse ./out/$name.profraw -o ./out/$name.profdata
        $sysroot/lib/rustlib/$target_triple/bin/llvm-cov show -format=html -output-dir=./out/cov_$name -Xdemangler=rustfilt ./out/$name -instr-profile=./out/$name.profdata -show-line-counts-or-regions -show-instantiations
    done

    find ./src/bin -maxdepth 1 -mindepth 1 -type f | while read line; do
        name=`basename ${line%.*}`
        rustc -g -o ./out/$name --edition 2021 -C instrument-coverage -L ./out --extern purs $line
        LLVM_PROFILE_FILE=./out/$name.profraw ./out/$name
        $sysroot/lib/rustlib/$target_triple/bin/llvm-profdata merge -sparse ./out/$name.profraw -o ./out/$name.profdata
        $sysroot/lib/rustlib/$target_triple/bin/llvm-cov show -format=html -output-dir=./out/cov_$name -Xdemangler=rustfilt ./out/$name -instr-profile=./out/$name.profdata -show-line-counts-or-regions -show-instantiations
    done
fi

name=purs
rustc -g -o ./out/$name --edition 2021 -C instrument-coverage -L ./out --extern purs ./src/main.rs
LLVM_PROFILE_FILE=./out/$name.profraw ./out/$name
$sysroot/lib/rustlib/$target_triple/bin/llvm-profdata merge -sparse ./out/$name.profraw -o ./out/$name.profdata
$sysroot/lib/rustlib/$target_triple/bin/llvm-cov show -format=html -output-dir=./out/cov_$name -Xdemangler=rustfilt ./out/$name -instr-profile=./out/$name.profdata -show-line-counts-or-regions -show-instantiations
