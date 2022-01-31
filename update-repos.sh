#!/bin/bash
for repo in deps/*; do
  if [[ -d $repo ]]; then
    (
      cd "$repo"
      git pull # if there are changes and this fails then you're a bozo
      if [[ -f "Cargo.toml" ]]; then
        cargo build --quiet --release 2> .cargo-warnings
      fi
    )
  fi
done