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

# Run the project (which will compile Slint files through build.rs)
run_project