# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from . import (
    block_create,
    block_read,
    device_create,
    events_listen,
    events_subscribe,
    human_find,
    invite_1_greeter_wait_peer,
    invite_2a_greeter_get_hashed_nonce,
    invite_2b_greeter_send_nonce,
    invite_3a_greeter_wait_peer_trust,
    invite_3b_greeter_signify_trust,
    invite_4_greeter_communicate,
    invite_delete,
    invite_list,
    invite_new,
    message_get,
    organization_config,
    organization_stats,
    ping,
    pki_enrollment_accept,
    pki_enrollment_list,
    pki_enrollment_reject,
    realm_create,
    realm_finish_reencryption_maintenance,
    realm_get_role_certificates,
    realm_start_reencryption_maintenance,
    realm_stats,
    realm_status,
    realm_update_roles,
    user_create,
    user_get,
    user_revoke,
    vlob_create,
    vlob_list_versions,
    vlob_maintenance_get_reencryption_batch,
    vlob_maintenance_save_reencryption_batch,
    vlob_poll_changes,
    vlob_read,
    vlob_update,
)

class AnyCmdReq:
    @classmethod
    def load(
        cls, raw: bytes
    ) -> (
        block_create.Req
        | block_read.Req
        | device_create.Req
        | events_listen.Req
        | events_subscribe.Req
        | human_find.Req
        | invite_1_greeter_wait_peer.Req
        | invite_2a_greeter_get_hashed_nonce.Req
        | invite_2b_greeter_send_nonce.Req
        | invite_3a_greeter_wait_peer_trust.Req
        | invite_3b_greeter_signify_trust.Req
        | invite_4_greeter_communicate.Req
        | invite_delete.Req
        | invite_list.Req
        | invite_new.Req
        | message_get.Req
        | organization_config.Req
        | organization_stats.Req
        | ping.Req
        | pki_enrollment_accept.Req
        | pki_enrollment_list.Req
        | pki_enrollment_reject.Req
        | realm_create.Req
        | realm_finish_reencryption_maintenance.Req
        | realm_get_role_certificates.Req
        | realm_start_reencryption_maintenance.Req
        | realm_stats.Req
        | realm_status.Req
        | realm_update_roles.Req
        | user_create.Req
        | user_get.Req
        | user_revoke.Req
        | vlob_create.Req
        | vlob_list_versions.Req
        | vlob_maintenance_get_reencryption_batch.Req
        | vlob_maintenance_save_reencryption_batch.Req
        | vlob_poll_changes.Req
        | vlob_read.Req
        | vlob_update.Req
    ): ...

__all__ = [
    "AnyCmdReq",
    "block_create",
    "block_read",
    "device_create",
    "events_listen",
    "events_subscribe",
    "human_find",
    "invite_1_greeter_wait_peer",
    "invite_2a_greeter_get_hashed_nonce",
    "invite_2b_greeter_send_nonce",
    "invite_3a_greeter_wait_peer_trust",
    "invite_3b_greeter_signify_trust",
    "invite_4_greeter_communicate",
    "invite_delete",
    "invite_list",
    "invite_new",
    "message_get",
    "organization_config",
    "organization_stats",
    "ping",
    "pki_enrollment_accept",
    "pki_enrollment_list",
    "pki_enrollment_reject",
    "realm_create",
    "realm_finish_reencryption_maintenance",
    "realm_get_role_certificates",
    "realm_start_reencryption_maintenance",
    "realm_stats",
    "realm_status",
    "realm_update_roles",
    "user_create",
    "user_get",
    "user_revoke",
    "vlob_create",
    "vlob_list_versions",
    "vlob_maintenance_get_reencryption_batch",
    "vlob_maintenance_save_reencryption_batch",
    "vlob_poll_changes",
    "vlob_read",
    "vlob_update",
]
