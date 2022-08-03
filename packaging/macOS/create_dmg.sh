#!/bin/sh
: '
MacOS-only script, using the project create-dmg which needs to be installed.
`brew install create-dmg`

Path to the app in its dist folder is needed as argument.
'


# Check if variable isn't empty and has the app
if [ -z "$1" ]; then
    echo "Error: No argument given, path to app needed"
    exit 1
elif ! [[ $1 == *"/dist/Parsec.app" ]]; then
    echo "Error: Path to dist/Parsec.app needed as argument"
    exit 1
fi


# Get .app parent directory
if [[ $1 == "/Users/"* ]]; then # Check if path is already absolute
    APP_DIR=`dirname $1`
else
    APP_DIR=`dirname $PWD/$1`
fi


# Check if said directory exists
if [ ! -d "$APP_DIR" ]; then
    echo "Error: Given argument isn't a directory or doesn't exist"
    exit 1
fi


# Might be a remaining .zip file in the folder from notarization
rm $APP_DIR/parsec.zip


# Check if folder is empty, a bit overkill
if [ `ls $APP_DIR | wc -l | xargs` = 0 ]; then
    echo "Error: Empty dist folder"
    exit 1
fi


# Get script directory to find background image later
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )


# Get app version and set .dmg name accordingly
VERSION=`plutil -p $APP_DIR/Parsec.app/Contents/Info.plist | grep CFBundleShortVersionString | cut -f2 -d">" | xargs`
DMG_NAME="parsec-${VERSION}-macos-amd64.dmg"


# Build & compress as .dmg
create-dmg \
    --background $SCRIPT_DIR/dmg/background.png \
    --text-size 16 \
    --icon-size 156 \
    --window-pos 200 120 \
    --window-size 750 480 \
    --icon Parsec.app 178 220 \
    --app-drop-link 570 220 \
    $DMG_NAME $APP_DIR
