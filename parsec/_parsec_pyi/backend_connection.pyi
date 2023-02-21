# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from typing import Iterable

from parsec._parsec_pyi.addrs import BackendInvitationAddr, BackendOrganizationAddr
from parsec._parsec_pyi.crypto import HashDigest, PublicKey, SigningKey
from parsec._parsec_pyi.enumerate import InvitationDeletedReason, InvitationType
from parsec._parsec_pyi.ids import (
    BlockID,
    DeviceID,
    EnrollmentID,
    InvitationToken,
    RealmID,
    SequesterServiceID,
    UserID,
    VlobID,
)
from parsec._parsec_pyi.protocol import (
    AuthenticatedPingRep,
    BlockCreateRep,
    BlockReadRep,
    DeviceCreateRep,
    EventsListenRep,
    EventsSubscribeRep,
    HumanFindRep,
    Invite1GreeterWaitPeerRep,
    Invite2aGreeterGetHashedNonceRep,
    Invite2bGreeterSendNonceRep,
    Invite3aGreeterWaitPeerTrustRep,
    Invite3bGreeterSignifyTrustRep,
    Invite4GreeterCommunicateRep,
    InviteDeleteRep,
    InviteListRep,
    InviteNewRep,
    MessageGetRep,
    OrganizationConfigRep,
    OrganizationStatsRep,
    PkiEnrollmentAcceptRep,
    PkiEnrollmentListRep,
    PkiEnrollmentRejectRep,
    RealmCreateRep,
    RealmFinishReencryptionMaintenanceRep,
    RealmGetRoleCertificatesRep,
    RealmStartReencryptionMaintenanceRep,
    RealmStatsRep,
    RealmStatusRep,
    RealmUpdateRolesRep,
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
)
from parsec._parsec_pyi.protocol.invite import (
    Invite1ClaimerWaitPeerRep,
    Invite2aClaimerSendHashedNonceRep,
    Invite2bClaimerSendNonceRep,
    Invite3aClaimerSignifyTrustRep,
    Invite3bClaimerWaitPeerTrustRep,
    Invite4ClaimerCommunicateRep,
    InviteInfoRep,
)
from parsec._parsec_pyi.protocol.ping import InvitedPingRep
from parsec._parsec_pyi.protocol.vlob import ReencryptionBatchEntry
from parsec._parsec_pyi.time import DateTime

class AuthenticatedCmds:
    def __init__(
        self, addr: BackendOrganizationAddr, device_id: DeviceID, signing_key: SigningKey
    ) -> None: ...
    @property
    def addr(self) -> BackendOrganizationAddr: ...
    async def block_create(
        self,
        block_id: BlockID,
        realm_id: RealmID,
        block: bytes,
    ) -> BlockCreateRep: ...
    async def block_read(self, block_id: BlockID) -> BlockReadRep: ...
    async def device_create(
        self, device_certificate: bytes, redacted_device_certificate: bytes
    ) -> DeviceCreateRep: ...
    async def events_listen(self, wait: bool) -> EventsListenRep: ...
    async def events_subscribe(self) -> EventsSubscribeRep: ...
    async def human_find(
        self,
        query: str | None,
        page: int,
        per_page: int,
        omit_revoked: bool,
        omit_non_human: bool,
    ) -> HumanFindRep: ...
    async def invite_1_greeter_wait_peer(
        self,
        token: InvitationToken,
        greeter_public_key: PublicKey,
    ) -> Invite1GreeterWaitPeerRep: ...
    async def invite_2a_greeter_get_hashed_nonce(
        self, token: InvitationToken
    ) -> Invite2aGreeterGetHashedNonceRep: ...
    async def invite_2b_greeter_send_nonce(
        self,
        token: InvitationToken,
        greeter_nonce: bytes,
    ) -> Invite2bGreeterSendNonceRep: ...
    async def invite_3a_greeter_wait_peer_trust(
        self, token: InvitationToken
    ) -> Invite3aGreeterWaitPeerTrustRep: ...
    async def invite_3b_greeter_signify_trust(
        self, token: InvitationToken
    ) -> Invite3bGreeterSignifyTrustRep: ...
    async def invite_4_greeter_communicate(
        self,
        token: InvitationToken,
        payload: bytes,
    ) -> Invite4GreeterCommunicateRep: ...
    async def invite_delete(
        self,
        token: InvitationToken,
        reason: InvitationDeletedReason,
    ) -> InviteDeleteRep: ...
    async def invite_list(self) -> InviteListRep: ...
    async def invite_new(
        self,
        type: InvitationType,
        send_email: bool = False,
        claimer_email: str | None = None,
    ) -> InviteNewRep: ...
    async def message_get(self, offset: int) -> MessageGetRep: ...
    async def organization_config(self) -> OrganizationConfigRep: ...
    async def organization_stats(self) -> OrganizationStatsRep: ...
    async def ping(self, ping: str) -> AuthenticatedPingRep: ...
    async def pki_enrollment_accept(
        self,
        enrollment_id: EnrollmentID,
        accepter_der_x509_certificate: bytes,
        accept_payload_signature: bytes,
        accept_payload: bytes,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> PkiEnrollmentAcceptRep: ...
    async def pki_enrollment_list(self) -> PkiEnrollmentListRep: ...
    async def pki_enrollment_reject(
        self, enrollment_id: EnrollmentID
    ) -> PkiEnrollmentRejectRep: ...
    async def realm_create(self, role_certificate: bytes) -> RealmCreateRep: ...
    async def realm_finish_reencryption_maintenance(
        self,
        realm_id: RealmID,
        encryption_revision: int,
    ) -> RealmFinishReencryptionMaintenanceRep: ...
    async def realm_get_role_certificates(
        self, realm_id: RealmID
    ) -> RealmGetRoleCertificatesRep: ...
    async def realm_start_reencryption_maintenance(
        self,
        realm_id: RealmID,
        encryption_revision: int,
        timestamp: DateTime,
        per_participant_message: dict[UserID, bytes],
    ) -> RealmStartReencryptionMaintenanceRep: ...
    async def realm_stats(self, realm_id: RealmID) -> RealmStatsRep: ...
    async def realm_status(self, realm_id: RealmID) -> RealmStatusRep: ...
    async def realm_update_roles(
        self,
        role_certificate: bytes,
        recipient_message: bytes | None,
    ) -> RealmUpdateRolesRep: ...
    async def user_create(
        self,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> UserCreateRep: ...
    async def user_get(self, user_id: UserID) -> UserGetRep: ...
    async def user_revoke(self, revoked_user_certificate: bytes) -> UserRevokeRep: ...
    async def vlob_create(
        self,
        realm_id: RealmID,
        encryption_revision: int,
        vlob_id: VlobID,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: dict[SequesterServiceID, bytes] | None,
    ) -> VlobCreateRep: ...
    async def vlob_list_versions(self, vlob_id: VlobID) -> VlobListVersionsRep: ...
    async def vlob_maintenance_get_reencryption_batch(
        self,
        realm_id: RealmID,
        encryption_revision: int,
        size: int,
    ) -> VlobMaintenanceGetReencryptionBatchRep: ...
    async def vlob_maintenance_save_reencryption_batch(
        self,
        realm_id: RealmID,
        encryption_revision: int,
        batch: Iterable[ReencryptionBatchEntry],
    ) -> VlobMaintenanceSaveReencryptionBatchRep: ...
    async def vlob_poll_changes(
        self, realm_id: RealmID, last_checkpoint: int
    ) -> VlobPollChangesRep: ...
    async def vlob_read(
        self,
        encryption_revision: int,
        vlob_id: VlobID,
        version: int | None = None,
        timestamp: DateTime | None = None,
    ) -> VlobReadRep: ...
    async def vlob_update(
        self,
        encryption_revision: int,
        vlob_id: VlobID,
        version: int,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: dict[SequesterServiceID, bytes] | None,
    ) -> VlobUpdateRep: ...

class InvitedCmds:
    def __init__(self, addr: BackendInvitationAddr) -> None: ...
    @property
    def addr(self) -> BackendInvitationAddr: ...
    async def invite_1_claimer_wait_peer(
        self, claimer_public_key: PublicKey
    ) -> Invite1ClaimerWaitPeerRep: ...
    async def invite_2a_claimer_send_hashed_nonce(
        self, claimer_hashed_nonce: HashDigest
    ) -> Invite2aClaimerSendHashedNonceRep: ...
    async def invite_2b_claimer_send_nonce(
        self, claimer_nonce: bytes
    ) -> Invite2bClaimerSendNonceRep: ...
    async def invite_3a_claimer_signify_trust(self) -> Invite3aClaimerSignifyTrustRep: ...
    async def invite_3b_claimer_wait_peer_trust(self) -> Invite3bClaimerWaitPeerTrustRep: ...
    async def invite_4_claimer_communicate(
        self, payload: bytes
    ) -> Invite4ClaimerCommunicateRep: ...
    async def invite_info(self) -> InviteInfoRep: ...
    async def ping(self, ping: str = "") -> InvitedPingRep: ...
