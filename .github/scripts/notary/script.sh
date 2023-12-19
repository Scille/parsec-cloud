#!/usr/bin/env bash

SCRIPTDIR=${SCRIPTDIR:="$(dirname "$(realpath -s "$0")")"}
source "$SCRIPTDIR"/lib.sh

if [ $# -ne 2 ]; then
    echo "usage: $0 <ORGA> <PROJECT_NUMBER>" 1>&2
    exit 1
fi

PROJECT_ORGA=${1}
PROJECT_NUMBER=${2}
GH="gh ${GH_ADDITIONAL_ARGS:=""}"

set -o pipefail
echo "Debug tools version:"
echo "- gh version: $(gh --version | head -n 1)"
echo "- jq version: $(jq --version)"
echo "- base64 version: $(base64 --version | head -n 1)"

TMP_DIR=$(mktemp --tmpdir --directory "notary.XXXX")
echo "temporary folder is $TMP_DIR"

echo "Looking for issues that are assigned to the wrong project."

touch $TMP_DIR/issues_wrong_project.json

# Search for Issue that aren't linked to the board already
$GH issue list \
    --json id,title,number \
    --search "-project:\"$PROJECT_ORGA/$PROJECT_NUMBER\"" \
    --jq '.[] += {"type": "issue"} | .[]' \
    | tee -a $TMP_DIR/issues_wrong_project.json

# Search for PRs that aren't linked to the board and linked to an issue
$GH pr list \
    --json id,title,number \
    --search "-project:\"$PROJECT_ORGA/$PROJECT_NUMBER\" -linked:issue" \
    --jq '.[] += {"type": "pr"} | .[]' \
    | tee -a $TMP_DIR/issues_wrong_project.json

jq -r '. | @base64' $TMP_DIR/issues_wrong_project.json > $TMP_DIR/issues_wrong_project.json.b64

ISSUES_TO_ADD=$(wc -l $TMP_DIR/issues_wrong_project.json.b64 | cut -f 1 -d ' ')
if [ $ISSUES_TO_ADD -eq 0 ]; then
    echo "No issues to add !"
    exit 0
fi

PROJECT_DATA=$(get_project_v2 $PROJECT_ORGA $PROJECT_NUMBER)
PROJECT_ID=$(jq -r .id <<<"$PROJECT_DATA")
PROJECT_TITLE=$(jq -r .title <<<"$PROJECT_DATA")

for raw_row in $(<$TMP_DIR/issues_wrong_project.json.b64); do
    row=$(echo $raw_row | base64 --decode | jq . -c)
    ID=$(jq -r .id <<<"$row")
    TITLE=$(jq -r .title <<<"$row")
    TYPE=$(jq -r .type <<<"$row")

    echo -n "Adding $TYPE \"$TITLE\" to project $PROJECT_TITLE > "
    add_item_to_project $PROJECT_ID $ID
    RC=$?
    echo
    if [ $RC -ne 0 ]; then
        echo "Failed to add $TYPE \"$TITLE\" to project $PROJECT_TITLE" >&2
        exit $RC
    fi
done
