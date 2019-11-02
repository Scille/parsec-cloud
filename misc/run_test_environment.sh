#!/usr/bin/env bash
: '
Create a temporary environment and initialize a test setup for parsec.

WARNING: it also leaves an in-memory backend running in the background
on port 6888.

It is typically a good idea to source this script in order to export the XDG
variables so the upcoming parsec commands point to the test environment:

    $ source misc/run_test_environment.sh

This scripts create two users, alice and bob who both own two devices,
laptop and pc. They each have their workspace, respectively
alice_workspace and bob_workspace, that their sharing with each other.

The --empty (or -e) argument may be used to bypass the initialization of the
test environment:

    $ source misc/run_test_environment.sh --empty

This can be used to perform a user or device enrollment on the same machine.
For instance, consider the following scenario:

    $ source misc/run_test_environment.sh
    $ parsec core gui
    # Connect as bob@laptop and register a new device called pc
    # Copy the URL

Then, in a second terminal:

    $ source misc/run_test_environment.sh --empty
    $ xdg-open "<paste the URL here>"  # Or
    $ firefox --no-remote "<paste the URL here>"
    # A second instance of parsec pops-up
    # Enter the token to complete the registration
'

# Parse arguments

while [[ "$#" -gt 0 ]]; do case $1 in
  -e|--empty) empty=true;;
  *) echo "Unknown parameter passed: $1"; exit 1;;
esac; shift; done

# Set the temporary environment up

PORT=6888
TMP=`mktemp -d`
export XDG_CACHE_HOME="$TMP/cache"
export XDG_DATA_HOME="$TMP/share"
export XDG_CONFIG_HOME="$TMP/config"
mkdir $XDG_CACHE_HOME $XDG_DATA_HOME $XDG_CONFIG_HOME
mkdir $XDG_DATA_HOME/applications
echo """\
Configure your test environment with the following variables:

    export XDG_CACHE_HOME=$XDG_CACHE_HOME
    export XDG_DATA_HOME=$XDG_DATA_HOME
    export XDG_CONFIG_HOME=$XDG_CONFIG_HOME
"""

# Configure MIME types locally

echo """\
[Desktop Entry]
Name=Parsec
Exec=parsec core gui %u
Type=Application
Terminal=false
StartupNotify=false
StartupWMClass=Parsec
MimeType=x-scheme-handler/parsec;
""" > $XDG_DATA_HOME/applications/parsec.desktop
update-desktop-database 2> /dev/null
xdg-mime default parsec.desktop x-scheme-handler/parsec

# Keep the environment empty

if [[ "$empty" = true ]]; then return; fi

# Kill the previous backend server

pkill -f "parsec backend run -b MOCKED --db MOCKED -P $PORT --administration-token *"

# Fire up the new backend server

# ADMIN_TOKEN=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1)
ADMIN_TOKEN=V8VjaXrOz6gUC6ZEHPab0DSsjfq6DmcJ
parsec backend run -b MOCKED --db MOCKED -P $PORT --administration-token $ADMIN_TOKEN &
sleep 1

# Initialize the test organization

python3 misc/initialize_test_organization.py -B "parsec://localhost:$PORT?no_ssl=true" -T $ADMIN_TOKEN $@
