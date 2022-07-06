#!bash
PROJECT_ID=$1
ITEM_ID=$2

QUERY_FILE=.github/scripts/notary/graphql/add_item_to_project_v2.graphql

gh api graphql \
    -f project="$PROJECT_ID" \
    -f item="$ITEM_ID" \
    -f query="$(<$QUERY_FILE)" | cat
