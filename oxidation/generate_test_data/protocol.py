# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS
# flake8: noqa

from pendulum import datetime
from utils import *
from parsec.crypto import *
from parsec.api.protocol import *
from parsec.api.data import *

################### BlockCreate ##################

serializer = block_create_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "block_create",
        "block_id": BlockID.from_hex("57c629b69d6c4abbaf651cafa46dbc93"),
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "block": b"foobar",
    }
)
serializer.req_loads(serialized)
display("block create req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("block create rep", serialized, [])

################### BlockRead ##################

serializer = block_read_serializer

serialized = serializer.req_dumps(
    {"cmd": "block_read", "block_id": BlockID.from_hex("57c629b69d6c4abbaf651cafa46dbc93")}
)
serializer.req_loads(serialized)
display("block read req", serialized, [])

serialized = serializer.rep_dumps({"block": b"foobar"})
serializer.rep_loads(serialized)
display("block read rep", serialized, [])

################### Ping ##################

serializer = ping_serializer

serialized = serializer.req_dumps({"cmd": "ping", "ping": "ping"})
serializer.req_loads(serialized)
display("ping req", serialized, [])

serialized = serializer.rep_dumps({"pong": "pong"})
serializer.rep_loads(serialized)
display("ping rep", serialized, [])

################### Invite ##################

serializer = invite_new_serializer

serialized = serializer.req_dumps(
    {"type": "USER", "cmd": "invite_new", "claimer_email": "alice@dev1", "send_email": True}
)
serializer.req_loads(serialized)
display("invite_new_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "token": InvitationToken.from_hex("d864b93ded264aae9ae583fd3d40c45a"),
        "email_sent": InvitationEmailSentStatus.SUCCESS,
    }
)
serializer.rep_loads(serialized)
display("invite_new_rep", serialized, [])

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
                "created_on": datetime(2000, 1, 2, 1),
                "claimer_email": "alice@dev1",
                "status": InvitationStatus.IDLE,
            }
        ]
    }
)
serializer.rep_loads(serialized)
display("invite_list_rep", serialized, [])

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
display("invite_info_rep", serialized, [])

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

serializer = invite_2a_greeter_get_hashed_nonce_serializer

serialized = serializer.rep_dumps(
    {
        "claimer_hashed_nonce": HashDigest(
            unhexlify("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
        )
    }
)
serializer.rep_loads(serialized)
display("invite_2a_greeter_get_hashed_nonce_rep", serialized, [])

serializer = invite_2b_greeter_send_nonce_serializer

serialized = serializer.rep_dumps({"claimer_nonce": b"foobar"})
serializer.rep_loads(serialized)
display("invite_2b_greeter_send_nonce_rep", serialized, [])

serializer = invite_2b_claimer_send_nonce_serializer

serialized = serializer.req_dumps(
    {"cmd": "invite_2b_claimer_send_nonce", "claimer_nonce": b"foobar"}
)
serializer.req_loads(serialized)
display("invite_2b_claimer_send_nonce_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_2b_claimer_send_nonce_rep", serialized, [])

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

serializer = invite_3b_claimer_wait_peer_trust_serializer

serialized = serializer.req_dumps({"cmd": "invite_3b_claimer_wait_peer_trust"})
serializer.req_loads(serialized)
display("invite_3b_claimer_wait_peer_trust_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_3b_claimer_wait_peer_trust_rep", serialized, [])

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

serializer = invite_3a_claimer_signify_trust_serializer

serialized = serializer.req_dumps({"cmd": "invite_3a_claimer_signify_trust"})
serializer.req_loads(serialized)
display("invite_3a_claimer_signify_trust_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("invite_3a_claimer_signify_trust_rep", serialized, [])

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

serializer = invite_4_claimer_communicate_serializer

serialized = serializer.req_dumps({"cmd": "invite_4_claimer_communicate", "payload": b"foobar"})
serializer.req_loads(serialized)
display("invite_4_claimer_communicate_req", serialized, [])

serialized = serializer.rep_dumps({"payload": b"foobar"})
serializer.rep_loads(serialized)
display("invite_4_claimer_communicate_rep", serialized, [])

################### Organization ##################

serializer = apiv1_organization_bootstrap_serializer
serialized = serializer.req_dumps(
    {
        "cmd": "api_v1_organization_bootstrap",
        "bootstrap_token": "foobar",
        "root_verify_key": VerifyKey(
            unhexlify("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
        ),
        "user_certificate": b"foobar",
        "device_certificate": b"foobar",
        "redacted_user_certificate": b"foobar",
        "redacted_device_certificate": b"foobar",
    }
)
serializer.req_loads(serialized)
display("api_v1_organization_bootstrap_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("api_v1_organization_bootstrap_rep", serialized, [])

serializer = organization_stats_serializer

serialized = serializer.req_dumps({"cmd": "organization_stats"})
serializer.req_loads(serialized)
display("organization_stats_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "data_size": 8,
        "metadata_size": 8,
        "realms": 1,
        "users": 1,
        "active_users": 1,
        "users_per_profile_detail": [{"profile": UserProfile.ADMIN, "active": 1, "revoked": 0}],
    }
)
serializer.rep_loads(serialized)
display("organization_stats_rep", serialized, [])

serializer = organization_config_serializer

serialized = serializer.req_dumps({"cmd": "organization_config"})
serializer.req_loads(serialized)
display("organization_config_req", serialized, [])

serialized = serializer.rep_dumps({"user_profile_outsider_allowed": False, "active_users_limit": 1})
serializer.rep_loads(serialized)
display("organization_config_rep", serialized, [])

################### Realm ##################

serializer = realm_create_serializer

serialized = serializer.req_dumps({"cmd": "realm_create", "role_certificate": b"foobar"})
serializer.req_loads(serialized)
display("realm_create_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_create_rep", serialized, [])

serializer = realm_status_serializer

serialized = serializer.req_dumps(
    {"cmd": "realm_status", "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5")}
)
serializer.req_loads(serialized)
display("realm_status_req", serialized, [])

serialized = serializer.rep_dumps(
    {
        "in_maintenance": True,
        "maintenance_type": MaintenanceType.GARBAGE_COLLECTION,
        "maintenance_started_on": datetime(2000, 1, 2, 1),
        "maintenance_started_by": DeviceID("alice@dev1"),
        "encryption_revision": 8,
    }
)
serializer.rep_loads(serialized)
display("realm_status_rep", serialized, [])

serializer = realm_stats_serializer

serialized = serializer.req_dumps(
    {"cmd": "realm_stats", "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5")}
)
serializer.req_loads(serialized)
display("realm_stats_req", serialized, [])

serialized = serializer.rep_dumps({"blocks_size": 8, "vlobs_size": 8})
serializer.rep_loads(serialized)
display("realm_stats_rep", serialized, [])

serializer = realm_get_role_certificates_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_get_role_certificates",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "since": datetime(2000, 1, 2, 1),
    }
)
serializer.req_loads(serialized)
display("realm_get_role_certificates_req", serialized, [])

serialized = serializer.rep_dumps({"certificates": [b"foobar"]})
serializer.rep_loads(serialized)
display("realm_get_role_certificates_rep", serialized, [])

serializer = realm_update_roles_serializer

serialized = serializer.req_dumps(
    {"cmd": "realm_update_roles", "role_certificate": b"foobar", "recipient_message": b"foobar"}
)
serializer.req_loads(serialized)
display("realm_update_roles_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_update_roles_rep", serialized, [])

serializer = realm_start_reencryption_maintenance_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_start_reencryption_maintenance",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
        "timestamp": datetime(2000, 1, 2, 1),
        "per_participant_message": {UserID("109b68ba5cdf428ea0017fc6bcc04d4a"): b"foobar"},
    }
)
serializer.req_loads(serialized)
display("realm_start_reencryption_maintenance_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_start_reencryption_maintenance_rep", serialized, [])

serializer = realm_finish_reencryption_maintenance_serializer

serialized = serializer.req_dumps(
    {
        "cmd": "realm_finish_reencryption_maintenance",
        "realm_id": RealmID.from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5"),
        "encryption_revision": 8,
    }
)
serializer.req_loads(serialized)
display("realm_finish_reencryption_maintenance_req", serialized, [])

serialized = serializer.rep_dumps({})
serializer.rep_loads(serialized)
display("realm_finish_reencryption_maintenance_rep", serialized, [])
