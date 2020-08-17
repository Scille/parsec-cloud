#!/usr/bin/env fish
: '
Create a temporary environment and initialize a test setup for parsec.

Run `tests/scripts/run_testenv.fish --help` for more information.
'

# Make sure this script is sourced
set SOURCE_COMMANDS source .
if not contains "$_" $SOURCE_COMMANDS
  echo "This script must be sourced. Use --help for more information."
  exit 1
end

# Cross-shell script directory detection
# set SCRIPT_DIR (dirname (realpath -s $argv[1]))
set SCRIPT_DIR (dirname (realpath (status -f)))

# Run python script and source
set SOURCE_FILE (tempfile)
$SCRIPT_DIR/run_testenv.py --source-file $SOURCE_FILE $argv || exit $status
source $SOURCE_FILE

# Clean up
rm $SOURCE_FILE
