#!/bin/bash

if [ $# -lt 2 ]
then
  echo "Needs at least 2 arguments: <website file location> <addr:port> [--init]"
  echo "Website file location can be '.' in order to use dependent repositories in local dir"
else
  if [[ $3 == "--init" ]]; then
    . bare-init.sh --create
  else
    . bare-init.sh
  fi
  echo "starting..."
  bash update-repos.sh
  echo "finished updating."

  # set the default website location using '.'
  website_location="deps/personal-website"
  if [[ ! $1 == "." ]]; then
    website_location="$1"
  fi

  if [[ -f python-config ]]; then
    . python-config
  fi
  
  if [[ -z "${PYTHON_EXEC}" ]]; then
    echo "Please provide a 'python-config' file with the environment variable
      PYTHON_EXEC set to the name of the latest python executable"
    echo "since it's not present, it will be set to 'python'"
    export PYTHON_EXEC="python"
  fi
  # start the actual server, and then continue with other tasks
  # note that SERVER_GHR_DEPS is now defined
  bash run.sh "$website_location" "$2"
fi
