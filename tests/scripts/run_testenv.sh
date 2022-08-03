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
  SCRIPT_DIR=$(dirname $(realpath -s $BASH_SOURCE))
elif [ -n "$ZSH_VERSION" ]; then
  SCRIPT_DIR=$(dirname $(realpath -s $0))
fi

# In Python we trust (aka shell's tempfile&mktemp doesn't work on all platforms)
SOURCE_FILE=$(python -c "import tempfile; print(tempfile.mkstemp()[1])")

# Run python script and source
poetry run $SCRIPT_DIR/run_testenv.py --source-file $SOURCE_FILE $@ || return $?
source $SOURCE_FILE

# Clean up
rm $SOURCE_FILE
unset SOURCE_FILE
unset SCRIPT_DIR
