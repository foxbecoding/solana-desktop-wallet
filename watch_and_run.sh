#!/bin/bash

# Function to run the project
run_project() {
  cargo run
  exit_code=$?
  if [ $exit_code -ne 0 ]; then
    echo "Cargo run failed with exit code $exit_code"
    exit $exit_code
  fi
}

if [ "$1" == "run_project" ]; then
  run_project
else
  # Use cargo watch to watch the app and src directories and call the run_project function on changes
  # The \$ is used to escape $ so it's not interpreted within the single quotes
  cargo watch -w app -w src -s 'bash -c "./watch_and_run.sh run_project"'
fi