#!/bin/bash

# refresh env
/bin/bash ./flush_env.sh

# start frontend
cd frontend;
screen -S blog_frontend -d -m bun dev;
cd ..;

# start backend
cd backend;
screen -S blog_backend -d -m cargo r;
cd ..;
