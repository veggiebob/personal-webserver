#!/bin/bash

# nitty gritty stuff that actually runs the executable

if [ $# -ne 2 ]
then
  echo "Needs at least 2 arguments: <website file location> <addr:port>"
else
  cargo build --release 2> .cargo-output
  authbind --deep ./target/release/personal-webserver "$1" "$2"
fi