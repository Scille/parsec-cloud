# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS


AUTHENTICATED_CMDS = {
    "events_subscribe",
    "events_listen",
    "ping",
    # Message
    "message_get",
    # User&Device
    "user_get",
    "user_find",
    "user_invite",
    "user_cancel_invitation",
    "user_create",
    "user_revoke",
    "device_invite",
    "device_cancel_invitation",
    "device_create",
    # Human
    "human_find",
    # Block
    "block_create",
    "block_read",
    # Vlob
    "vlob_poll_changes",
    "vlob_create",
    "vlob_read",
    "vlob_update",
    "vlob_list_versions",
    "vlob_maintenance_get_reencryption_batch",
    "vlob_maintenance_save_reencryption_batch",
    # Realm
    "realm_create",
    "realm_status",
    "realm_get_role_certificates",
    "realm_update_roles",
    "realm_start_reencryption_maintenance",
    "realm_finish_reencryption_maintenance",
}
ANONYMOUS_CMDS = {
    "user_claim",
    "user_get_invitation_creator",
    "device_claim",
    "device_get_invitation_creator",
    "organization_bootstrap",
    "ping",
}
ADMINISTRATION_CMDS = {
    "organization_create",
    "organization_stats",
    "organization_status",
    "organization_update",
    "ping",
}
