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
  echo "Running tests"
  TEST_OUTPUT=$(cargo test 2>&1)
  echo "$TEST_OUTPUT"
  
  PASSED_TESTS=$(echo "$TEST_OUTPUT" | grep "test result:" | awk '{print $4}')
  FAILED_TESTS=$(echo "$TEST_OUTPUT" | grep "test result:" | awk '{print $6}')
  
  echo "Tests Passed: $PASSED_TESTS"
  echo "Tests Failed: $FAILED_TESTS"

  echo "Running test with code coverage"
  TESTS_WITH_COVERAGE=$(RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="calc-%p-%m.profraw" cargo +nightly test)

  echo "Generating Coverage report.."
  COVERAGE_REPORT=$(grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./target/debug/coverage/)
  echo "Coverage report done, check target folder."

  # Ask the user if they want to open the coverage report
  read -p "Do you want to open the coverage report in the default browser? (y/n): " choice
  if [ "$choice" = "y" ] || [ "$choice" = "Y" ]; then
    ./open_report.sh
  fi
}


if [ "$1" == "-tb" ]; then
    run_tests
    test_build
elif [ "$1" == "-t" ]; then
    run_tests
else
    test_build
fi
