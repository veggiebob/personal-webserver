#!/bin/bash
if [ $# -lt 2 ]
then
  echo "Needs at least 2 arguments: <website file location> <addr:port> [--init]"
else
  if [[ $3 == "--init" ]]; then
    . bare-init.sh --create
  else
    . bare-init.sh
  fi

  # start the actual server, and then continue with other tasks
  # note that SERVER_GHR_DEPS is now defined
  echo "Path is $PATH"
  bash run.sh "$1" "$2" &
  bash update-repos.sh
fi
