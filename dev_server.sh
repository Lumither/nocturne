#!/bin/bash

# refresh env
/bin/bash ./flush_env.sh

# start frontend
cd frontend || exit
screen -S nocturne_frontend -d -m bun dev
cd ..

# start backend
cd backend || exit
screen -S nocturne_backend -d -m cargo r
cd ..
