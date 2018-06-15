#!/usr/bin/env bash

clear
rustc rust/$1.rs && ./$1 && rm $1
