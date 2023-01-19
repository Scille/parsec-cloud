#!/usr/bin/env bash
: '
Create a temporary environment and initialize a test setup for parsec.

Run `tests/scripts/run_testenv.sh --help` for more information.

For MacOS, please install coreutils from brew to get realpath:
`brew install coreutils`
'

# Make sure this script is sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]
then
    echo "This script must be sourced. Use --help for more information."
    exit 1
fi

# Prevent pre-commit from losing it work dir
if [[ -z "$PRE_COMMIT_HOME" ]]
then
    if [[ -z "$XDG_CACHE_HOME" ]]
    then
        export PRE_COMMIT_HOME="$HOME/.cache/pre-commit"
    else
        export PRE_COMMIT_HOME="$XDG_CACHE_HOME/pre-commit"
    fi
fi

# Cross-shell script directory detection
if [ -n "$BASH_SOURCE" ]; then
    PARSEC_TESTENV_SCRIPT_DIR=$(dirname $(realpath -s $BASH_SOURCE))
elif [ -n "$ZSH_VERSION" ]; then
    PARSEC_TESTENV_SCRIPT_DIR=$(dirname $(realpath -s $0))
fi

# In Python we trust (aka shell's tempfile&mktemp doesn't work on all platforms)
PARSEC_TESTENV_SOURCE_FILE=$(python -c "import tempfile; print(tempfile.mkstemp()[1])")

# Run python script and source
$PARSEC_TESTENV_SCRIPT_DIR/run_testenv.py --source-file $PARSEC_TESTENV_SOURCE_FILE $@ || return $?
source $PARSEC_TESTENV_SOURCE_FILE

# Clean up

function cleanup() {
    echo "Running cleanup"
    $PARSEC_TESTENV_SCRIPT_DIR/run_testenv.py --cleanup

    echo "Removing generated env file"
    rm -v $PARSEC_TESTENV_SOURCE_FILE

    echo "Unsetting configured env variable"
    unset XDG_CACHE_HOME
    unset XDG_DATA_HOME
    unset XDG_CONFIG_HOME
    unset PARSEC_TESTENV_SOURCE_FILE
    unset PARSEC_TESTENV_SCRIPT_DIR

    echo "You can now close your current shell"
}
