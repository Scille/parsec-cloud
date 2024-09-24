#!/bin/bash
# This scripts copies device files (.keys) from the legacy device directory (based on $HOME/snap)
# to the new device directory (based on $HOME/.config)

config_dir=${PARSEC_BASE_CONFIG_DIR:-$HOME/.config}
device_dir=$config_dir/parsec3/libparsec/devices
mkdir -pv "$device_dir"
# Iterate over device directory from all installed parsec snaps
for old_device_dir in $(find "$HOME"/snap -type f -name '*.keys' 2>/dev/null | grep parsec | xargs dirname | sort | uniq); do
    sentinel_file=$old_device_dir/.parsec-migrated-devices
    # Skip directory if it contains the same devices that are stored in the sentinel file
    if [ -f "$sentinel_file" ] && [[ "cat $sentinel_file" == "$(ls -1 "$old_device_dir"/*.keys)" ]]; then
        continue
    fi
    echo "Copying devices from '$old_device_dir' to '$device_dir'"
    # Copy devices preserving attributes (--archive)
    # and ignoring if already exists in destination directory (--update=none)
    if cp --archive --update=none --verbose "$old_device_dir"/*.keys "$device_dir" ; then
        # Register copied devices in a sentinel file
        ls -1 "$old_device_dir" > "$sentinel_file"
    else
        echo "Error copying devices from '$old_device_dir'"
    fi
done
