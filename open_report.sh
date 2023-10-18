#!/bin/bash

# Attempt to open the coverage report in the default browser, suppressing error messages
xdg-open ./target/debug/coverage/index.html 2>/dev/null || open ./target/debug/coverage/index.html 2>/dev/null

# Provide feedback to the user
echo "Attempting to open the coverage report. If the report doesn't open, you can manually access it at ./target/debug/coverage/index.html"
