#!bash
source .github/scripts/notary/lib.sh

if [ $# -ne 2 ]; then
    echo "usage: $0 <ORGA> <PROJECT_NUMBER>" 1>&2
    exit 1
fi

PROJECT_ORGA=${1}
PROJECT_NUMBER=${2}

set -o pipefail
echo "Debug tool version"
set -x
gh --version
jq --version
base64 --version
set +x

echo "Looking for issues that are assigned to the wrong project."
echo -n '' > issues_wrong_project.json

for type in issue; do
    gh $type list \
        --json id,title,number \
        --search "-project:\"$PROJECT_ORGA/$PROJECT_NUMBER\"" \
        --jq ".[] += {\"type\": \"$type\"} | .[]" \
        | tee -a issues_wrong_project.json
done

jq -r '. | @base64' issues_wrong_project.json > issues_wrong_project.json.b64

ISSUES_TO_ADD=$(wc -l issues_wrong_project.json.b64 | cut -f 1 -d ' ')
if [ $ISSUES_TO_ADD -eq 0 ]; then
    echo "No issues to add !"
    exit 0
fi

PROJECT_DATA=$(get_project_v2 $PROJECT_ORGA $PROJECT_NUMBER)
PROJECT_ID=$(jq -r .id <<<"$PROJECT_DATA")
PROJECT_TITLE=$(jq -r .title <<<"$PROJECT_DATA")

for raw_row in $(<issues_wrong_project.json.b64); do
    row=$(echo $raw_row | base64 --decode | jq . -c)
    ID=$(jq -r .id <<<"$row")
    TITLE=$(jq -r .title <<<"$row")
    TYPE=$(jq -r .type <<<"$row")

    echo -n "Adding $TYPE \"$TITLE\" to project $PROJECT_TITLE > "
    add_item_to_project $PROJECT_ID $ID
    echo
    RC=$?
    if [ $RC -ne 0 ]; then
        echo "Failed to add $TYPE \"$TITLE\" to project $PROJECT_TITLE" >&2
        exit $RC
    fi
done
