# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from typing import Union

from parsec._parsec_pyi.addrs import BackendOrganizationAddr
from parsec._parsec_pyi.crypto import SigningKey
from parsec._parsec_pyi.ids import DeviceID
from parsec._parsec_pyi.protocol import (
    AuthenticatedPingReq,
    BlockCreateReq,
    BlockReadReq,
    DeviceCreateReq,
    EventsListenReq,
    EventsSubscribeReq,
    HumanFindReq,
    Invite1GreeterWaitPeerReq,
    Invite2aGreeterGetHashedNonceReq,
    Invite2bGreeterSendNonceReq,
    Invite3aGreeterWaitPeerTrustReq,
    Invite3bGreeterSignifyTrustReq,
    Invite4GreeterCommunicateReq,
    InviteDeleteReq,
    InviteListReq,
    InviteNewReq,
    MessageGetReq,
    OrganizationConfigReq,
    OrganizationStatsReq,
    PkiEnrollmentAcceptReq,
    PkiEnrollmentListReq,
    PkiEnrollmentRejectReq,
    RealmCreateReq,
    RealmFinishReencryptionMaintenanceReq,
    RealmGetRoleCertificatesReq,
    RealmStartReencryptionMaintenanceReq,
    RealmStatsReq,
    RealmStatusReq,
    RealmUpdateRolesReq,
    UserCreateReq,
    UserGetReq,
    UserRevokeReq,
    VlobCreateReq,
    VlobListVersionsReq,
    VlobMaintenanceGetReencryptionBatchReq,
    VlobMaintenanceSaveReencryptionBatchReq,
    VlobPollChangesReq,
    VlobReadReq,
    VlobUpdateReq,
)

AllAuthenticatedCmds = Union[
    BlockCreateReq,
    BlockReadReq,
    DeviceCreateReq,
    EventsListenReq,
    EventsSubscribeReq,
    HumanFindReq,
    Invite1GreeterWaitPeerReq,
    Invite2aGreeterGetHashedNonceReq,
    Invite2bGreeterSendNonceReq,
    Invite3aGreeterWaitPeerTrustReq,
    Invite3bGreeterSignifyTrustReq,
    Invite4GreeterCommunicateReq,
    InviteDeleteReq,
    InviteListReq,
    InviteNewReq,
    MessageGetReq,
    OrganizationStatsReq,
    PkiEnrollmentAcceptReq,
    PkiEnrollmentListReq,
    PkiEnrollmentRejectReq,
    RealmCreateReq,
    RealmFinishReencryptionMaintenanceReq,
    RealmGetRoleCertificatesReq,
    RealmStartReencryptionMaintenanceReq,
    RealmStatsReq,
    RealmStatusReq,
    RealmUpdateRolesReq,
    UserCreateReq,
    UserGetReq,
    UserRevokeReq,
    VlobCreateReq,
    VlobListVersionsReq,
    VlobMaintenanceGetReencryptionBatchReq,
    VlobMaintenanceSaveReencryptionBatchReq,
    VlobPollChangesReq,
    VlobReadReq,
    VlobUpdateReq,
    OrganizationConfigReq,
    AuthenticatedPingReq,
]

class AuthenticatedCmdsType:
    @staticmethod
    def BLOCK_CREATE(req: BlockCreateReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def BLOCK_READ(req: BlockReadReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def DEVICE_CREATE(req: DeviceCreateReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def EVENTS_LISTEN(req: EventsListenReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def EVENTS_SUBSCRIBE(req: EventsSubscribeReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def HUMAN_FIND(req: HumanFindReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE1_GREETER_WAIT_PEER(req: Invite1GreeterWaitPeerReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE2A_GREETER_GET_HASHED_NONCE(
        req: Invite2aGreeterGetHashedNonceReq,
    ) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE2B_GREETER_SEND_NONCE(req: Invite2bGreeterSendNonceReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE3A_GREETER_WAIT_PEER_TRUST(
        req: Invite3aGreeterWaitPeerTrustReq,
    ) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE3B_GREETER_SIGNIFY_TRUST(
        req: Invite3bGreeterSignifyTrustReq,
    ) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE4GREETER_COMMUNICATE(req: Invite4GreeterCommunicateReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE_DELETE(req: InviteDeleteReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE_LIST(req: InviteListReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def INVITE_NEW(req: InviteNewReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def MESSAGE_GET(req: MessageGetReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def ORGANIZATION_STATS(req: OrganizationStatsReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def PKI_ENROLLMENT_ACCEPT(req: PkiEnrollmentAcceptReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def PKI_ENROLLMENT_LIST(req: PkiEnrollmentListReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def PKI_ENROLLMENT_REJECT(req: PkiEnrollmentRejectReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def REALM_CREATE(req: RealmCreateReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def REALM_FINISH_REENCRYPTION_MAINTENANCE(
        req: RealmFinishReencryptionMaintenanceReq,
    ) -> AuthenticatedCmdsType: ...
    @staticmethod
    def REALM_GET_ROLE_CERTIFICATES(req: RealmGetRoleCertificatesReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def REALM_START_REENCRYPTION_MAINTENANCE(
        req: RealmStartReencryptionMaintenanceReq,
    ) -> AuthenticatedCmdsType: ...
    @staticmethod
    def REALM_STATS(req: RealmStatsReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def REALM_STATUS(req: RealmStatusReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def REALM_UPDATE_ROLES(req: RealmUpdateRolesReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def USER_CREATE(req: UserCreateReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def USER_GET(req: UserGetReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def USER_REVOKE(req: UserRevokeReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def VLOB_CREATE(req: VlobCreateReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def VLOB_LIST_VERSIONS(req: VlobListVersionsReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def VLOB_MAINTENANCE_GET_REENCRYPTION_BATCH(
        req: VlobMaintenanceGetReencryptionBatchReq,
    ) -> AuthenticatedCmdsType: ...
    @staticmethod
    def VLOB_MAINTENANCE_SAVE_REENCRYPTION_BATCH(
        req: VlobMaintenanceSaveReencryptionBatchReq,
    ) -> AuthenticatedCmdsType: ...
    @staticmethod
    def VLOB_POLL_CHANGES(req: VlobPollChangesReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def VLOB_READ(req: VlobReadReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def VLOB_UPDATE(req: VlobUpdateReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def ORGANIZATION_CONFIG(req: OrganizationConfigReq) -> AuthenticatedCmdsType: ...
    @staticmethod
    def PING(req: AuthenticatedPingReq) -> AuthenticatedCmdsType: ...

class AuthenticatedCmds:
    def __init__(
        self, server_url: BackendOrganizationAddr, device_id: DeviceID, signing_key: SigningKey
    ) -> None: ...
    async def send_command(self, cmd: AuthenticatedCmdsType) -> AllAuthenticatedCmds: ...
