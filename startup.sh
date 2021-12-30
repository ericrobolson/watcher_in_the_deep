#!/bin/bash

# This script will clone the tool, then launch it. 
# It is suggested you add a .gitignore entry for `.witd`.
# You may also want to modify the PROGRAM env var.

CUR_DIR=${PWD}
DEV_ENV=.witd
PROGRAM="in . do echo NAME end"

# Clone dev-env if it doesn't exist
[ ! -d ${DEV_ENV} ] && git clone https://github.com/ericrobolson/watcher_in_the_deep ${DEV_ENV}

# Update dev-env
cd ${DEV_ENV}
git checkout -- . 
git pull

# Run the program
cargo run --release $PROGRAM
