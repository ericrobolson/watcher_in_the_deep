#!/bin/bash

# This script will clone the tool, then launch it. 
# It is suggested you add a .gitignore entry for `.dev_env`.
# You may also want to modify the PROGRAM env var.

CUR_DIR=${PWD}
DEV_ENV=.dev_env
PROGRAM= "in . do echo NAME end"

# Clone dev-env if it doesn't exist
[ ! -d ${DEV_ENV} ] && git clone https://github.com/ericrobolson/dev-env.git ${DEV_ENV}

# Update dev-env
cd ${DEV_ENV}
git checkout -- . 
git pull

# Run the program
cargo run --release ${PROGRAM}
