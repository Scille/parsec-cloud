# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

try:
    from ._libparsec import (
        BackendAddr,
        BackendOrganizationAddr,
        BackendOrganizationBootstrapAddr,
        BackendOrganizationFileLinkAddr,
        BackendInvitationAddr,
        BackendActionAddr,
        BackendPkiEnrollmentAddr,
        OrganizationID,
        DeviceName,
        DeviceID,
        UserID,
        HumanHandle,
        EntryID,
        BlockID,
        RealmID,
        VlobID,
        ChunkID,
        FileDescriptor,
        InvitationToken,
        SASCode,
        generate_sas_codes,
        generate_sas_code_candidates,
        InviteUserData,
        InviteUserConfirmation,
        InviteDeviceData,
        InviteDeviceConfirmation,
        DeviceLabel,
        EntryName,
        WorkspaceEntry,
        BlockAccess,
        FileManifest,
        FolderManifest,
        WorkspaceManifest,
        UserManifest,
        Chunk,
        LocalFileManifest,
        LocalFolderManifest,
        LocalWorkspaceManifest,
        LocalUserManifest,
        # Block
        BlockCreateReq,
        BlockCreateRep,
        BlockReadReq,
        BlockReadRep,
        # Cmd
        AuthenticatedAnyCmdReq,
        InvitedAnyCmdReq,
        # Events
        EventsListenReq,
        EventsListenRep,
        EventsSubscribeReq,
        EventsSubscribeRep,
        # Invite
        InviteNewReq,
        InviteNewRep,
        InviteDeleteReq,
        InviteDeleteRep,
        InviteListReq,
        InviteListRep,
        InviteInfoReq,
        InviteInfoRep,
        Invite1ClaimerWaitPeerReq,
        Invite1ClaimerWaitPeerRep,
        Invite1GreeterWaitPeerReq,
        Invite1GreeterWaitPeerRep,
        Invite2aClaimerSendHashedNonceHashNonceReq,
        Invite2aClaimerSendHashedNonceHashNonceRep,
        Invite2aGreeterGetHashedNonceReq,
        Invite2aGreeterGetHashedNonceRep,
        Invite2bClaimerSendNonceReq,
        Invite2bClaimerSendNonceRep,
        Invite2bGreeterSendNonceReq,
        Invite2bGreeterSendNonceRep,
        Invite3aClaimerSignifyTrustReq,
        Invite3aClaimerSignifyTrustRep,
        Invite3aGreeterWaitPeerTrustReq,
        Invite3aGreeterWaitPeerTrustRep,
        Invite3bClaimerWaitPeerTrustReq,
        Invite3bClaimerWaitPeerTrustRep,
        Invite3bGreeterSignifyTrustReq,
        Invite3bGreeterSignifyTrustRep,
        Invite4ClaimerCommunicateReq,
        Invite4ClaimerCommunicateRep,
        Invite4GreeterCommunicateReq,
        Invite4GreeterCommunicateRep,
        InviteListItem,
        # Message
        MessageGetReq,
        MessageGetRep,
        Message,
        # Organization
        OrganizationStatsReq,
        OrganizationStatsRep,
        OrganizationConfigReq,
        OrganizationConfigRep,
        UsersPerProfileDetailItem,
        # Ping
        AuthenticatedPingReq,
        AuthenticatedPingRep,
        InvitedPingReq,
        InvitedPingRep,
        # Realm
        RealmCreateReq,
        RealmCreateRep,
        RealmStatusReq,
        RealmStatusRep,
        RealmStatsReq,
        RealmStatsRep,
        RealmGetRoleCertificateReq,
        RealmGetRoleCertificateRep,
        RealmUpdateRolesReq,
        RealmUpdateRolesRep,
        RealmStartReencryptionMaintenanceReq,
        RealmStartReencryptionMaintenanceRep,
        RealmFinishReencryptionMaintenanceReq,
        RealmFinishReencryptionMaintenanceRep,
        # User
        UserGetReq,
        UserGetRep,
        UserCreateReq,
        UserCreateRep,
        UserRevokeReq,
        UserRevokeRep,
        DeviceCreateReq,
        DeviceCreateRep,
        HumanFindReq,
        HumanFindRep,
        Trustchain,
        HumanFindResultItem,
        # Vlob
        VlobCreateReq,
        VlobCreateRep,
        VlobReadReq,
        VlobReadRep,
        VlobUpdateReq,
        VlobUpdateRep,
        VlobPollChangesReq,
        VlobPollChangesRep,
        VlobListVersionsReq,
        VlobListVersionsRep,
        VlobMaintenanceGetReencryptionBatchReq,
        VlobMaintenanceGetReencryptionBatchRep,
        VlobMaintenanceSaveReencryptionBatchReq,
        VlobMaintenanceSaveReencryptionBatchRep,
        ReencryptionBatchEntry,
        # Trustchain
        UserCertificate,
        DeviceCertificate,
        RevokedUserCertificate,
        TrustchainContext,
        freeze_time,
        # LocalDevice
        LocalDevice,
        # WorkspaceStorage
        WorkspaceStorage,
    )
except ImportError as exc:
    print(f"Import error in libparsec/types: {exc}")

__all__ = (
    "BackendAddr",
    "BackendOrganizationAddr",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationFileLinkAddr",
    "BackendInvitationAddr",
    "BackendPkiEnrollmentAddr",
    "OrganizationID",
    "BackendActionAddr",
    "EntryID",
    "DeviceName",
    "DeviceID",
    "UserID",
    "HumanHandle",
    "BlockID",
    "RealmID",
    "VlobID",
    "ChunkID",
    "FileDescriptor",
    "InvitationToken",
    "SASCode",
    "generate_sas_codes",
    "generate_sas_code_candidates",
    "InviteUserData",
    "InviteUserConfirmation",
    "InviteDeviceData",
    "InviteDeviceConfirmation",
    "DeviceLabel",
    "EntryName",
    "WorkspaceEntry",
    "BlockAccess",
    "FileManifest",
    "FolderManifest",
    "WorkspaceManifest",
    "UserManifest",
    "Chunk",
    "LocalFileManifest",
    "LocalFolderManifest",
    "LocalWorkspaceManifest",
    "LocalUserManifest",
    # Block
    "BlockCreateReq",
    "BlockCreateRep",
    "BlockReadReq",
    "BlockReadRep",
    # Cmd
    "AuthenticatedAnyCmdReq",
    "InvitedAnyCmdReq",
    # Events
    "EventsListenReq",
    "EventsListenRep",
    "EventsSubscribeReq",
    "EventsSubscribeRep",
    # Invite
    "InviteNewReq",
    "InviteNewRep",
    "InviteDeleteReq",
    "InviteDeleteRep",
    "InviteListReq",
    "InviteListRep",
    "InviteInfoReq",
    "InviteInfoRep",
    "Invite1ClaimerWaitPeerReq",
    "Invite1ClaimerWaitPeerRep",
    "Invite1GreeterWaitPeerReq",
    "Invite1GreeterWaitPeerRep",
    "Invite2aClaimerSendHashedNonceHashNonceReq",
    "Invite2aClaimerSendHashedNonceHashNonceRep",
    "Invite2aGreeterGetHashedNonceReq",
    "Invite2aGreeterGetHashedNonceRep",
    "Invite2bClaimerSendNonceReq",
    "Invite2bClaimerSendNonceRep",
    "Invite2bGreeterSendNonceReq",
    "Invite2bGreeterSendNonceRep",
    "Invite3aClaimerSignifyTrustReq",
    "Invite3aClaimerSignifyTrustRep",
    "Invite3aGreeterWaitPeerTrustReq",
    "Invite3aGreeterWaitPeerTrustRep",
    "Invite3bClaimerWaitPeerTrustReq",
    "Invite3bClaimerWaitPeerTrustRep",
    "Invite3bGreeterSignifyTrustReq",
    "Invite3bGreeterSignifyTrustRep",
    "Invite4ClaimerCommunicateReq",
    "Invite4ClaimerCommunicateRep",
    "Invite4GreeterCommunicateReq",
    "Invite4GreeterCommunicateRep",
    "InviteListItem",
    # Message
    "MessageGetReq",
    "MessageGetRep",
    "Message",
    # Organization
    "OrganizationStatsReq",
    "OrganizationStatsRep",
    "OrganizationConfigReq",
    "OrganizationConfigRep",
    "UsersPerProfileDetailItem",
    # Ping
    "AuthenticatedPingReq",
    "AuthenticatedPingRep",
    "InvitedPingReq",
    "InvitedPingRep",
    # Realm
    "RealmCreateReq",
    "RealmCreateRep",
    "RealmStatusReq",
    "RealmStatusRep",
    "RealmStatsReq",
    "RealmStatsRep",
    "RealmGetRoleCertificateReq",
    "RealmGetRoleCertificateRep",
    "RealmUpdateRolesReq",
    "RealmUpdateRolesRep",
    "RealmStartReencryptionMaintenanceReq",
    "RealmStartReencryptionMaintenanceRep",
    "RealmFinishReencryptionMaintenanceReq",
    "RealmFinishReencryptionMaintenanceRep",
    # User
    "UserGetReq",
    "UserGetRep",
    "UserCreateReq",
    "UserCreateRep",
    "UserRevokeReq",
    "UserRevokeRep",
    "DeviceCreateReq",
    "DeviceCreateRep",
    "HumanFindReq",
    "HumanFindRep",
    "Trustchain",
    "HumanFindResultItem",
    # Vlob
    "VlobCreateReq",
    "VlobCreateRep",
    "VlobReadReq",
    "VlobReadRep",
    "VlobUpdateReq",
    "VlobUpdateRep",
    "VlobPollChangesReq",
    "VlobPollChangesRep",
    "VlobListVersionsReq",
    "VlobListVersionsRep",
    "VlobMaintenanceGetReencryptionBatchReq",
    "VlobMaintenanceGetReencryptionBatchRep",
    "VlobMaintenanceSaveReencryptionBatchReq",
    "VlobMaintenanceSaveReencryptionBatchRep",
    "ReencryptionBatchEntry",
    # Trustchain
    "UserCertificate",
    "DeviceCertificate",
    "RevokedUserCertificate",
    "TrustchainContext",
    "freeze_time",
    # LocalDevice
    "LocalDevice",
    # Storage
    "WorkspaceStorage",
)
