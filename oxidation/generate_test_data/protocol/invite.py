# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from libparsec.types import DateTime
from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### InviteNew ##################

serializer = invite_new_serializer

serialized = serializer.req_dumps(
    {"type": "USER", "cmd": "invite_new", "claimer_email": "alice@dev1", "send_email": True}
)
serializer.req_loads(serialized)
display("invite_new_req_user", serialized, [])

serialized = serializer.req_dumps({"type": "DEVICE", "cmd": "invite_new", "send_email": True})
serializer.req_loads(serialized)
display("invite_new_req_device", serialized, [])

serialized = serializer.rep_dumps(
    {
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
        "email_sent": InvitationEmailSentStatus.SUCCESS,
    }
)
serializer.rep_loads(serialized)
display("invite_new_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_allowed"})
serializer.rep_loads(serialized)
display("invite_new_rep_not_allowed", serialized, [])

serialized = serializer.rep_dumps({"status": "already_member"})
serializer.rep_loads(serialized)
display("invite_new_rep_already_member", serialized, [])

serialized = serializer.rep_dumps({"status": "not_available"})
serializer.rep_loads(serialized)
display("invite_new_rep_not_available", serialized, [])

################### InviteDelete ##################

serializer = invite_delete_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_delete",
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
        "reason": InvitationDeletedReason.FINISHED,
    }
)
serializer.req_loads(serialized)
display("invite_delete_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_delete_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_delete_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_deleted"})
serializer.rep_loads(serialized)
display("invite_delete_rep_already_deleted", serialized, [])

################### InviteList ##################

serializer = invite_list_serializer

serialized = serializer.req_dumps({"cmd": "invite_list"})
serializer.req_loads(serialized)
display("invite_list_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "invitations": [
            {
                "type": "USER",
                "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
                "created_on": DateTime(2000, 1, 2, 1),
                "claimer_email": "alice@dev1",
                "status": InvitationStatus.IDLE,
            },
            {
                "type": "DEVICE",
                "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
                "created_on": DateTime(2000, 1, 2, 1),
                "status": InvitationStatus.IDLE,
            },
        ]
    }
)
serializer.rep_loads(serialized)
display("invite_list_rep", serialized, [])

################### InviteInfo ##################

serializer = invite_info_serializer

serialized = serializer.req_dumps({"cmd": "invite_info"})
serializer.req_loads(serialized)
display("invite_info_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "type": "USER",
        "claimer_email": "alice@dev1",
        "greeter_user_id": UserID("109b68ba5cdf428ea0017fc6bcc04d4a"),
        "greeter_human_handle": HumanHandle("bob@dev1", "bob"),
    }
)
serializer.rep_loads(serialized)
display("invite_info_rep_user", serialized, [])

serialized = serializer.rep_dumps(
    {
        "type": "DEVICE",
        "greeter_user_id": UserID("109b68ba5cdf428ea0017fc6bcc04d4a"),
        "greeter_human_handle": HumanHandle("bob@dev1", "bob"),
    }
)
serializer.rep_loads(serialized)
display("invite_info_rep_device", serialized, [])

################### Invite1ClaimerWaitPeer ##################

serializer = invite_1_claimer_wait_peer_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_1_claimer_wait_peer",
        "claimer_public_key": PublicKey(
            unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        ),
    }
)
serializer.req_loads(serialized)
display("invite_1_claimer_wait_peer_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "greeter_public_key": PublicKey(
            unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        )
    }
)
serializer.rep_loads(serialized)
display("invite_1_claimer_wait_peer_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_1_claimer_wait_peer_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_1_claimer_wait_peer_rep_invalid_state", serialized, [])

################### Invite1GreeterWaitPeer ##################

serializer = invite_1_greeter_wait_peer_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_1_greeter_wait_peer",
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
        "greeter_public_key": PublicKey(
            unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        ),
    }
)
serializer.req_loads(serialized)
display("invite_1_greeter_wait_peer_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "claimer_public_key": PublicKey(
            unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        )
    }
)
serializer.rep_loads(serialized)
display("invite_1_greeter_wait_peer_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_1_greeter_wait_peer_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_deleted"})
serializer.rep_loads(serialized)
display("invite_1_greeter_wait_peer_rep_already_deleted", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_1_greeter_wait_peer_rep_invalid_state", serialized, [])

################### Invite2aClaimerSendHashedNonce ##################

serializer = invite_2a_claimer_send_hashed_nonce_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_2a_claimer_send_hashed_nonce_hash_nonce",
        "claimer_hashed_nonce": HashDigest(
            unhexlify("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
        ),
    }
)
serializer.req_loads(serialized)
display("invite_2a_claimer_send_hashed_nonce_hash_nonce_req", serialized, [])

serialized = serializer.rep_dumps({"greeter_nonce": b"foobar"})
serializer.rep_loads(serialized)
display("invite_2a_claimer_send_hashed_nonce_hash_nonce_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_2a_claimer_send_hashed_nonce_hash_nonce_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_deleted"})
serializer.rep_loads(serialized)
display("invite_2a_claimer_send_hashed_nonce_hash_nonce_rep_already_deleted", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_2a_claimer_send_hashed_nonce_hash_nonce_rep_invalid_state", serialized, [])

################### Invite2aGreeterGetHashedNonce ##################

serializer = invite_2a_greeter_get_hashed_nonce_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_2a_greeter_get_hashed_nonce",
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
    }
)
serializer.req_loads(serialized)
display("invite_2a_greeter_get_hashed_nonce_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "claimer_hashed_nonce": HashDigest(
            unhexlify("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
        )
    }
)
serializer.rep_loads(serialized)
display("invite_2a_greeter_get_hashed_nonce_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_2a_greeter_get_hashed_nonce_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_deleted"})
serializer.rep_loads(serialized)
display("invite_2a_greeter_get_hashed_nonce_rep_already_deleted", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_2a_greeter_get_hashed_nonce_rep_invalid_state", serialized, [])

################### Invite2bGreeterSendNonce ##################

serializer = invite_2b_greeter_send_nonce_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_2b_greeter_send_nonce",
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
        "greeter_nonce": b"foobar",
    }
)
serializer.req_loads(serialized)
display("invite_2b_greeter_send_nonce_req", serialized, [])

serialized = serializer.rep_dumps({"claimer_nonce": b"foobar"})
serializer.rep_loads(serialized)
display("invite_2b_greeter_send_nonce_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_2b_greeter_send_nonce_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_deleted"})
serializer.rep_loads(serialized)
display("invite_2b_greeter_send_nonce_rep_already_deleted", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_2b_greeter_send_nonce_rep_invalid_state", serialized, [])

################## Invite2bClaimerSendNonce ##################

serializer = invite_2b_claimer_send_nonce_serializer

serialized = serializer.req_dumps(
    {"cmd": "invite_2b_claimer_send_nonce", "claimer_nonce": b"foobar"}
)
serializer.req_loads(serialized)
display("invite_2b_claimer_send_nonce_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_2b_claimer_send_nonce_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_2b_claimer_send_nonce_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_2b_claimer_send_nonce_rep_invalid_state", serialized, [])

################### Invite3aGreeterWaitPeerTrust ##################

serializer = invite_3a_greeter_wait_peer_trust_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_3a_greeter_wait_peer_trust",
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
    }
)
serializer.req_loads(serialized)
display("invite_3a_greeter_wait_peer_trust_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_3a_greeter_wait_peer_trust_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_3a_greeter_wait_peer_trust_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_deleted"})
serializer.rep_loads(serialized)
display("invite_3a_greeter_wait_peer_trust_rep_already_deleted", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_3a_greeter_wait_peer_trust_rep_invalid_state", serialized, [])

################### Invite3bClaimerWaitPeerTrust ##################

serializer = invite_3b_claimer_wait_peer_trust_serializer

serialized = serializer.req_dumps({"cmd": "invite_3b_claimer_wait_peer_trust"})
serializer.req_loads(serialized)
display("invite_3b_claimer_wait_peer_trust_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_3b_claimer_wait_peer_trust_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_3b_claimer_wait_peer_trust_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_3b_claimer_wait_peer_trust_rep_invalid_state", serialized, [])

################### Invite3bGreeterSignifyTrust ##################

serializer = invite_3b_greeter_signify_trust_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_3b_greeter_signify_trust",
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
    }
)
serializer.req_loads(serialized)
display("invite_3b_greeter_signify_trust_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_3b_greeter_signify_trust_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_3b_greeter_signify_trust_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_deleted"})
serializer.rep_loads(serialized)
display("invite_3b_greeter_signify_trust_rep_already_deleted", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_3b_greeter_signify_trust_rep_invalid_state", serialized, [])

################### Invite3aClaimerSignifyTrust ##################

serializer = invite_3a_claimer_signify_trust_serializer

serialized = serializer.req_dumps({"cmd": "invite_3a_claimer_signify_trust"})
serializer.req_loads(serialized)
display("invite_3a_claimer_signify_trust_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_3a_claimer_signify_trust_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_3a_claimer_signify_trust_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_3a_claimer_signify_trust_rep_invalid_state", serialized, [])

################### Invite4GreeterCommunicate ##################

serializer = invite_4_greeter_communicate_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "invite_4_greeter_communicate",
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
        "payload": b"foobar",
    }
)
serializer.req_loads(serialized)
display("invite_4_greeter_communicate_req", serialized, [])

serialized = serializer.rep_dumps({"payload": b"foobar"})
serializer.rep_loads(serialized)
display("invite_4_greeter_communicate_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_4_greeter_communicate_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "already_deleted"})
serializer.rep_loads(serialized)
display("invite_4_greeter_communicate_rep_already_deleted", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_4_greeter_communicate_rep_invalid_state", serialized, [])

################### Invite4ClaimerCommunicate ##################

serializer = invite_4_claimer_communicate_serializer

serialized = serializer.req_dumps({"cmd": "invite_4_claimer_communicate", "payload": b"foobar"})
serializer.req_loads(serialized)
display("invite_4_claimer_communicate_req", serialized, [])

serialized = serializer.rep_dumps({"payload": b"foobar"})
serializer.rep_loads(serialized)
display("invite_4_claimer_communicate_rep", serialized, [])

serialized = serializer.rep_dumps({"status": "not_found"})
serializer.rep_loads(serialized)
display("invite_4_claimer_communicate_rep_not_found", serialized, [])

serialized = serializer.rep_dumps({"status": "invalid_state"})
serializer.rep_loads(serialized)
display("invite_4_claimer_communicate_rep_invalid_state", serialized, [])
