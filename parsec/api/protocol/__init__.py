# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from parsec._parsec import (
    AuthenticatedAnyCmdReq,
    BlockCreateRep,
    BlockCreateReq,
    BlockReadRep,
    BlockReadReq,
    DeviceName,
    EnrollmentID,
    InvitedAnyCmdReq,
    PkiEnrollmentStatus,
    SequesterServiceID,
    UserProfile,
)
from parsec.api.protocol.base import (
    IncompatibleAPIVersionsError,
    InvalidMessageError,
    MessageSerializationError,
    packb,
    settle_compatible_versions,
    unpackb,
)
from parsec.api.protocol.block import BlockID, block_create_serializer, block_read_serializer
from parsec.api.protocol.cmds import AUTHENTICATED_CMDS, INVITED_CMDS
from parsec.api.protocol.events import events_listen_serializer, events_subscribe_serializer
from parsec.api.protocol.handshake import (
    AuthenticatedClientHandshake,
    BaseClientHandshake,
    HandshakeBadAdministrationToken,
    HandshakeBadIdentity,
    HandshakeError,
    HandshakeFailedChallenge,
    HandshakeOrganizationExpired,
    HandshakeOutOfBallparkError,
    HandshakeRevokedDevice,
    HandshakeRVKMismatch,
    HandshakeType,
    InvitedClientHandshake,
    ServerHandshake,
)
from parsec.api.protocol.invite import (
    InvitationDeletedReason,
    InvitationEmailSentStatus,
    InvitationStatus,
    InvitationStatusField,
    InvitationToken,
    InvitationTokenField,
    InvitationType,
    InvitationTypeField,
    invite_1_claimer_wait_peer_serializer,
    invite_1_greeter_wait_peer_serializer,
    invite_2a_claimer_send_hashed_nonce_serializer,
    invite_2a_greeter_get_hashed_nonce_serializer,
    invite_2b_claimer_send_nonce_serializer,
    invite_2b_greeter_send_nonce_serializer,
    invite_3a_claimer_signify_trust_serializer,
    invite_3a_greeter_wait_peer_trust_serializer,
    invite_3b_claimer_wait_peer_trust_serializer,
    invite_3b_greeter_signify_trust_serializer,
    invite_4_claimer_communicate_serializer,
    invite_4_greeter_communicate_serializer,
    invite_delete_serializer,
    invite_info_serializer,
    invite_list_serializer,
    invite_new_serializer,
)
from parsec.api.protocol.message import message_get_serializer
from parsec.api.protocol.organization import (
    organization_bootstrap_serializer,
    organization_bootstrap_webhook_serializer,
    organization_config_serializer,
    organization_stats_serializer,
)
from parsec.api.protocol.ping import authenticated_ping_serializer, invited_ping_serializer
from parsec.api.protocol.pki import (
    pki_enrollment_accept_serializer,
    pki_enrollment_info_serializer,
    pki_enrollment_list_serializer,
    pki_enrollment_reject_serializer,
    pki_enrollment_submit_serializer,
)
from parsec.api.protocol.realm import (
    MaintenanceType,
    RealmID,
    RealmIDField,
    RealmRole,
    RealmRoleField,
    realm_create_serializer,
    realm_finish_reencryption_maintenance_serializer,
    realm_get_role_certificates_serializer,
    realm_start_reencryption_maintenance_serializer,
    realm_stats_serializer,
    realm_status_serializer,
    realm_update_roles_serializer,
)
from parsec.api.protocol.shamir import (
    invite_shamir_recovery_reveal_serializer,
    shamir_recovery_others_list_serializer,
    shamir_recovery_self_info_serializer,
    shamir_recovery_setup_serializer,
)
from parsec.api.protocol.types import (
    DeviceID,
    DeviceIDField,
    DeviceLabel,
    DeviceLabelField,
    HumanHandle,
    HumanHandleField,
    OrganizationID,
    OrganizationIDField,
    UserID,
    UserIDField,
    UserProfileField,
)
from parsec.api.protocol.user import (
    device_create_serializer,
    human_find_serializer,
    user_create_serializer,
    user_get_serializer,
    user_revoke_serializer,
)
from parsec.api.protocol.vlob import (
    VlobID,
    VlobIDField,
    vlob_create_serializer,
    vlob_list_versions_serializer,
    vlob_maintenance_get_reencryption_batch_serializer,
    vlob_maintenance_save_reencryption_batch_serializer,
    vlob_poll_changes_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
)

__all__ = (
    "MessageSerializationError",
    "InvalidMessageError",
    "packb",
    "unpackb",
    "HandshakeError",
    "HandshakeFailedChallenge",
    "HandshakeBadAdministrationToken",
    "HandshakeBadIdentity",
    "HandshakeOrganizationExpired",
    "HandshakeRVKMismatch",
    "HandshakeRevokedDevice",
    "HandshakeOutOfBallparkError",
    "IncompatibleAPIVersionsError",
    "settle_compatible_versions",
    "ServerHandshake",
    "HandshakeType",
    "BaseClientHandshake",
    "AuthenticatedClientHandshake",
    "InvitedClientHandshake",
    # Types
    "UserID",
    "DeviceID",
    "DeviceName",
    "OrganizationID",
    "HumanHandle",
    "UserIDField",
    "DeviceIDField",
    "OrganizationIDField",
    "HumanHandleField",
    "UserProfileField",
    "UserProfile",
    "DeviceLabelField",
    "DeviceLabel",
    # Organization
    "organization_bootstrap_serializer",
    "organization_bootstrap_webhook_serializer",
    "organization_stats_serializer",
    "organization_config_serializer",
    # Events
    "events_subscribe_serializer",
    "events_listen_serializer",
    # Ping
    "authenticated_ping_serializer",
    "invited_ping_serializer",
    # User
    "user_get_serializer",
    "user_create_serializer",
    "user_revoke_serializer",
    "device_create_serializer",
    "human_find_serializer",
    # Invite
    "InvitationToken",
    "InvitationTokenField",
    "InvitationType",
    "InvitationTypeField",
    "InvitationDeletedReason",
    "InvitationStatus",
    "InvitationStatusField",
    "InvitationEmailSentStatus",
    "invite_new_serializer",
    "invite_delete_serializer",
    "invite_list_serializer",
    "invite_info_serializer",
    "invite_1_claimer_wait_peer_serializer",
    "invite_1_greeter_wait_peer_serializer",
    "invite_2a_claimer_send_hashed_nonce_serializer",
    "invite_2a_greeter_get_hashed_nonce_serializer",
    "invite_2b_greeter_send_nonce_serializer",
    "invite_2b_claimer_send_nonce_serializer",
    "invite_3a_greeter_wait_peer_trust_serializer",
    "invite_3a_claimer_signify_trust_serializer",
    "invite_3b_claimer_wait_peer_trust_serializer",
    "invite_3b_greeter_signify_trust_serializer",
    "invite_4_greeter_communicate_serializer",
    "invite_4_claimer_communicate_serializer",
    # Message
    "message_get_serializer",
    # Realm
    "RealmID",
    "RealmIDField",
    "RealmRole",
    "RealmRoleField",
    "MaintenanceType",
    "realm_create_serializer",
    "realm_status_serializer",
    "realm_stats_serializer",
    "realm_get_role_certificates_serializer",
    "realm_update_roles_serializer",
    "realm_start_reencryption_maintenance_serializer",
    "realm_finish_reencryption_maintenance_serializer",
    # Vlob
    "VlobID",
    "VlobIDField",
    "vlob_create_serializer",
    "vlob_read_serializer",
    "vlob_update_serializer",
    "vlob_poll_changes_serializer",
    "vlob_list_versions_serializer",
    "vlob_maintenance_get_reencryption_batch_serializer",
    "vlob_maintenance_save_reencryption_batch_serializer",
    # Block
    "BlockID",
    "block_create_serializer",
    "block_read_serializer",
    "BlockReadReq",
    "BlockReadRep",
    "BlockCreateReq",
    "BlockCreateRep",
    # PKI enrollment
    "pki_enrollment_submit_serializer",
    "pki_enrollment_info_serializer",
    "pki_enrollment_list_serializer",
    "pki_enrollment_reject_serializer",
    "pki_enrollment_accept_serializer",
    "PkiEnrollmentStatus",
    "EnrollmentID",
    # Sequester
    "SequesterServiceID",
    # Shamir
    "shamir_recovery_others_list_serializer",
    "shamir_recovery_self_info_serializer",
    "shamir_recovery_setup_serializer",
    "invite_shamir_recovery_reveal_serializer",
    # List of cmds
    "AUTHENTICATED_CMDS",
    "INVITED_CMDS",
    "AuthenticatedAnyCmdReq",
    "InvitedAnyCmdReq",
)
