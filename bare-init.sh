#!/bin/bash

repos=("left-right-parsing" "personal-website" "gym-data-recorder")


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
  
  # just add it to the path no matter what
  echo "adding $(pwd)/deps/$rep/ to PATH!"
  export PATH="$(pwd)/deps/$rep/":$PATH;
  if [[ -d "deps" && -f "$(pwd)/deps/$rep/Cargo.toml" ]]; then
    # prioritize release mode, but debug is ok too
    export PATH="$(pwd)/deps/$rep/target/release":$PATH;
    export PATH="$(pwd)/deps/$rep/target/debug":$PATH;
  fi
  if [[ -d "deps" ]]; then
    if compgen -G "*.py" > /dev/null; then
      # so that directories with python can have scripts in them
      echo "adding python repo directory to PATH! $(pwd)"
      export PATH="$(pwd)/deps/$rep/":$PATH;
    fi
  fi
  if [[ $bare == "true" &&  -d "deps" ]]; then
    (
      cd "deps"
      git clone "github-personal:veggiebob/$rep.git"
    )
  fi
done