# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Union

from parsec._parsec_pyi.ids import DeviceID
from parsec._parsec_pyi.addrs import BackendOrganizationAddr
from parsec._parsec_pyi.crypto import SigningKey
from parsec._parsec_pyi.protocol import * # noqa

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
    @classmethod
    def BLOCK_CREATE(cls, req: BlockCreateReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def BLOCK_READ(cls, req: BlockReadReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def DEVICE_CREATE(cls, req: DeviceCreateReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def EVENTS_LISTEN(cls, req: EventsListenReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def EVENTS_SUBSCRIBE(cls, req: EventsSubscribeReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def HUMAN_FIND(cls, req: HumanFindReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE1_GREETER_WAIT_PEER(cls, req: Invite1GreeterWaitPeerReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE2A_GREETER_GET_HASHED_NONCE(
        cls, req: Invite2aGreeterGetHashedNonceReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE2B_GREETER_SEND_NONCE(
        cls, req: Invite2bGreeterSendNonceReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE3A_GREETER_WAIT_PEER_TRUST(
        cls, req: Invite3aGreeterWaitPeerTrustReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE3B_GREETER_SIGNIFY_TRUST(
        cls, req: Invite3bGreeterSignifyTrustReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE4GREETER_COMMUNICATE(
        cls, req: Invite4GreeterCommunicateReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE_DELETE(cls, req: InviteDeleteReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE_LIST(cls, req: InviteListReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def INVITE_NEW(cls, req: InviteNewReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def MESSAGE_GET(cls, req: MessageGetReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def ORGANIZATION_STATS(cls, req: OrganizationStatsReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def PKI_ENROLLMENT_ACCEPT(cls, req: PkiEnrollmentAcceptReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def PKI_ENROLLMENT_LIST(cls, req: PkiEnrollmentListReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def PKI_ENROLLMENT_REJECT(cls, req: PkiEnrollmentRejectReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def REALM_CREATE(cls, req: RealmCreateReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def REALM_FINISH_REENCRYPTION_MAINTENANCE(
        cls, req: RealmFinishReencryptionMaintenanceReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def REALM_GET_ROLE_CERTIFICATES(
        cls, req: RealmGetRoleCertificatesReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def REALM_START_REENCRYPTION_MAINTENANCE(
        cls, req: RealmStartReencryptionMaintenanceReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def REALM_STATS(cls, req: RealmStatsReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def REALM_STATUS(cls, req: RealmStatusReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def REALM_UPDATE_ROLES(cls, req: RealmUpdateRolesReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def USER_CREATE(cls, req: UserCreateReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def USER_GET(cls, req: UserGetReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def USER_REVOKE(cls, req: UserRevokeReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def VLOB_CREATE(cls, req: VlobCreateReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def VLOB_LIST_VERSIONS(cls, req: VlobListVersionsReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def VLOB_MAINTENANCE_GET_REENCRYPTION_BATCH(
        cls, req: VlobMaintenanceGetReencryptionBatchReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def VLOB_MAINTENANCE_SAVE_REENCRYPTION_BATCH(
        cls, req: VlobMaintenanceSaveReencryptionBatchReq
    ) -> AuthenticatedCmdsType: ...
    @classmethod
    def VLOB_POLL_CHANGES(cls, req: VlobPollChangesReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def VLOB_READ(cls, req: VlobReadReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def VLOB_UPDATE(cls, req: VlobUpdateReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def ORGANIZATION_CONFIG(cls, req: OrganizationConfigReq) -> AuthenticatedCmdsType: ...
    @classmethod
    def PING(cls, req: AuthenticatedPingReq) -> AuthenticatedCmdsType: ...

class AuthenticatedCmds:
    def __init__(
        self, server_url: BackendOrganizationAddr, device_id: DeviceID, signing_key: SigningKey
    ) -> None: ...
    async def send_command(self, cmd: AuthenticatedCmdsType) -> AllAuthenticatedCmds: ...
