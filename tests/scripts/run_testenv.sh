#!/usr/bin/env bash
: '
Create a temporary environment and initialize a test setup for parsec.

Run `tests/scripts/run_testenv.sh --help` for more information.
'

# Make sure this script is sourced
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]
then
echo "This script must be sourced. Use --help for more information."
exit 1
fi

# Cross-shell script directory detection
if [ -n "$BASH_SOURCE" ]; then
  script_dir=$(dirname $(realpath -s $BASH_SOURCE))
elif [ -n "$ZSH_VERSION" ]; then
  script_dir=$(dirname $(realpath -s $0))
fi

# Run python script and source
source_file=$(tempfile)
$script_dir/run_testenv.py --source-file $source_file $@ || return $?
source $source_file

# Clean up
rm $source_file
unset source_file
unset script_dir
