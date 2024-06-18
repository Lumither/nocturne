#!/bin/bash

# refresh env
/bin/bash ./flush_env.sh

# start frontend
cd frontend || exit
bun next build
screen -S nocturne_frontend -d -m bun start
cd ..

# start backend
cd backend || exit
screen -S nocturne_backend -d -m cargo r -r
cd ..
