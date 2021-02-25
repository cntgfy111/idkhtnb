#!/bin/bash

if [ ! -d ./server_data/public ]; then
    mkdir ./server_data/public
fi

if [ ! -d ./server_data/files]; then
    mkdir ./server_data/files
fi

cur_d=$(pwd)
echo $cur_d

cd ../idkhtnf

if [ ! -d ./node_modules ]; then
    npm i
fi

npm run build
cp -r ./build/* ../idkhtnb/server_data/public/

cd $cur_d
rustup override set nightly
cargo run
