#!/bin/bash

# refresh env
/bin/bash ./flush_env.sh

# start frontend
cd frontend;
bun next build
screen -S blog_frontend -d -m bun start;
cd ..;

# start backend
cd backend;
screen -S blog_backend -d -m cargo r -r;
cd ..;
