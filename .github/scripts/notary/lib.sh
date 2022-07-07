#!bash

QUERY_FOLDER=.github/scripts/notary/graphql

# List project fields like `status`, `tag`, `labels` and other
function list_project_fields() {
    local QUERY_FILE=$QUERY_FOLDER/list_field_id.graphql

    local project_id="$1"

    gh api graphql \
        -f project="$project_id" \
        -f query="$(<$QUERY_FILE)" \
        --jq .data.node.fields.nodes | cat
}

# Update a project item field value
# Can be used to update `labels`, `status` and other fields
function update_project_item_field() {
    local QUERY_FILE=$QUERY_FOLDER/update_field.graphql

    local project_id="$1"
    local item_id="$2"
    local field_id="$3"
    local field_value="$4"

    gh api graphql \
        -f project="$1" \
        -f item="$2" \
        -f field="$3" \
        -f query="$(<$QUERY_FILE)" | cat
}

# Retrieve the organization project identified by:
#
# - an organization name (`:organization`)
# - a project number (`:project_number`)
#
# You could find those info in the below url template
# https://github.com/orgs/:organization/projects/:project_number
function get_project_v2() {
    local QUERY_FILE=$QUERY_FOLDER/get_project_v2_id.graphql

    local organization="$1"
    local project_number="$2"

    gh api graphql \
        -f organization="$organization" \
        -F number="$project_number" \
        -f query="$(<${QUERY_FILE})" \
        --jq .data.organization.projectV2 | cat
}

# Add an item like a issue or a pr to a project
function add_item_to_project() {
    local QUERY_FILE=$QUERY_FOLDER/add_item_to_project_v2.graphql

    local project_id="$1"
    local item_id="$2"

    gh api graphql \
        -f project="$project_id" \
        -f item="$item_id" \
        -f query="$(<$QUERY_FILE)" | cat
}

# Get project status field id with the desired field value id
function project_status_field_with_init_value() {
    local project_id=$1
    local init_status_name=$2

    list_project_fields $project_id | jq ". | map(select(.name == \"Status\"))[0] | {\"name\": .name, \"id\": .id, \"init_status_id\": .options | map(select(.name == \"$init_status_name\"))[0].id }"
}
