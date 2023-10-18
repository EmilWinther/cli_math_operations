#!/bin/bash

export PATH="$HOME/.cargo/bin:$PATH"

test_build() {
  echo "Building and installing..."
  OUTPUT=$(cargo install --path . 2>&1)
  
  if echo "$OUTPUT" | grep -q 'warning\|error'; then
    echo "Build contains warnings or errors. Do you want to continue? [y/n]"
    read -r CONTINUE
    if [ "$CONTINUE" != "y" ] && [ "$CONTINUE" != "yes" ]; then
      echo "Exiting."
      exit 1
    fi
  fi
  
  echo "Testing command..."
  calc -o add 3 3
}

test_build
