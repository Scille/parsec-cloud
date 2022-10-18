# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Awaitable, List, Optional, Protocol, Any, Tuple
from uuid import UUID

from parsec._parsec import (
    AuthenticatedPingRep,
    BlockCreateRep,
    BlockID,
    BlockReadRep,
    DateTime,
    DeviceCreateRep,
    EventsListenRep,
    EventsSubscribeRep,
    HumanFindRep,
    InvitationDeletedReason,
    InvitationToken,
    InvitationType,
    Invite1ClaimerWaitPeerRep,
    Invite1GreeterWaitPeerRep,
    Invite2aClaimerSendHashedNonceRep,
    Invite2aGreeterGetHashedNonceRep,
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
    MessageGetRep,
    OrganizationConfigRep,
    OrganizationID,
    OrganizationStatsRep,
    PublicKey,
    RealmCreateRep,
    RealmFinishReencryptionMaintenanceRep,
    RealmGetRoleCertificatesRep,
    RealmID,
    RealmStartReencryptionMaintenanceRep,
    RealmStatusRep,
    RealmUpdateRolesRep,
    SequesterServiceID,
    UserCreateRep,
    UserGetRep,
    UserID,
    UserRevokeRep,
    VerifyKey,
    VlobCreateRep,
    VlobID,
    VlobListVersionsRep,
    VlobMaintenanceGetReencryptionBatchRep,
    VlobMaintenanceSaveReencryptionBatchRep,
    VlobPollChangesRep,
    VlobReadRep,
)


class EventsSubscribeCallbackType(Protocol):
    def __call__(self) -> Awaitable[EventsSubscribeRep]:
        ...


class EventsListenCallbackType(Protocol):
    def __call__(self, wait: bool = False) -> Awaitable[EventsListenRep]:
        ...


class AuthenticatedPingCallbackType(Protocol):
    def __call__(self, ping: str = "") -> Awaitable[AuthenticatedPingRep]:
        ...


class MessageGetCallbackType(Protocol):
    def __call__(self, offset: int) -> Awaitable[MessageGetRep]:
        ...


class UserGetCallbackType(Protocol):
    def __call__(self, user_id: UserID) -> Awaitable[UserGetRep]:
        ...


class UserCreateCallbackType(Protocol):
    def __call__(
        self,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> Awaitable[UserCreateRep]:
        ...


class UserRevokeCallbackType(Protocol):
    def __call__(self, revoked_user_certificate: bytes) -> Awaitable[UserRevokeRep]:
        ...


class DeviceCreateCallbackType(Protocol):
    def __call__(
        self, device_certificate: bytes, redacted_device_certificate: bytes
    ) -> Awaitable[DeviceCreateRep]:
        ...


class OrganizationBootstrapCallbackType(Protocol):
    def __call__(
        self,
        organization_id: OrganizationID,
        bootstrap_token: str,
        root_verify_key: VerifyKey,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> Awaitable[dict[str, Any]]:
        ...


class PkiEnrollmentListCallbackType(Protocol):
    def __call__(self) -> Awaitable[Any]:
        ...


class PkiEnrollmentRejectCallbackType(Protocol):
    def __call__(self, enrollment_id: UUID) -> Awaitable[dict[str, Any]]:
        ...


class PkiEnrollmentAcceptCallbackType(Protocol):
    def __call__(
        self,
        enrollment_id: UUID,
        accepter_der_x509_certificate: bytes,
        accept_payload_signature: bytes,
        accept_payload: bytes,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> Awaitable[dict[str, Any]]:
        ...


class HumanFindCallbackType(Protocol):
    def __call__(
        self,
        query: Optional[str] = None,
        page: int = 1,
        per_page: int = 100,
        omit_revoked: bool = False,
        omit_non_human: bool = False,
    ) -> Awaitable[HumanFindRep]:
        ...


class InviteNewCallbackType(Protocol):
    def __call__(
        self,
        type: InvitationType,
        send_email: bool = False,
        claimer_email: Optional[str] = None,
    ) -> Awaitable[InviteNewRep]:
        ...


class InviteListCallbackType(Protocol):
    def __call__(self) -> Awaitable[InviteListRep]:
        ...


class InviteDeleteCallbackType(Protocol):
    def __call__(
        self, token: InvitationToken, reason: InvitationDeletedReason
    ) -> Awaitable[InviteDeleteRep]:
        ...


class InviteInfoCallbackType(Protocol):
    def __call__(self) -> Awaitable[InviteInfoRep]:
        ...


class Invite1ClaimerWaitPeerCallbackType(Protocol):
    def __call__(self, claimer_public_key: PublicKey) -> Awaitable[Invite1ClaimerWaitPeerRep]:
        ...


class Invite1GreeterWaitPeerCallbackType(Protocol):
    def __call__(
        self, token: InvitationToken, greeter_public_key: PublicKey
    ) -> Awaitable[Invite1GreeterWaitPeerRep]:
        ...


class Invite2aClaimerSendHashedNonceCallbackType(Protocol):
    def __call__(self, claimer_hashed_nonce: bytes) -> Awaitable[Invite2aClaimerSendHashedNonceRep]:
        ...


class Invite2aGreeterGetHashedNonceCallbackType(Protocol):
    def __call__(self, token: InvitationToken) -> Awaitable[Invite2aGreeterGetHashedNonceRep]:
        ...


class Invite2bGreeterSendNonceCallbackType(Protocol):
    def __call__(
        self, token: InvitationToken, greeter_nonce: bytes
    ) -> Awaitable[Invite2bGreeterSendNonceRep]:
        ...


class Invite3aGreeterWaitPeerTrustCallbackType(Protocol):
    def __call__(self, token: InvitationToken) -> Awaitable[Invite3aGreeterWaitPeerTrustRep]:
        ...


class Invite3aClaimerSignifyTrustCallbackType(Protocol):
    def __call__(self) -> Awaitable[Invite3aClaimerSignifyTrustRep]:
        ...


class Invite3bClaimerWaitPeerTrustCallbackType(Protocol):
    def __call__(self) -> Awaitable[Invite3bClaimerWaitPeerTrustRep]:
        ...


class Invite3bGreeterSignifyTrustCallbackType(Protocol):
    def __call__(self, token: InvitationToken) -> Awaitable[Invite3bGreeterSignifyTrustRep]:
        ...


class Invite4GreeterCommunicateCallbackType(Protocol):
    def __call__(
        self, token: InvitationToken, payload: Optional[bytes]
    ) -> Awaitable[Invite4GreeterCommunicateRep]:
        ...


class Invite4ClaimerCommunicateCallbackType(Protocol):
    def __call__(self, payload: Optional[bytes]) -> Awaitable[Invite4ClaimerCommunicateRep]:
        ...


class BlockCreateCallbackType(Protocol):
    def __call__(
        self, block_id: BlockID, realm_id: RealmID, block: bytes
    ) -> Awaitable[BlockCreateRep]:
        ...


class BlockReadCallbackType(Protocol):
    def __call__(self, block_id: BlockID) -> Awaitable[BlockReadRep]:
        ...


class VlobCreateCallbackType(Protocol):
    def __call__(
        self,
        realm_id: RealmID,
        encryption_revision: int,
        vlob_id: VlobID,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: Optional[dict[SequesterServiceID, bytes]],
    ) -> Awaitable[VlobCreateRep]:
        ...


class VlobReadCallbackType(Protocol):
    def __call__(
        self,
        encryption_revision: int,
        vlob_id: VlobID,
        version: Optional[int] = None,
        timestamp: Optional[DateTime] = None,
    ) -> Awaitable[VlobReadRep]:
        ...


class VlobUpdateCallbackCallbackType(Protocol):
    def __call__(
        self,
        encryption_revision: int,
        vlob_id: VlobID,
        version: int,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: Optional[dict[SequesterServiceID, bytes]],
    ) -> Awaitable[Any]:
        ...


class VlobPollChangesCallbackType(Protocol):
    def __call__(self, realm_id: RealmID, last_checkpoint: int) -> Awaitable[VlobPollChangesRep]:
        ...


class VlobListVersionsCallbackType(Protocol):
    def __call__(self, vlob_id: VlobID) -> Awaitable[VlobListVersionsRep]:
        ...


class VlobMaintenanceGetReenscryptionBatchCallbackType(Protocol):
    def __call__(
        self, realm_id: RealmID, encryption_revision: int, size: int
    ) -> Awaitable[VlobMaintenanceGetReencryptionBatchRep]:
        ...


class VlobMaintenanceSaveReencryptionBatchCallbackType(Protocol):
    def __call__(
        self,
        realm_id: RealmID,
        encryption_revision: int,
        batch: List[Tuple[VlobID, int, bytes]],
    ) -> Awaitable[VlobMaintenanceSaveReencryptionBatchRep]:
        ...


class RealmCreateCallbackType(Protocol):
    def __call__(self, role_certificate: bytes) -> Awaitable[RealmCreateRep]:
        ...


class RealmStatusCallbackType(Protocol):
    def __call__(self, realm_id: RealmID) -> Awaitable[RealmStatusRep]:
        ...


class RealmGetRoleCertificatesCallbackType(Protocol):
    def __call__(self, realm_id: RealmID) -> Awaitable[RealmGetRoleCertificatesRep]:
        ...


class RealmUpdateRolesCallbackType(Protocol):
    def __call__(
        self, role_certificate: bytes, recipient_message: bytes
    ) -> Awaitable[RealmUpdateRolesRep]:
        ...


class RealmStartReencryptionMaintenanceCallbackType(Protocol):
    def __call__(
        self,
        realm_id: RealmID,
        encryption_revision: int,
        timestamp: DateTime,
        per_participant_message: dict[UserID, bytes],
    ) -> Awaitable[RealmStartReencryptionMaintenanceRep]:
        ...


class RealmFinishReencryptionMaintenanceCallbackType(Protocol):
    def __call__(
        self, realm_id: RealmID, encryption_revision: int
    ) -> Awaitable[RealmFinishReencryptionMaintenanceRep]:
        ...


class OrganizationStatsCallbackType(Protocol):
    def __call__(self) -> Awaitable[OrganizationStatsRep]:
        ...


class OrganizationConfigCallbackType(Protocol):
    def __call__(
        self,
    ) -> Awaitable[OrganizationConfigRep]:
        ...
