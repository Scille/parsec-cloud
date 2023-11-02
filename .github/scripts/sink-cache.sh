#!/usr/bin/env bash

# This script require that you have `gh` installed (Github CLI: https://cli.github.com/)
if [ -z "$(command -v gh)" ]; then
    echo "Github CLI is not installed" >&2
    echo "Follow the instruction at https://cli.github.com/ to install it" >&2
    exit 1
fi

# Alongside `gh` you also need to have added the `gh-actions-cache` extension (https://github.com/actions/gh-actions-cache)
GH_EXT_ACTIONS_CACHE="$(gh extension list | grep 'actions/gh-actions-cache')"
if [ -z "$GH_EXT_ACTIONS_CACHE" ]; then
    echo "Github CLI is missing the extension 'actions/gh-actions-cache'" >&2
    echo "Install it with" >&2
    echo >&2
    echo "gh extension install actions/gh-actions-cache"
    exit 1
fi
GH_EXT_ACTIONS_CACHE_VERSION=$(echo "$GH_EXT_ACTIONS_CACHE" | cut -f4 -d ' ')

echo "Using actions/gh-actions-cache at $GH_EXT_ACTIONS_CACHE_VERSION"

function remove_cache_entry {
    gh actions-cache delete --confirm "$1"
}

function list_cache_entries {
    gh actions-cache list --sort size --limit 5 | cut -f 1
}

if [ $# -gt 0 ]; then
    # Delete the specfied cache entries
    for entry in "$@"; do
        remove_cache_entry "$entry"
    done
else
    # Delete all cache entries
    ENTRIES=$(list_cache_entries)
    until [ -z "$ENTRIES" ]; do
        (
            # We change IFS to split each entries by newline
            IFS=$'\n';
            for entry in "$ENTRIES"; do
                remove_cache_entry "$entry" &
            done
            wait
        )
        ENTRIES=$(list_cache_entries)
    done
fi
