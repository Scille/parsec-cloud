#!/usr/bin/env bash
: '
Create a temporary environment and initialize a test setup for parsec.

Run `tests/scripts/run_test_environment.sh --help` for more information.
'

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]
then
echo "This script must be sourced. Use --help for more information."
exit 1
fi

source_file=$(tempfile)
$(dirname $(realpath -s $0))/run_test_environment.py --source-file $source_file $@
source $source_file
rm $source_file
unset source_file
