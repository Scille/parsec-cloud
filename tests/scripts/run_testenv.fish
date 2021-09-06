#!/usr/bin/env fish

# Make sure this script is sourced
set SOURCE_COMMANDS source .
if not contains "$_" $SOURCE_COMMANDS
  echo "This script must be sourced. Use --help for more information."
  exit 1
end

# Cross-shell script directory detection
# set SCRIPT_DIR (dirname (realpath -s $argv[1]))
set SCRIPT_DIR (dirname (realpath (status -f)))

# In Python we trust (aka shell's tempfile&mktemp doesn't work on all platforms)
set SOURCE_FILE (python -c "import tempfile; print(tempfile.mkstemp()[1])")

# Run python script and source
eval $SCRIPT_DIR/run_testenv.py --source-file $SOURCE_FILE $argv; or exit $status
source $SOURCE_FILE

# Clean up
rm $SOURCE_FILE
