#!/bin/bash
for repo in deps/*; do
  if [[ -d $repo ]]; then
    (
      cd "$repo"
      git pull
      if [[ -f "Cargo.toml" ]]; then
        cargo build --release &> .cargo-build-output
      fi
    )
  fi
done