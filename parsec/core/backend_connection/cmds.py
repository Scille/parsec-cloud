# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Any, Awaitable, Iterable, Union, cast

from parsec._parsec import (
    ArchivingConfigRep,
    AuthenticatedPingRep,
    BlockCreateRep,
    BlockReadRep,
    DateTime,
    DeviceCreateRep,
    EnrollmentID,
    EventsListenRep,
    EventsSubscribeRep,
    HashDigest,
    HumanFindRep,
    InvitationDeletedReason,
    Invite1ClaimerWaitPeerRep,
    Invite1GreeterWaitPeerRep,
    Invite2aClaimerSendHashedNonceRep,
    Invite2aGreeterGetHashedNonceRep,
    Invite2bClaimerSendNonceRep,
    Invite2bGreeterSendNonceRep,
    Invite3aClaimerSignifyTrustRep,
    Invite3aGreeterWaitPeerTrustRep,
    Invite3bClaimerWaitPeerTrustRep,
    Invite3bGreeterSignifyTrustRep,
    Invite4ClaimerCommunicateRep,
    Invite4GreeterCommunicateRep,
    InviteDeleteRep,
    InvitedPingRep,
    InviteInfoRep,
    InviteListRep,
    InviteNewRep,
    InviteShamirRecoveryRevealRep,
    MessageGetRep,
    OrganizationBootstrapRep,
    OrganizationBootstrapRepBadTimestamp,
    OrganizationConfigRep,
    OrganizationStatsRep,
    PkiEnrollmentAcceptRep,
    PkiEnrollmentInfoRep,
    PkiEnrollmentListRep,
    PkiEnrollmentRejectRep,
    PkiEnrollmentSubmitRep,
    ProtocolError,
    PublicKey,
    RealmCreateRep,
    RealmCreateRepBadTimestamp,
    RealmFinishReencryptionMaintenanceRep,
    RealmGetRoleCertificatesRep,
    RealmStartReencryptionMaintenanceRep,
    RealmStartReencryptionMaintenanceRepBadTimestamp,
    RealmStatsRep,
    RealmStatusRep,
    RealmUpdateArchivingRep,
    RealmUpdateArchivingRepBadTimestamp,
    RealmUpdateRolesRep,
    RealmUpdateRolesRepBadTimestamp,
    ReencryptionBatchEntry,
    ShamirRecoveryOthersListRep,
    ShamirRecoverySelfInfoRep,
    ShamirRecoverySetup,
    ShamirRecoverySetupRep,
    ShamirRevealToken,
    UserCreateRep,
    UserGetRep,
    UserRevokeRep,
    VerifyKey,
    VlobCreateRep,
    VlobCreateRepBadTimestamp,
    VlobListVersionsRep,
    VlobMaintenanceGetReencryptionBatchRep,
    VlobMaintenanceSaveReencryptionBatchRep,
    VlobPollChangesRep,
    VlobReadRep,
    VlobUpdateRep,
    VlobUpdateRepBadTimestamp,
)
from parsec.api.protocol import (
    BlockID,
    InvitationToken,
    InvitationType,
    RealmID,
    SequesterServiceID,
    UserID,
    VlobID,
    archiving_config_serializer,
    authenticated_ping_serializer,
    block_create_serializer,
    block_read_serializer,
    device_create_serializer,
    events_listen_serializer,
    events_subscribe_serializer,
    human_find_serializer,
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
    invite_shamir_recovery_reveal_serializer,
    invited_ping_serializer,
    message_get_serializer,
    organization_bootstrap_serializer,
    organization_config_serializer,
    organization_stats_serializer,
    pki_enrollment_accept_serializer,
    pki_enrollment_list_serializer,
    pki_enrollment_reject_serializer,
    realm_create_serializer,
    realm_finish_reencryption_maintenance_serializer,
    realm_get_role_certificates_serializer,
    realm_start_reencryption_maintenance_serializer,
    realm_status_serializer,
    realm_update_archiving_serializer,
    realm_update_roles_serializer,
    shamir_recovery_others_list_serializer,
    shamir_recovery_self_info_serializer,
    shamir_recovery_setup_serializer,
    user_create_serializer,
    user_get_serializer,
    user_revoke_serializer,
    vlob_create_serializer,
    vlob_list_versions_serializer,
    vlob_maintenance_get_reencryption_batch_serializer,
    vlob_maintenance_save_reencryption_batch_serializer,
    vlob_poll_changes_serializer,
    vlob_read_serializer,
    vlob_update_serializer,
)
from parsec.api.protocol.base import ApiCommandSerializer
from parsec.api.transport import Transport, TransportError
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendOutOfBallparkError,
    BackendProtocolError,
)

COMMAND_RETURN_TYPE = Union[
    AuthenticatedPingRep,
    BlockCreateRep,
    BlockReadRep,
    DeviceCreateRep,
    EventsListenRep,
    EventsSubscribeRep,
    HumanFindRep,
    Invite1ClaimerWaitPeerRep,
    Invite1GreeterWaitPeerRep,
    Invite2aClaimerSendHashedNonceRep,
    Invite2aGreeterGetHashedNonceRep,
    Invite2aGreeterGetHashedNonceRep,
    Invite2bClaimerSendNonceRep,
    Invite2bGreeterSendNonceRep,
    Invite3aClaimerSignifyTrustRep,
    Invite3aGreeterWaitPeerTrustRep,
    Invite3bClaimerWaitPeerTrustRep,
    Invite3bGreeterSignifyTrustRep,
    Invite4ClaimerCommunicateRep,
    Invite4GreeterCommunicateRep,
    InviteDeleteRep,
    InviteInfoRep,
    InviteListRep,
    InviteNewRep,
    InvitedPingRep,
    MessageGetRep,
    OrganizationConfigRep,
    OrganizationStatsRep,
    PkiEnrollmentAcceptRep,
    PkiEnrollmentInfoRep,
    PkiEnrollmentListRep,
    PkiEnrollmentRejectRep,
    PkiEnrollmentSubmitRep,
    RealmCreateRep,
    RealmFinishReencryptionMaintenanceRep,
    RealmFinishReencryptionMaintenanceRep,
    RealmGetRoleCertificatesRep,
    RealmStartReencryptionMaintenanceRep,
    RealmStatsRep,
    RealmStatusRep,
    RealmUpdateRolesRep,
    RealmUpdateArchivingRep,
    UserCreateRep,
    UserGetRep,
    UserRevokeRep,
    VlobCreateRep,
    VlobListVersionsRep,
    VlobMaintenanceGetReencryptionBatchRep,
    VlobMaintenanceSaveReencryptionBatchRep,
    VlobPollChangesRep,
    VlobReadRep,
    VlobUpdateRep,
    ShamirRecoveryOthersListRep,
    ShamirRecoverySelfInfoRep,
    ShamirRecoverySetupRep,
    InviteShamirRecoveryRevealRep,
    dict[str, object],
]


async def _send_cmd(
    transport: Transport, serializer: ApiCommandSerializer, **req: object
) -> COMMAND_RETURN_TYPE:
    """
    Raises:
        Backend
        BackendNotAvailable
        BackendProtocolError

        BackendCmdsInvalidRequest
        BackendCmdsInvalidResponse
        BackendNotAvailable
        BackendCmdsBadResponse
    """
    cmd = req["cmd"]
    transport.logger.info("Request", cmd=cmd)
    try:
        raw_req = serializer.req_dumps(req)

    except ProtocolError as exc:
        transport.logger.exception("Invalid request data", cmd=cmd, error=exc)
        raise BackendProtocolError("Invalid request data") from exc

    try:
        await transport.send(raw_req)
        raw_rep = await transport.recv()

    except TransportError as exc:
        transport.logger.debug("Request failed (backend not available)", cmd=cmd)
        raise BackendNotAvailable(exc) from exc

    try:
        rep: dict[str, Any] = serializer.rep_loads(raw_rep)

    except ProtocolError as exc:
        transport.logger.exception("Invalid response data", cmd=cmd, error=exc)
        raise BackendProtocolError("Invalid response data") from exc

    if getattr(rep, "status", None) == "invalid_msg_format":
        transport.logger.error("Invalid request data according to backend", cmd=cmd, rep=rep)
        raise BackendProtocolError("Invalid request data according to backend")
    elif isinstance(
        rep,
        (
            OrganizationBootstrapRepBadTimestamp,
            RealmCreateRepBadTimestamp,
            RealmUpdateRolesRepBadTimestamp,
            RealmStartReencryptionMaintenanceRepBadTimestamp,
            VlobCreateRepBadTimestamp,
            VlobUpdateRepBadTimestamp,
            RealmUpdateArchivingRepBadTimestamp,
        ),
    ):
        raise BackendOutOfBallparkError(rep)

    return rep


###  Backend authenticated cmds  ###

### Organization API ###


async def organization_stats(transport: Transport) -> OrganizationStatsRep:
    return cast(
        OrganizationStatsRep,
        await _send_cmd(transport, organization_stats_serializer, cmd="organization_stats"),
    )


async def organization_config(transport: Transport) -> OrganizationConfigRep:
    return cast(
        OrganizationConfigRep,
        await _send_cmd(transport, organization_config_serializer, cmd="organization_config"),
    )


async def archiving_config(transport: Transport) -> ArchivingConfigRep:
    return cast(
        ArchivingConfigRep,
        await _send_cmd(transport, archiving_config_serializer, cmd="archiving_config"),
    )


### Events&misc API ###


async def authenticated_ping(transport: Transport, ping: str = "") -> AuthenticatedPingRep:
    return cast(
        AuthenticatedPingRep,
        await _send_cmd(transport, authenticated_ping_serializer, cmd="ping", ping=ping),
    )


async def invited_ping(transport: Transport, ping: str = "") -> Awaitable[InvitedPingRep]:
    return cast(
        Awaitable[InvitedPingRep],
        await _send_cmd(transport, invited_ping_serializer, cmd="ping", ping=ping),
    )


async def events_subscribe(
    transport: Transport,
) -> EventsSubscribeRep:
    return cast(
        EventsSubscribeRep,
        await _send_cmd(transport, events_subscribe_serializer, cmd="events_subscribe"),
    )


async def events_listen(transport: Transport, wait: bool = True) -> EventsListenRep:
    return cast(
        EventsListenRep,
        await _send_cmd(transport, events_listen_serializer, cmd="events_listen", wait=wait),
    )


### Message API ###


async def message_get(transport: Transport, offset: int) -> MessageGetRep:
    return cast(
        MessageGetRep,
        await _send_cmd(transport, message_get_serializer, cmd="message_get", offset=offset),
    )


### Vlob API ###


async def vlob_create(
    transport: Transport,
    realm_id: RealmID,
    encryption_revision: int,
    vlob_id: VlobID,
    timestamp: DateTime,
    blob: bytes,
    sequester_blob: dict[SequesterServiceID, bytes] | None,
) -> VlobCreateRep:
    return cast(
        VlobCreateRep,
        await _send_cmd(
            transport,
            vlob_create_serializer,
            cmd="vlob_create",
            realm_id=realm_id,
            encryption_revision=encryption_revision,
            vlob_id=vlob_id,
            timestamp=timestamp,
            blob=blob,
            sequester_blob=sequester_blob,
        ),
    )


async def vlob_read(
    transport: Transport,
    encryption_revision: int,
    vlob_id: VlobID,
    version: int | None = None,
    timestamp: DateTime | None = None,
) -> VlobReadRep:
    return cast(
        VlobReadRep,
        await _send_cmd(
            transport,
            vlob_read_serializer,
            cmd="vlob_read",
            encryption_revision=encryption_revision,
            vlob_id=vlob_id,
            version=version,
            timestamp=timestamp,
        ),
    )


async def vlob_update(
    transport: Transport,
    encryption_revision: int,
    vlob_id: VlobID,
    version: int,
    timestamp: DateTime,
    blob: bytes,
    sequester_blob: dict[SequesterServiceID, bytes] | None,
) -> VlobUpdateRep:
    return cast(
        VlobUpdateRep,
        await _send_cmd(
            transport,
            vlob_update_serializer,
            cmd="vlob_update",
            encryption_revision=encryption_revision,
            vlob_id=vlob_id,
            version=version,
            timestamp=timestamp,
            blob=blob,
            sequester_blob=sequester_blob,
        ),
    )


async def vlob_poll_changes(
    transport: Transport, realm_id: RealmID, last_checkpoint: int
) -> VlobPollChangesRep:
    return cast(
        VlobPollChangesRep,
        await _send_cmd(
            transport,
            vlob_poll_changes_serializer,
            cmd="vlob_poll_changes",
            realm_id=realm_id,
            last_checkpoint=last_checkpoint,
        ),
    )


async def vlob_list_versions(transport: Transport, vlob_id: VlobID) -> VlobListVersionsRep:
    return cast(
        VlobListVersionsRep,
        await _send_cmd(
            transport,
            vlob_list_versions_serializer,
            cmd="vlob_list_versions",
            vlob_id=vlob_id,
        ),
    )


async def vlob_maintenance_get_reencryption_batch(
    transport: Transport, realm_id: RealmID, encryption_revision: int, size: int
) -> VlobMaintenanceGetReencryptionBatchRep:
    return cast(
        VlobMaintenanceGetReencryptionBatchRep,
        await _send_cmd(
            transport,
            vlob_maintenance_get_reencryption_batch_serializer,
            cmd="vlob_maintenance_get_reencryption_batch",
            realm_id=realm_id,
            encryption_revision=encryption_revision,
            size=size,
        ),
    )


async def vlob_maintenance_save_reencryption_batch(
    transport: Transport,
    realm_id: RealmID,
    encryption_revision: int,
    batch: Iterable[ReencryptionBatchEntry],
) -> VlobMaintenanceSaveReencryptionBatchRep:
    return cast(
        VlobMaintenanceSaveReencryptionBatchRep,
        await _send_cmd(
            transport,
            vlob_maintenance_save_reencryption_batch_serializer,
            cmd="vlob_maintenance_save_reencryption_batch",
            realm_id=realm_id,
            encryption_revision=encryption_revision,
            batch=list(batch),
        ),
    )


### Realm API ###


async def realm_create(transport: Transport, role_certificate: bytes) -> RealmCreateRep:
    return cast(
        RealmCreateRep,
        await _send_cmd(
            transport,
            realm_create_serializer,
            cmd="realm_create",
            role_certificate=role_certificate,
        ),
    )


async def realm_status(transport: Transport, realm_id: RealmID) -> RealmStatusRep:
    return cast(
        RealmStatusRep,
        await _send_cmd(transport, realm_status_serializer, cmd="realm_status", realm_id=realm_id),
    )


async def realm_get_role_certificates(
    transport: Transport, realm_id: RealmID
) -> RealmGetRoleCertificatesRep:
    return cast(
        RealmGetRoleCertificatesRep,
        await _send_cmd(
            transport,
            realm_get_role_certificates_serializer,
            cmd="realm_get_role_certificates",
            realm_id=realm_id,
        ),
    )


async def realm_update_roles(
    transport: Transport, role_certificate: bytes, recipient_message: bytes
) -> RealmUpdateRolesRep:
    return cast(
        RealmUpdateRolesRep,
        await _send_cmd(
            transport,
            realm_update_roles_serializer,
            cmd="realm_update_roles",
            role_certificate=role_certificate,
            recipient_message=recipient_message,
        ),
    )


async def realm_update_archiving(
    transport: Transport,
    archiving_certificate: bytes,
) -> RealmUpdateArchivingRep:
    return cast(
        RealmUpdateArchivingRep,
        await _send_cmd(
            transport,
            realm_update_archiving_serializer,
            cmd="realm_update_archiving",
            archiving_certificate=archiving_certificate,
        ),
    )


async def realm_start_reencryption_maintenance(
    transport: Transport,
    realm_id: RealmID,
    encryption_revision: int,
    timestamp: DateTime,
    per_participant_message: dict[UserID, bytes],
) -> RealmStartReencryptionMaintenanceRep:
    return cast(
        RealmStartReencryptionMaintenanceRep,
        await _send_cmd(
            transport,
            realm_start_reencryption_maintenance_serializer,
            cmd="realm_start_reencryption_maintenance",
            realm_id=realm_id,
            encryption_revision=encryption_revision,
            timestamp=timestamp,
            per_participant_message=per_participant_message,
        ),
    )


async def realm_finish_reencryption_maintenance(
    transport: Transport, realm_id: RealmID, encryption_revision: int
) -> RealmFinishReencryptionMaintenanceRep:
    return cast(
        RealmFinishReencryptionMaintenanceRep,
        await _send_cmd(
            transport,
            realm_finish_reencryption_maintenance_serializer,
            cmd="realm_finish_reencryption_maintenance",
            realm_id=realm_id,
            encryption_revision=encryption_revision,
        ),
    )


### Block API ###


async def block_create(
    transport: Transport, block_id: BlockID, realm_id: RealmID, block: bytes
) -> BlockCreateRep:
    return cast(
        BlockCreateRep,
        await _send_cmd(
            transport,
            block_create_serializer,
            cmd="block_create",
            block_id=block_id,
            realm_id=realm_id,
            block=block,
        ),
    )


async def block_read(transport: Transport, block_id: BlockID) -> BlockReadRep:
    return cast(
        BlockReadRep,
        await _send_cmd(transport, block_read_serializer, cmd="block_read", block_id=block_id),
    )


### Invite API ###


async def invite_new(
    transport: Transport,
    type: InvitationType,
    send_email: bool = False,
    claimer_email: str | None = None,
    claimer_user_id: UserID | None = None,
) -> InviteNewRep:
    return cast(
        InviteNewRep,
        await _send_cmd(
            transport,
            invite_new_serializer,
            cmd="invite_new",
            type=type,
            send_email=send_email,
            claimer_email=claimer_email,
            claimer_user_id=claimer_user_id,
        ),
    )


async def invite_list(transport: Transport) -> InviteListRep:
    return cast(
        InviteListRep,
        await _send_cmd(transport, invite_list_serializer, cmd="invite_list"),
    )


async def invite_delete(
    transport: Transport, token: InvitationToken, reason: InvitationDeletedReason
) -> InviteDeleteRep:
    return cast(
        InviteDeleteRep,
        await _send_cmd(
            transport,
            invite_delete_serializer,
            cmd="invite_delete",
            token=token,
            reason=reason,
        ),
    )


async def invite_info(transport: Transport) -> InviteInfoRep:
    return cast(
        InviteInfoRep,
        await _send_cmd(transport, invite_info_serializer, cmd="invite_info"),
    )


async def invite_1_claimer_wait_peer(
    transport: Transport,
    greeter_user_id: UserID,
    claimer_public_key: PublicKey,
) -> Invite1ClaimerWaitPeerRep:
    return cast(
        Invite1ClaimerWaitPeerRep,
        await _send_cmd(
            transport,
            invite_1_claimer_wait_peer_serializer,
            cmd="invite_1_claimer_wait_peer",
            greeter_user_id=greeter_user_id,
            claimer_public_key=claimer_public_key,
        ),
    )


async def invite_1_greeter_wait_peer(
    transport: Transport, token: InvitationToken, greeter_public_key: PublicKey
) -> Invite1GreeterWaitPeerRep:
    return cast(
        Invite1GreeterWaitPeerRep,
        await _send_cmd(
            transport,
            invite_1_greeter_wait_peer_serializer,
            cmd="invite_1_greeter_wait_peer",
            token=token,
            greeter_public_key=greeter_public_key,
        ),
    )


async def invite_2a_claimer_send_hashed_nonce(
    transport: Transport, greeter_user_id: UserID, claimer_hashed_nonce: HashDigest
) -> Invite2aClaimerSendHashedNonceRep:
    return cast(
        Invite2aClaimerSendHashedNonceRep,
        await _send_cmd(
            transport,
            invite_2a_claimer_send_hashed_nonce_serializer,
            cmd="invite_2a_claimer_send_hashed_nonce",
            greeter_user_id=greeter_user_id,
            claimer_hashed_nonce=claimer_hashed_nonce,
        ),
    )


async def invite_2a_greeter_get_hashed_nonce(
    transport: Transport, token: InvitationToken
) -> Invite2aGreeterGetHashedNonceRep:
    return cast(
        Invite2aGreeterGetHashedNonceRep,
        await _send_cmd(
            transport,
            invite_2a_greeter_get_hashed_nonce_serializer,
            cmd="invite_2a_greeter_get_hashed_nonce",
            token=token,
        ),
    )


async def invite_2b_greeter_send_nonce(
    transport: Transport, token: InvitationToken, greeter_nonce: bytes
) -> Invite2bGreeterSendNonceRep:
    return cast(
        Invite2bGreeterSendNonceRep,
        await _send_cmd(
            transport,
            invite_2b_greeter_send_nonce_serializer,
            cmd="invite_2b_greeter_send_nonce",
            token=token,
            greeter_nonce=greeter_nonce,
        ),
    )


async def invite_2b_claimer_send_nonce(
    transport: Transport, greeter_user_id: UserID, claimer_nonce: bytes
) -> Invite2bClaimerSendNonceRep:
    return cast(
        Invite2bClaimerSendNonceRep,
        await _send_cmd(
            transport,
            invite_2b_claimer_send_nonce_serializer,
            cmd="invite_2b_claimer_send_nonce",
            greeter_user_id=greeter_user_id,
            claimer_nonce=claimer_nonce,
        ),
    )


async def invite_3a_greeter_wait_peer_trust(
    transport: Transport, token: InvitationToken
) -> Invite3aGreeterWaitPeerTrustRep:
    return cast(
        Invite3aGreeterWaitPeerTrustRep,
        await _send_cmd(
            transport,
            invite_3a_greeter_wait_peer_trust_serializer,
            cmd="invite_3a_greeter_wait_peer_trust",
            token=token,
        ),
    )


async def invite_3a_claimer_signify_trust(
    transport: Transport,
    greeter_user_id: UserID,
) -> Invite3aClaimerSignifyTrustRep:
    return cast(
        Invite3aClaimerSignifyTrustRep,
        await _send_cmd(
            transport,
            invite_3a_claimer_signify_trust_serializer,
            cmd="invite_3a_claimer_signify_trust",
            greeter_user_id=greeter_user_id,
        ),
    )


async def invite_3b_claimer_wait_peer_trust(
    transport: Transport,
    greeter_user_id: UserID,
) -> Invite3bClaimerWaitPeerTrustRep:
    return cast(
        Invite3bClaimerWaitPeerTrustRep,
        await _send_cmd(
            transport,
            invite_3b_claimer_wait_peer_trust_serializer,
            cmd="invite_3b_claimer_wait_peer_trust",
            greeter_user_id=greeter_user_id,
        ),
    )


async def invite_3b_greeter_signify_trust(
    transport: Transport, token: InvitationToken
) -> Invite3bGreeterSignifyTrustRep:
    return cast(
        Invite3bGreeterSignifyTrustRep,
        await _send_cmd(
            transport,
            invite_3b_greeter_signify_trust_serializer,
            cmd="invite_3b_greeter_signify_trust",
            token=token,
        ),
    )


async def invite_4_greeter_communicate(
    transport: Transport, token: InvitationToken, payload: bytes | None
) -> Invite4GreeterCommunicateRep:
    return cast(
        Invite4GreeterCommunicateRep,
        await _send_cmd(
            transport,
            invite_4_greeter_communicate_serializer,
            cmd="invite_4_greeter_communicate",
            token=token,
            payload=payload,
        ),
    )


async def invite_4_claimer_communicate(
    transport: Transport, greeter_user_id: UserID, payload: bytes | None
) -> Invite4ClaimerCommunicateRep:
    return cast(
        Invite4ClaimerCommunicateRep,
        await _send_cmd(
            transport,
            invite_4_claimer_communicate_serializer,
            cmd="invite_4_claimer_communicate",
            greeter_user_id=greeter_user_id,
            payload=payload,
        ),
    )


### User API ###


async def user_get(transport: Transport, user_id: UserID) -> UserGetRep:
    return cast(
        UserGetRep,
        await _send_cmd(transport, user_get_serializer, cmd="user_get", user_id=user_id),
    )


async def human_find(
    transport: Transport,
    query: str | None = None,
    page: int = 1,
    per_page: int = 100,
    omit_revoked: bool = False,
    omit_non_human: bool = False,
) -> HumanFindRep:
    return cast(
        HumanFindRep,
        await _send_cmd(
            transport,
            human_find_serializer,
            cmd="human_find",
            query=query,
            page=page,
            per_page=per_page,
            omit_revoked=omit_revoked,
            omit_non_human=omit_non_human,
        ),
    )


async def user_create(
    transport: Transport,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> UserCreateRep:
    return cast(
        UserCreateRep,
        await _send_cmd(
            transport,
            user_create_serializer,
            cmd="user_create",
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        ),
    )


async def user_revoke(transport: Transport, revoked_user_certificate: bytes) -> UserRevokeRep:
    return cast(
        UserRevokeRep,
        await _send_cmd(
            transport,
            user_revoke_serializer,
            cmd="user_revoke",
            revoked_user_certificate=revoked_user_certificate,
        ),
    )


async def device_create(
    transport: Transport, device_certificate: bytes, redacted_device_certificate: bytes
) -> DeviceCreateRep:
    return cast(
        DeviceCreateRep,
        await _send_cmd(
            transport,
            device_create_serializer,
            cmd="device_create",
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        ),
    )


### Shamir API ###


async def shamir_recovery_others_list(
    transport: Transport,
) -> ShamirRecoveryOthersListRep:
    return cast(
        ShamirRecoveryOthersListRep,
        await _send_cmd(
            transport,
            shamir_recovery_others_list_serializer,
            cmd="shamir_recovery_others_list",
        ),
    )


async def shamir_recovery_self_info(
    transport: Transport,
) -> ShamirRecoverySelfInfoRep:
    return cast(
        ShamirRecoverySelfInfoRep,
        await _send_cmd(
            transport,
            shamir_recovery_self_info_serializer,
            cmd="shamir_recovery_self_info",
        ),
    )


async def shamir_recovery_setup(
    transport: Transport,
    setup: ShamirRecoverySetup | None,
) -> ShamirRecoverySetupRep:
    return cast(
        ShamirRecoverySetupRep,
        await _send_cmd(
            transport,
            shamir_recovery_setup_serializer,
            cmd="shamir_recovery_setup",
            setup=setup,
        ),
    )


async def invite_shamir_recovery_reveal(
    transport: Transport,
    reveal_token: ShamirRevealToken,
) -> InviteShamirRecoveryRevealRep:
    return cast(
        InviteShamirRecoveryRevealRep,
        await _send_cmd(
            transport,
            invite_shamir_recovery_reveal_serializer,
            cmd="invite_shamir_recovery_reveal",
            reveal_token=reveal_token,
        ),
    )


###  Backend anonymous cmds  ###


async def organization_bootstrap(
    transport: Transport,
    bootstrap_token: str,
    root_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> OrganizationBootstrapRep:
    return cast(
        OrganizationBootstrapRep,
        await _send_cmd(
            transport,
            organization_bootstrap_serializer,
            cmd="organization_bootstrap",
            bootstrap_token=bootstrap_token,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        ),
    )


### PKI enrollment API ###


async def pki_enrollment_list(transport: Transport) -> dict[str, object]:
    return cast(
        dict[str, object],
        await _send_cmd(transport, pki_enrollment_list_serializer, cmd="pki_enrollment_list"),
    )


async def pki_enrollment_reject(
    transport: Transport, enrollment_id: EnrollmentID
) -> dict[str, object]:
    return cast(
        dict[str, object],
        await _send_cmd(
            transport,
            pki_enrollment_reject_serializer,
            cmd="pki_enrollment_reject",
            enrollment_id=enrollment_id,
        ),
    )


async def pki_enrollment_accept(
    transport: Transport,
    enrollment_id: EnrollmentID,
    accepter_der_x509_certificate: bytes,
    accept_payload_signature: bytes,
    accept_payload: bytes,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> dict[str, object]:
    return cast(
        dict[str, object],
        await _send_cmd(
            transport,
            pki_enrollment_accept_serializer,
            cmd="pki_enrollment_accept",
            enrollment_id=enrollment_id,
            accepter_der_x509_certificate=accepter_der_x509_certificate,
            accept_payload_signature=accept_payload_signature,
            accept_payload=accept_payload,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        ),
    )
