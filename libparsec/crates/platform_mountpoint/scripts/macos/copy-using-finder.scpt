#!/usr/bin/osascript

on run argv
    -- Get file and dest path from arguments
    set file_path to item 1 of argv
    set dest_path to item 2 of argv

    -- Use Finder to copy the file to dest
    tell application "Finder"
        set source_file to POSIX file file_path as alias
        set destination_folder to POSIX file dest_path as alias
        duplicate source_file to destination_folder
    end tell
end run


