#!/bin/bash

repos=("left-right-parsing" "personal-website")


if [[ $1 == "--create" ]]
then
  bare="true"
  mkdir deps
else
  bare="false"
fi

export SERVER_GHR_DEPS=""

for rep in "${repos[@]}"; do
  export SERVER_GHR_DEPS="veggiebob/$rep":$SERVER_GHR_DEPS;
  if [[ -d "deps" && -f "$(pwd)/deps/$rep/Cargo.toml" ]]; then
    # prioritize release mode, but debug is ok too
    export PATH="$(pwd)/deps/$rep/target/release":$PATH;
    export PATH="$(pwd)/deps/$rep/target/debug":$PATH;
  fi
  if [[ $bare == "true" &&  -d "deps" ]]; then
    (
      cd "deps"
      git clone "github-personal:veggiebob/$rep.git"
    )
  fi
done