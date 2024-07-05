# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from . import (
    block_create,
    block_read,
    certificate_get,
    device_create,
    events_listen,
    invite_1_greeter_wait_peer,
    invite_2a_greeter_get_hashed_nonce,
    invite_2b_greeter_send_nonce,
    invite_3a_greeter_wait_peer_trust,
    invite_3b_greeter_signify_trust,
    invite_4_greeter_communicate,
    invite_cancel,
    invite_greeter_step,
    invite_list,
    invite_new_device,
    invite_new_user,
    ping,
    pki_enrollment_accept,
    pki_enrollment_list,
    pki_enrollment_reject,
    realm_create,
    realm_get_keys_bundle,
    realm_rename,
    realm_rotate_key,
    realm_share,
    realm_unshare,
    shamir_recovery_setup,
    user_create,
    user_revoke,
    user_update,
    vlob_create,
    vlob_poll_changes,
    vlob_read_batch,
    vlob_read_versions,
    vlob_update,
)

class AnyCmdReq:
    @classmethod
    def load(
        cls, raw: bytes
    ) -> (
        block_create.Req
        | block_read.Req
        | certificate_get.Req
        | device_create.Req
        | events_listen.Req
        | invite_1_greeter_wait_peer.Req
        | invite_2a_greeter_get_hashed_nonce.Req
        | invite_2b_greeter_send_nonce.Req
        | invite_3a_greeter_wait_peer_trust.Req
        | invite_3b_greeter_signify_trust.Req
        | invite_4_greeter_communicate.Req
        | invite_cancel.Req
        | invite_greeter_step.Req
        | invite_list.Req
        | invite_new_device.Req
        | invite_new_user.Req
        | ping.Req
        | pki_enrollment_accept.Req
        | pki_enrollment_list.Req
        | pki_enrollment_reject.Req
        | realm_create.Req
        | realm_get_keys_bundle.Req
        | realm_rename.Req
        | realm_rotate_key.Req
        | realm_share.Req
        | realm_unshare.Req
        | shamir_recovery_setup.Req
        | user_create.Req
        | user_revoke.Req
        | user_update.Req
        | vlob_create.Req
        | vlob_poll_changes.Req
        | vlob_read_batch.Req
        | vlob_read_versions.Req
        | vlob_update.Req
    ): ...

__all__ = [
    "AnyCmdReq",
    "block_create",
    "block_read",
    "certificate_get",
    "device_create",
    "events_listen",
    "invite_1_greeter_wait_peer",
    "invite_2a_greeter_get_hashed_nonce",
    "invite_2b_greeter_send_nonce",
    "invite_3a_greeter_wait_peer_trust",
    "invite_3b_greeter_signify_trust",
    "invite_4_greeter_communicate",
    "invite_cancel",
    "invite_greeter_step",
    "invite_list",
    "invite_new_device",
    "invite_new_user",
    "ping",
    "pki_enrollment_accept",
    "pki_enrollment_list",
    "pki_enrollment_reject",
    "realm_create",
    "realm_get_keys_bundle",
    "realm_rename",
    "realm_rotate_key",
    "realm_share",
    "realm_unshare",
    "shamir_recovery_setup",
    "user_create",
    "user_revoke",
    "user_update",
    "vlob_create",
    "vlob_poll_changes",
    "vlob_read_batch",
    "vlob_read_versions",
    "vlob_update",
]
