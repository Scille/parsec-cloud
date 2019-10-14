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
'

PORT=6888
TMP=`mktemp -d`
export XDG_CACHE_HOME="$TMP/cache"
export XDG_DATA_HOME="$TMP/share"
export XDG_CONFIG_HOME="$TMP/config"
mkdir $XDG_CACHE_HOME $XDG_DATA_HOME $XDG_CONFIG_HOME

echo """\
Configure your test environment with the following variables:

    export XDG_CACHE_HOME=$XDG_CACHE_HOME
    export XDG_DATA_HOME=$XDG_DATA_HOME
    export XDG_CONFIG_HOME=$XDG_CONFIG_HOME
"""

ADMIN_TOKEN=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1)
pkill -f "parsec backend run -b MOCKED --db MOCKED -P $PORT --administration-token *"
parsec backend run -b MOCKED --db MOCKED -P $PORT --administration-token $ADMIN_TOKEN &
sleep 1
python3 misc/initialize_test_organization.py -B "parsec://localhost:$PORT?no_ssl=true" -T $ADMIN_TOKEN $@
