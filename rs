#!/usr/bin/env bash

clear
echo "Compiling & Running: $1.rs"
echo "----------------------------"
rustc rust/$1.rs && ./$1 && rm $1
