#!/usr/bin/env fish

# Make sure this script is sourced
set SOURCE_COMMANDS source .
if not contains "$_" $SOURCE_COMMANDS
    echo "This script must be sourced. Use --help for more information."
    exit 1
end

# Prevent pre-commit from losing it work dir
if test -z "$PRE_COMMIT_HOME"
    if test -z "$XDG_CACHE_HOME"
        set --export PRE_COMMIT_HOME "$HOME/.cache/pre-commit"
    else
        set --export PRE_COMMIT_HOME "$XDG_CACHE_HOME/pre-commit"
    end
end

# Cross-shell script directory detection
# set SCRIPT_DIR (dirname (realpath -s $argv[1]))
set SCRIPT_DIR (dirname (realpath (status -f)))

# In Python we trust (aka shell's tempfile&mktemp doesn't work on all platforms)
set SOURCE_FILE (python -c "import tempfile; print(tempfile.mkstemp()[1])")

echo ">>> " "cargo run --package parsec_cli --features testenv run-testenv --main-process-id $fish_pid --source-file \"$SOURCE_FILE\" $argv"
eval cargo run --package parsec_cli --features testenv run-testenv --main-process-id $fish_pid --source-file "$SOURCE_FILE" $argv; or exit $status

# `$SOURCE_FILE` uses bash syntax to configure environ variables (i.e. `export FOO=BAR`)
# here we convert is to fish syntax (i.e. `set --export FOO BAR`)
sed --in-place --regexp-extended 's/^export (\w+)=/set --export \1 /' $SOURCE_FILE

source $SOURCE_FILE

# Clean up
rm $SOURCE_FILE
