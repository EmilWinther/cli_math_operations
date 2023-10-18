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
  echo "Addition:"
  calc -o add 3 3
  echo "Subtract:"
  calc -o subtract 3 3
  echo "Multiply:"
  calc -o multiply 3 3
  echo "Divide:"
  calc -o divide 3 3
}

run_tests() {
  TEST_OUTPUT=$(cargo test 2>&1)
  echo "$TEST_OUTPUT"
  
  PASSED_TESTS=$(echo "$TEST_OUTPUT" | grep "test result:" | awk '{print $4}')
  FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep "test result:" | awk '{print $6}')
  
  echo "Tests Passed: $PASSED_TESTS"
  echo "Tests Failed: $FAILED_TESTS"
}

if [ "$1" == "-tb" ]; then
    run_tests
    test_build
elif [ "$1" == "-t" ]; then
    run_tests
else
    test_build
fi
