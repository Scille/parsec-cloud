#!/usr/bin/env bash

# Create a temporary environment and initialize a test setup for Parsec.
#
# For MacOS, please install coreutils from brew to get realpath:
#`brew install coreutils`

# Make sure this script is sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]
then
    echo "This script must be sourced:"
    echo ". ./run_testenv.sh"
    exit 1
fi

# Prevent pre-commit from losing its work dir
if [[ -z "$PRE_COMMIT_HOME" ]]
then
    if [[ -z "$XDG_CACHE_HOME" ]]
    then
        export PRE_COMMIT_HOME="$HOME/.cache/pre-commit"
    else
        export PRE_COMMIT_HOME="$XDG_CACHE_HOME/pre-commit"
    fi
fi

# In Python we trust (aka shell's tempfile&mktemp doesn't work on all platforms)
SOURCE_FILE=$(python -c "import tempfile; print(tempfile.mkstemp()[1])")

echo ">>> cargo run --package parsec_cli --features testenv run-testenv --main-process-id $$ --source-file \"$SOURCE_FILE\" $*"
cargo run --package parsec_cli --features testenv run-testenv --main-process-id $$ --source-file "$SOURCE_FILE" "$@" || return $?
source "$SOURCE_FILE"

# Clean up
rm "$SOURCE_FILE"
unset SOURCE_FILE
unset SCRIPT_DIR
