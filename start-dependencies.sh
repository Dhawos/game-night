#!/bin/bash

docker-compose up --wait
sleep 10
diesel setup
