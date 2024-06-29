#!/bin/bash

repos=("left-right-parsing" "static-personal-website" "gym-data-recorder" "secret-santa" "bf2spl")


if [[ $1 == "--create" ]]
then
  bare="true"
  mkdir deps
else
  bare="false"
fi

if [[ -z "${GITHUB_USER}" ]]; then
    echo "GITHUB_USER being set to 'git@github.com'"
    export GITHUB_USER="git@github.com"
fi

export SERVER_GHR_DEPS=""

for rep in "${repos[@]}"; do
  export SERVER_GHR_DEPS="veggiebob/$rep":$SERVER_GHR_DEPS;
  
  # just add it to the path no matter what
  export PATH="$(pwd)/deps/$rep/":$PATH;
  if [[ -d "deps" && -f "$(pwd)/deps/$rep/Cargo.toml" ]]; then
    # prioritize release mode, but debug is ok too
    export PATH="$(pwd)/deps/$rep/target/release":$PATH;
    export PATH="$(pwd)/deps/$rep/target/debug":$PATH;
  fi
  if [[ -d "deps" ]]; then
    if compgen -G "*.py" > /dev/null; then
      # so that directories with python can have scripts in them
      export PATH="$(pwd)/deps/$rep/":$PATH;
    fi
  fi
  if [[ $bare == "true" &&  -d "deps" ]]; then
    (
      cd "deps"
      git clone "$GITHUB_USER:veggiebob/$rep.git"
    )
  fi
done