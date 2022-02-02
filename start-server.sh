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
  echo "starting..."
  bash update-repos.sh
  echo "finished updating."
  echo "path is $PATH"

  # set the default website location using '.'
  website_location="deps/personal-website"
  if [[ ! $1 == "." ]]; then
    website_location="$1"
  fi
#  echo "website location is $website_location"

  # start the actual server, and then continue with other tasks
  # note that SERVER_GHR_DEPS is now defined
  bash run.sh "$website_location" "$2" &
fi
