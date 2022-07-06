#!bash
ORGANIZATION=$1
PROJECT_NUMBER=$2

QUERY_FILE=.github/scripts/notary/graphql/get_project_v2_id.graphql

gh api graphql \
    -f organization="$ORGANIZATION" \
    -F number="$PROJECT_NUMBER" \
    -f query="$(<${QUERY_FILE})" \
    --jq .data.organization.projectV2 | cat
