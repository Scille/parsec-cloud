# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Tuple, List, Dict, Optional
from uuid import UUID

from parsec._parsec import (
    AuthenticatedPingRep,
    AuthenticatedPingRepUnknownStatus,
    BlockCreateRep,
    BlockCreateRepUnknownStatus,
    BlockReadRep,
    BlockReadRepUnknownStatus,
    DateTime,
    DeviceCreateRep,
    DeviceCreateRepUnknownStatus,
    EventsListenRep,
    EventsListenRepUnknownStatus,
    EventsSubscribeRep,
    EventsSubscribeRepUnknownStatus,
    HumanFindRep,
    HumanFindRepUnknownStatus,
    Invite1ClaimerWaitPeerRep,
    Invite1ClaimerWaitPeerRepUnknownStatus,
    Invite1GreeterWaitPeerRep,
    Invite1GreeterWaitPeerRepUnknownStatus,
    Invite2aClaimerSendHashedNonceRep,
    Invite2aClaimerSendHashedNonceRepUnknownStatus,
    Invite2aGreeterGetHashedNonceRep,
    Invite2aGreeterGetHashedNonceRepUnknownStatus,
    Invite2bClaimerSendNonceRep,
    Invite2bClaimerSendNonceRepUnknownStatus,
    Invite2bGreeterSendNonceRep,
    Invite2bGreeterSendNonceRepUnknownStatus,
    Invite3aClaimerSignifyTrustRep,
    Invite3aClaimerSignifyTrustRepUnknownStatus,
    Invite3aGreeterWaitPeerTrustRep,
    Invite3aGreeterWaitPeerTrustRepUnknownStatus,
    Invite3bClaimerWaitPeerTrustRep,
    Invite3bClaimerWaitPeerTrustRepUnknownStatus,
    Invite3bGreeterSignifyTrustRep,
    Invite3bGreeterSignifyTrustRepUnknownStatus,
    Invite4ClaimerCommunicateRep,
    Invite4ClaimerCommunicateRepUnknownStatus,
    Invite4GreeterCommunicateRep,
    Invite4GreeterCommunicateRepUnknownStatus,
    InviteDeleteRep,
    InviteDeleteRepUnknownStatus,
    InvitedPingRep,
    InvitedPingRepUnknownStatus,
    InviteInfoRep,
    InviteInfoRepUnknownStatus,
    InviteListRep,
    InviteListRepUnknownStatus,
    InviteNewRep,
    InviteNewRepUnknownStatus,
    MessageGetRep,
    MessageGetRepUnknownStatus,
    OrganizationConfigRep,
    OrganizationConfigRepUnknownStatus,
    OrganizationStatsRep,
    OrganizationStatsRepUnknownStatus,
    RealmCreateRep,
    RealmCreateRepBadTimestamp,
    RealmCreateRepUnknownStatus,
    RealmFinishReencryptionMaintenanceRep,
    RealmFinishReencryptionMaintenanceRepUnknownStatus,
    RealmGetRoleCertificatesRep,
    RealmGetRoleCertificatesRepUnknownStatus,
    RealmStartReencryptionMaintenanceRep,
    RealmStartReencryptionMaintenanceRepBadTimestamp,
    RealmStartReencryptionMaintenanceRepUnknownStatus,
    RealmStatsRep,
    RealmStatsRepUnknownStatus,
    RealmStatusRep,
    RealmStatusRepUnknownStatus,
    RealmUpdateRolesRep,
    RealmUpdateRolesRepBadTimestamp,
    RealmUpdateRolesRepUnknownStatus,
    ReencryptionBatchEntry,
    UserCreateRep,
    UserCreateRepUnknownStatus,
    UserGetRep,
    UserGetRepUnknownStatus,
    UserRevokeRep,
    UserRevokeRepUnknownStatus,
    VlobCreateRep,
    VlobCreateRepBadTimestamp,
    VlobCreateRepUnknownStatus,
    VlobListVersionsRep,
    VlobListVersionsRepUnknownStatus,
    VlobMaintenanceGetReencryptionBatchRep,
    VlobMaintenanceGetReencryptionBatchRepUnknownStatus,
    VlobMaintenanceSaveReencryptionBatchRep,
    VlobMaintenanceSaveReencryptionBatchRepUnknownStatus,
    VlobPollChangesRep,
    VlobPollChangesRepUnknownStatus,
    VlobReadRep,
    VlobReadRepUnknownStatus,
    VlobUpdateRep,
    VlobUpdateRepBadTimestamp,
    VlobUpdateRepUnknownStatus,
)
from parsec.crypto import VerifyKey, PublicKey
from parsec.api.transport import Transport, TransportError
from parsec.api.protocol import (
    OrganizationID,
    UserID,
    RealmID,
    VlobID,
    BlockID,
    SequesterServiceID,
    InvitationToken,
    ProtocolError,
    InvitationType,
    InvitationDeletedReason,
    invite_new_serializer,
    invite_delete_serializer,
    invite_list_serializer,
    invite_info_serializer,
    invite_1_claimer_wait_peer_serializer,
    invite_1_greeter_wait_peer_serializer,
    invite_2a_claimer_send_hashed_nonce_serializer,
    invite_2a_greeter_get_hashed_nonce_serializer,
    invite_2b_greeter_send_nonce_serializer,
    invite_2b_claimer_send_nonce_serializer,
    invite_3a_greeter_wait_peer_trust_serializer,
    invite_3b_claimer_wait_peer_trust_serializer,
    invite_3b_greeter_signify_trust_serializer,
    invite_3a_claimer_signify_trust_serializer,
    invite_4_greeter_communicate_serializer,
    invite_4_claimer_communicate_serializer,
    authenticated_ping_serializer,
    invited_ping_serializer,
    organization_stats_serializer,
    organization_config_serializer,
    apiv1_organization_bootstrap_serializer,
    events_subscribe_serializer,
    events_listen_serializer,
    message_get_serializer,
    vlob_read_serializer,
    vlob_create_serializer,
    vlob_update_serializer,
    vlob_poll_changes_serializer,
    vlob_list_versions_serializer,
    vlob_maintenance_get_reencryption_batch_serializer,
    vlob_maintenance_save_reencryption_batch_serializer,
    realm_create_serializer,
    realm_status_serializer,
    realm_get_role_certificates_serializer,
    realm_update_roles_serializer,
    realm_start_reencryption_maintenance_serializer,
    realm_finish_reencryption_maintenance_serializer,
    block_create_serializer,
    block_read_serializer,
    user_get_serializer,
    human_find_serializer,
    user_create_serializer,
    user_revoke_serializer,
    device_create_serializer,
    pki_enrollment_list_serializer,
    pki_enrollment_reject_serializer,
    pki_enrollment_accept_serializer,
)
from parsec.core.backend_connection.exceptions import (
    BackendNotAvailable,
    BackendOutOfBallparkError,
    BackendProtocolError,
)


async def _send_cmd(transport: Transport, serializer, **req) -> dict:
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
        rep = serializer.rep_loads(raw_rep)

    except ProtocolError as exc:
        transport.logger.exception("Invalid response data", cmd=cmd, error=exc)
        raise BackendProtocolError("Invalid response data") from exc

    # Legacy
    if isinstance(rep, dict):
        if rep["status"] == "invalid_msg_format":
            transport.logger.error("Invalid request data according to backend", cmd=cmd, rep=rep)
            raise BackendProtocolError("Invalid request data according to backend")

        if rep["status"] == "bad_timestamp":
            raise BackendOutOfBallparkError(rep)

        # Backward compatibility with older backends (<= v2.3)
        if rep["status"] == "invalid_certification" and "timestamp" in rep["reason"]:
            raise BackendOutOfBallparkError(rep)

    # New shinnny stuff
    elif isinstance(
        rep,
        (
            AuthenticatedPingRep,
            BlockCreateRep,
            BlockReadRep,
            EventsListenRep,
            EventsSubscribeRep,
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
            RealmCreateRep,
            RealmFinishReencryptionMaintenanceRep,
            RealmGetRoleCertificatesRep,
            RealmStartReencryptionMaintenanceRep,
            RealmStatsRep,
            RealmStatusRep,
            RealmUpdateRolesRep,
            RealmFinishReencryptionMaintenanceRep,
            UserGetRep,
            UserCreateRep,
            UserRevokeRep,
            DeviceCreateRep,
            HumanFindRep,
            VlobCreateRep,
            VlobListVersionsRep,
            VlobMaintenanceGetReencryptionBatchRep,
            VlobMaintenanceSaveReencryptionBatchRep,
            VlobPollChangesRep,
            VlobReadRep,
            VlobUpdateRep,
        ),
    ):
        if isinstance(
            rep,
            (
                AuthenticatedPingRepUnknownStatus,
                BlockCreateRepUnknownStatus,
                BlockReadRepUnknownStatus,
                DeviceCreateRepUnknownStatus,
                EventsListenRepUnknownStatus,
                EventsSubscribeRepUnknownStatus,
                HumanFindRepUnknownStatus,
                Invite1ClaimerWaitPeerRepUnknownStatus,
                Invite1GreeterWaitPeerRepUnknownStatus,
                Invite2aClaimerSendHashedNonceRepUnknownStatus,
                Invite2aGreeterGetHashedNonceRepUnknownStatus,
                Invite2bClaimerSendNonceRepUnknownStatus,
                Invite2bGreeterSendNonceRepUnknownStatus,
                Invite3aClaimerSignifyTrustRepUnknownStatus,
                Invite3aGreeterWaitPeerTrustRepUnknownStatus,
                Invite3bClaimerWaitPeerTrustRepUnknownStatus,
                Invite3bGreeterSignifyTrustRepUnknownStatus,
                Invite4ClaimerCommunicateRepUnknownStatus,
                Invite4GreeterCommunicateRepUnknownStatus,
                InviteDeleteRepUnknownStatus,
                InvitedPingRepUnknownStatus,
                InviteInfoRepUnknownStatus,
                InviteListRepUnknownStatus,
                InviteNewRepUnknownStatus,
                MessageGetRepUnknownStatus,
                OrganizationConfigRepUnknownStatus,
                OrganizationStatsRepUnknownStatus,
                RealmCreateRepUnknownStatus,
                RealmFinishReencryptionMaintenanceRepUnknownStatus,
                RealmGetRoleCertificatesRepUnknownStatus,
                RealmStartReencryptionMaintenanceRepUnknownStatus,
                RealmStatsRepUnknownStatus,
                RealmStatusRepUnknownStatus,
                RealmUpdateRolesRepUnknownStatus,
                UserCreateRepUnknownStatus,
                UserGetRepUnknownStatus,
                UserRevokeRepUnknownStatus,
                VlobCreateRepUnknownStatus,
                VlobListVersionsRepUnknownStatus,
                VlobMaintenanceGetReencryptionBatchRepUnknownStatus,
                VlobMaintenanceSaveReencryptionBatchRepUnknownStatus,
                VlobPollChangesRepUnknownStatus,
                VlobReadRepUnknownStatus,
                VlobUpdateRepUnknownStatus,
            ),
        ):
            if rep.status == "invalid_msg_format":
                transport.logger.error(
                    "Invalid request data according to backend", cmd=cmd, rep=rep
                )
                raise BackendProtocolError("Invalid request data according to backend")

            # Backward compatibility with older backends (<= v2.3)
            if rep.status == "invalid_certification" and "timestamp" in rep.reason:
                raise BackendOutOfBallparkError(rep)
        elif isinstance(
            rep,
            (
                RealmCreateRepBadTimestamp,
                RealmUpdateRolesRepBadTimestamp,
                RealmStartReencryptionMaintenanceRepBadTimestamp,
                VlobCreateRepBadTimestamp,
                VlobUpdateRepBadTimestamp,
            ),
        ):
            raise BackendOutOfBallparkError(rep)

    else:
        raise ProtocolError("Your protocol is not handled")

    return rep


###  Backend authenticated cmds  ###

### Organization API ###


async def organization_stats(transport: Transport) -> dict:
    return await _send_cmd(transport, organization_stats_serializer, cmd="organization_stats")


async def organization_config(transport: Transport) -> dict:
    return await _send_cmd(transport, organization_config_serializer, cmd="organization_config")


### Events&misc API ###


async def authenticated_ping(transport: Transport, ping: str = "") -> dict:
    return await _send_cmd(transport, authenticated_ping_serializer, cmd="ping", ping=ping)


async def invited_ping(transport: Transport, ping: str = "") -> dict:
    return await _send_cmd(transport, invited_ping_serializer, cmd="ping", ping=ping)


async def events_subscribe(
    transport: Transport,
) -> dict:
    return await _send_cmd(transport, events_subscribe_serializer, cmd="events_subscribe")


async def events_listen(transport: Transport, wait: bool = True) -> dict:
    return await _send_cmd(transport, events_listen_serializer, cmd="events_listen", wait=wait)


### Message API ###


async def message_get(transport: Transport, offset: int) -> dict:
    return await _send_cmd(transport, message_get_serializer, cmd="message_get", offset=offset)


### Vlob API ###


async def vlob_create(
    transport: Transport,
    realm_id: RealmID,
    encryption_revision: int,
    vlob_id: VlobID,
    timestamp: DateTime,
    blob: bytes,
    sequester_blob: Optional[Dict[SequesterServiceID, bytes]],
) -> dict:
    return await _send_cmd(
        transport,
        vlob_create_serializer,
        cmd="vlob_create",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
        vlob_id=vlob_id,
        timestamp=timestamp,
        blob=blob,
        sequester_blob=sequester_blob,
    )


async def vlob_read(
    transport: Transport,
    encryption_revision: int,
    vlob_id: VlobID,
    version: int = None,
    timestamp: DateTime = None,
) -> dict:
    return await _send_cmd(
        transport,
        vlob_read_serializer,
        cmd="vlob_read",
        encryption_revision=encryption_revision,
        vlob_id=vlob_id,
        version=version,
        timestamp=timestamp,
    )


async def vlob_update(
    transport: Transport,
    encryption_revision: int,
    vlob_id: VlobID,
    version: int,
    timestamp: DateTime,
    blob: bytes,
    sequester_blob: Optional[Dict[SequesterServiceID, bytes]],
) -> dict:
    return await _send_cmd(
        transport,
        vlob_update_serializer,
        cmd="vlob_update",
        encryption_revision=encryption_revision,
        vlob_id=vlob_id,
        version=version,
        timestamp=timestamp,
        blob=blob,
        sequester_blob=sequester_blob,
    )


async def vlob_poll_changes(transport: Transport, realm_id: RealmID, last_checkpoint: int) -> dict:
    return await _send_cmd(
        transport,
        vlob_poll_changes_serializer,
        cmd="vlob_poll_changes",
        realm_id=realm_id,
        last_checkpoint=last_checkpoint,
    )


async def vlob_list_versions(transport: Transport, vlob_id: VlobID) -> dict:
    return await _send_cmd(
        transport,
        vlob_list_versions_serializer,
        cmd="vlob_list_versions",
        vlob_id=vlob_id,
    )


async def vlob_maintenance_get_reencryption_batch(
    transport: Transport, realm_id: RealmID, encryption_revision: int, size: int
) -> dict:
    return await _send_cmd(
        transport,
        vlob_maintenance_get_reencryption_batch_serializer,
        cmd="vlob_maintenance_get_reencryption_batch",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
        size=size,
    )


async def vlob_maintenance_save_reencryption_batch(
    transport: Transport,
    realm_id: RealmID,
    encryption_revision: int,
    batch: List[Tuple[VlobID, int, bytes]],
) -> dict:
    return await _send_cmd(
        transport,
        vlob_maintenance_save_reencryption_batch_serializer,
        cmd="vlob_maintenance_save_reencryption_batch",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
        batch=[ReencryptionBatchEntry(vlob_id=x[0], version=x[1], blob=x[2]) for x in batch],
    )


### Realm API ###


async def realm_create(transport: Transport, role_certificate: bytes) -> dict:
    return await _send_cmd(
        transport,
        realm_create_serializer,
        cmd="realm_create",
        role_certificate=role_certificate,
    )


async def realm_status(transport: Transport, realm_id: RealmID) -> dict:
    return await _send_cmd(
        transport, realm_status_serializer, cmd="realm_status", realm_id=realm_id
    )


async def realm_get_role_certificates(transport: Transport, realm_id: RealmID) -> dict:
    return await _send_cmd(
        transport,
        realm_get_role_certificates_serializer,
        cmd="realm_get_role_certificates",
        realm_id=realm_id,
    )


async def realm_update_roles(
    transport: Transport, role_certificate: bytes, recipient_message: bytes
) -> dict:
    return await _send_cmd(
        transport,
        realm_update_roles_serializer,
        cmd="realm_update_roles",
        role_certificate=role_certificate,
        recipient_message=recipient_message,
    )


async def realm_start_reencryption_maintenance(
    transport: Transport,
    realm_id: RealmID,
    encryption_revision: int,
    timestamp: DateTime,
    per_participant_message: Dict[UserID, bytes],
) -> dict:
    return await _send_cmd(
        transport,
        realm_start_reencryption_maintenance_serializer,
        cmd="realm_start_reencryption_maintenance",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
        timestamp=timestamp,
        per_participant_message=per_participant_message,
    )


async def realm_finish_reencryption_maintenance(
    transport: Transport, realm_id: RealmID, encryption_revision: int
) -> dict:
    return await _send_cmd(
        transport,
        realm_finish_reencryption_maintenance_serializer,
        cmd="realm_finish_reencryption_maintenance",
        realm_id=realm_id,
        encryption_revision=encryption_revision,
    )


### Block API ###


async def block_create(
    transport: Transport, block_id: BlockID, realm_id: RealmID, block: bytes
) -> dict:
    return await _send_cmd(
        transport,
        block_create_serializer,
        cmd="block_create",
        block_id=block_id,
        realm_id=realm_id,
        block=block,
    )


async def block_read(transport: Transport, block_id: BlockID) -> dict:
    return await _send_cmd(transport, block_read_serializer, cmd="block_read", block_id=block_id)


### Invite API ###


async def invite_new(
    transport: Transport,
    type: InvitationType,
    send_email: bool = False,
    claimer_email: str = None,
):
    return await _send_cmd(
        transport,
        invite_new_serializer,
        cmd="invite_new",
        type=type,
        send_email=send_email,
        claimer_email=claimer_email,
    )


async def invite_list(transport: Transport):
    return await _send_cmd(transport, invite_list_serializer, cmd="invite_list")


async def invite_delete(
    transport: Transport, token: InvitationToken, reason: InvitationDeletedReason
):
    return await _send_cmd(
        transport,
        invite_delete_serializer,
        cmd="invite_delete",
        token=token,
        reason=reason,
    )


async def invite_info(transport: Transport):
    return await _send_cmd(transport, invite_info_serializer, cmd="invite_info")


async def invite_1_claimer_wait_peer(transport: Transport, claimer_public_key: PublicKey):
    return await _send_cmd(
        transport,
        invite_1_claimer_wait_peer_serializer,
        cmd="invite_1_claimer_wait_peer",
        claimer_public_key=claimer_public_key,
    )


async def invite_1_greeter_wait_peer(
    transport: Transport, token: InvitationToken, greeter_public_key: PublicKey
):
    return await _send_cmd(
        transport,
        invite_1_greeter_wait_peer_serializer,
        cmd="invite_1_greeter_wait_peer",
        token=token,
        greeter_public_key=greeter_public_key,
    )


async def invite_2a_claimer_send_hashed_nonce(transport: Transport, claimer_hashed_nonce: bytes):
    return await _send_cmd(
        transport,
        invite_2a_claimer_send_hashed_nonce_serializer,
        cmd="invite_2a_claimer_send_hashed_nonce",
        claimer_hashed_nonce=claimer_hashed_nonce,
    )


async def invite_2a_greeter_get_hashed_nonce(transport: Transport, token: InvitationToken):
    return await _send_cmd(
        transport,
        invite_2a_greeter_get_hashed_nonce_serializer,
        cmd="invite_2a_greeter_get_hashed_nonce",
        token=token,
    )


async def invite_2b_greeter_send_nonce(
    transport: Transport, token: InvitationToken, greeter_nonce: bytes
):
    return await _send_cmd(
        transport,
        invite_2b_greeter_send_nonce_serializer,
        cmd="invite_2b_greeter_send_nonce",
        token=token,
        greeter_nonce=greeter_nonce,
    )


async def invite_2b_claimer_send_nonce(transport: Transport, claimer_nonce: bytes):
    return await _send_cmd(
        transport,
        invite_2b_claimer_send_nonce_serializer,
        cmd="invite_2b_claimer_send_nonce",
        claimer_nonce=claimer_nonce,
    )


async def invite_3a_greeter_wait_peer_trust(transport: Transport, token: InvitationToken):
    return await _send_cmd(
        transport,
        invite_3a_greeter_wait_peer_trust_serializer,
        cmd="invite_3a_greeter_wait_peer_trust",
        token=token,
    )


async def invite_3a_claimer_signify_trust(transport: Transport):
    return await _send_cmd(
        transport,
        invite_3a_claimer_signify_trust_serializer,
        cmd="invite_3a_claimer_signify_trust",
    )


async def invite_3b_claimer_wait_peer_trust(transport: Transport):
    return await _send_cmd(
        transport,
        invite_3b_claimer_wait_peer_trust_serializer,
        cmd="invite_3b_claimer_wait_peer_trust",
    )


async def invite_3b_greeter_signify_trust(transport: Transport, token: InvitationToken):
    return await _send_cmd(
        transport,
        invite_3b_greeter_signify_trust_serializer,
        cmd="invite_3b_greeter_signify_trust",
        token=token,
    )


async def invite_4_greeter_communicate(
    transport: Transport, token: InvitationToken, payload: Optional[bytes]
):
    return await _send_cmd(
        transport,
        invite_4_greeter_communicate_serializer,
        cmd="invite_4_greeter_communicate",
        token=token,
        payload=payload,
    )


async def invite_4_claimer_communicate(transport: Transport, payload: Optional[bytes]):
    return await _send_cmd(
        transport,
        invite_4_claimer_communicate_serializer,
        cmd="invite_4_claimer_communicate",
        payload=payload,
    )


### User API ###


async def user_get(transport: Transport, user_id: UserID) -> dict:
    return await _send_cmd(transport, user_get_serializer, cmd="user_get", user_id=user_id)


async def human_find(
    transport: Transport,
    query: str = None,
    page: int = 1,
    per_page: int = 100,
    omit_revoked: bool = False,
    omit_non_human: bool = False,
) -> dict:
    return await _send_cmd(
        transport,
        human_find_serializer,
        cmd="human_find",
        query=query,
        page=page,
        per_page=per_page,
        omit_revoked=omit_revoked,
        omit_non_human=omit_non_human,
    )


async def user_create(
    transport: Transport,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> dict:
    return await _send_cmd(
        transport,
        user_create_serializer,
        cmd="user_create",
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )


async def user_revoke(transport: Transport, revoked_user_certificate: bytes) -> dict:
    return await _send_cmd(
        transport,
        user_revoke_serializer,
        cmd="user_revoke",
        revoked_user_certificate=revoked_user_certificate,
    )


async def device_create(
    transport: Transport, device_certificate: bytes, redacted_device_certificate: bytes
) -> dict:
    return await _send_cmd(
        transport,
        device_create_serializer,
        cmd="device_create",
        device_certificate=device_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )


###  Backend anonymous cmds  ###


async def organization_bootstrap(
    transport: Transport,
    organization_id: OrganizationID,
    bootstrap_token: str,
    root_verify_key: VerifyKey,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> dict:
    return await _send_cmd(
        transport,
        apiv1_organization_bootstrap_serializer,
        cmd="organization_bootstrap",
        organization_id=organization_id,
        bootstrap_token=bootstrap_token,
        root_verify_key=root_verify_key,
        user_certificate=user_certificate,
        device_certificate=device_certificate,
        redacted_user_certificate=redacted_user_certificate,
        redacted_device_certificate=redacted_device_certificate,
    )


### PKI enrollment API ###


async def pki_enrollment_list(transport: Transport) -> dict:
    return await _send_cmd(transport, pki_enrollment_list_serializer, cmd="pki_enrollment_list")


async def pki_enrollment_reject(transport: Transport, enrollment_id: UUID) -> dict:
    return await _send_cmd(
        transport,
        pki_enrollment_reject_serializer,
        cmd="pki_enrollment_reject",
        enrollment_id=enrollment_id,
    )


async def pki_enrollment_accept(
    transport: Transport,
    enrollment_id: UUID,
    accepter_der_x509_certificate: bytes,
    accept_payload_signature: bytes,
    accept_payload: bytes,
    user_certificate: bytes,
    device_certificate: bytes,
    redacted_user_certificate: bytes,
    redacted_device_certificate: bytes,
) -> dict:
    return await _send_cmd(
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
    )
