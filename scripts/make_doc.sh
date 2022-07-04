#!/bin/bash

# unify entry path for this script
current_path=`dirname $0`
raptors_root=`realpath $current_path"/.."`

cd $raptors_root

# build doc
cargo doc --no-deps

# update to github pages source directory
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=raptors\">" > target/doc/raptors/index.html
cp -r target/doc ./docs
