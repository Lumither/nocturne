#!/bin/bash

source .env

if [ -z "$BLOG_ENDPOINT" ]; then
  echo "BLOG_ENDPOINT is not set in the .env file"
  exit 1
fi

if [ -z "$GIT_WORK_DIR" ]; then
  echo "GIT_WORK_DIR is not set in the .env file"
  exit 1
fi

if [ -z "$BACKEND_PORT" ]; then
  echo "BACKEND_PORT is not set in the .env file"
  exit 1
fi

check_for_updates() {
  cd "$GIT_WORK_DIR" || exit
  res=$(git pull)
  if echo "$res" | grep -q "Already up to date"; then
    echo "Repo is up to date."
  else
    echo "Repo has updated; calling db update"
    curl --request POST -sL \
         --url "localhost:$BACKEND_PORT"
    #todo: to be continued
  fi
  }

if [ -d "$GIT_WORK_DIR" ]; then
  check_for_updates
else
  git clone "$BLOG_ENDPOINT" "$GIT_WORK_DIR"
  if [ $? -eq 0 ]; then
    check_for_updates
  else
    echo "Failed to clone the repo"
  fi
fi
