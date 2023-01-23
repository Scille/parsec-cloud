#!/bin/bash

CURRENT_DIR=$(basename `pwd`)

if [ "$CURRENT_DIR" != "tests" ]; then
    echo "Launch from folder tests/" >&2
    exit 1
fi

# Kill any running backend. Hopefully no command has localhost:6886 as its argument.
pkill -f "localhost:6886"

# Launch Parsec backend
python -Wignore -m parsec.cli backend run --log-level=WARNING -b MOCKED --db MOCKED \
    --email-host=MOCKED -P 6886 --spontaneous-organization-bootstrap --administration-token aaa \
    --backend-addr "parsec://localhost:6886?no_ssl=true" > /dev/null 2>&1 &

# Wait for the backend to start
sleep 3

if [ "$?" != "0" ]; then
    echo "Failed to start parsec backend" >&2
    exit 1
fi

# Generate files
OUTPUT_FILE="common/generated.ts"

python tools/generate_testing_device.py --backend-addr "parsec://localhost:6886?no_ssl=true" -o $OUTPUT_FILE
