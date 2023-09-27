// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E }

export enum DeviceFileType {
    Password = 'DeviceFileTypePassword',
    Recovery = 'DeviceFileTypeRecovery',
    Smartcard = 'DeviceFileTypeSmartcard',
}

export enum InvitationEmailSentStatus {
    BadRecipient = 'InvitationEmailSentStatusBadRecipient',
    NotAvailable = 'InvitationEmailSentStatusNotAvailable',
    Success = 'InvitationEmailSentStatusSuccess',
}

export enum InvitationStatus {
    Deleted = 'InvitationStatusDeleted',
    Idle = 'InvitationStatusIdle',
    Ready = 'InvitationStatusReady',
}

export enum Platform {
    Android = 'PlatformAndroid',
    Linux = 'PlatformLinux',
    MacOS = 'PlatformMacOS',
    Web = 'PlatformWeb',
    Windows = 'PlatformWindows',
}

export enum RealmRole {
    Contributor = 'RealmRoleContributor',
    Manager = 'RealmRoleManager',
    Owner = 'RealmRoleOwner',
    Reader = 'RealmRoleReader',
}

export enum UserProfile {
    Admin = 'UserProfileAdmin',
    Outsider = 'UserProfileOutsider',
    Standard = 'UserProfileStandard',
}
export type BackendAddr = string
export type BackendInvitationAddr = string
export type BackendOrganizationAddr = string
export type BackendOrganizationBootstrapAddr = string
export type DateTime = string
export type DeviceID = string
export type DeviceLabel = string
export type EntryName = string
export type InvitationToken = string
export type OrganizationID = string
export type Password = string
export type Path = string
export type SASCode = string
export type UserID = string
export type VlobID = string
export type SequesterVerifyKeyDer = Uint8Array
export type CacheSize = number
export type Handle = number

export interface AvailableDevice {
    keyFilePath: Path
    organizationId: OrganizationID
    deviceId: DeviceID
    humanHandle: HumanHandle | null
    deviceLabel: DeviceLabel | null
    slug: string
    ty: DeviceFileType
}

export interface ClientConfig {
    configDir: Path
    dataBaseDir: Path
    mountpointBaseDir: Path
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
}

export interface ClientInfo {
    organizationId: OrganizationID
    deviceId: DeviceID
    userId: UserID
    deviceLabel: DeviceLabel | null
    humanHandle: HumanHandle | null
    currentProfile: UserProfile
}

export interface DeviceClaimFinalizeInfo {
    handle: number
}

export interface DeviceClaimInProgress1Info {
    handle: number
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface DeviceClaimInProgress2Info {
    handle: number
    claimerSas: SASCode
}

export interface DeviceClaimInProgress3Info {
    handle: number
}

export interface DeviceGreetInProgress1Info {
    handle: number
    greeterSas: SASCode
}

export interface DeviceGreetInProgress2Info {
    handle: number
    claimerSas: SASCode
    claimerSasChoices: Array<SASCode>
}

export interface DeviceGreetInProgress3Info {
    handle: number
}

export interface DeviceGreetInProgress4Info {
    handle: number
    requestedDeviceLabel: DeviceLabel | null
}

export interface DeviceGreetInitialInfo {
    handle: number
}

export interface HumanHandle {
    email: string
    label: string
}

export interface UserClaimFinalizeInfo {
    handle: number
}

export interface UserClaimInProgress1Info {
    handle: number
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface UserClaimInProgress2Info {
    handle: number
    claimerSas: SASCode
}

export interface UserClaimInProgress3Info {
    handle: number
}

export interface UserGreetInProgress1Info {
    handle: number
    greeterSas: SASCode
}

export interface UserGreetInProgress2Info {
    handle: number
    claimerSas: SASCode
    claimerSasChoices: Array<SASCode>
}

export interface UserGreetInProgress3Info {
    handle: number
}

export interface UserGreetInProgress4Info {
    handle: number
    requestedHumanHandle: HumanHandle | null
    requestedDeviceLabel: DeviceLabel | null
}

export interface UserGreetInitialInfo {
    handle: number
}

export interface WorkspaceInfo {
    id: VlobID
    name: EntryName
    selfRole: RealmRole
}

// BootstrapOrganizationError
export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: 'AlreadyUsedToken'
    error: string
}
export interface BootstrapOrganizationErrorBadTimestamp {
    tag: 'BadTimestamp'
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface BootstrapOrganizationErrorInternal {
    tag: 'Internal'
    error: string
}
export interface BootstrapOrganizationErrorInvalidToken {
    tag: 'InvalidToken'
    error: string
}
export interface BootstrapOrganizationErrorOffline {
    tag: 'Offline'
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceError {
    tag: 'SaveDeviceError'
    error: string
}
export type BootstrapOrganizationError =
  | BootstrapOrganizationErrorAlreadyUsedToken
  | BootstrapOrganizationErrorBadTimestamp
  | BootstrapOrganizationErrorInternal
  | BootstrapOrganizationErrorInvalidToken
  | BootstrapOrganizationErrorOffline
  | BootstrapOrganizationErrorSaveDeviceError

// CancelError
export interface CancelErrorInternal {
    tag: 'Internal'
    error: string
}
export interface CancelErrorNotBound {
    tag: 'NotBound'
    error: string
}
export type CancelError =
  | CancelErrorInternal
  | CancelErrorNotBound

// ClaimInProgressError
export interface ClaimInProgressErrorActiveUsersLimitReached {
    tag: 'ActiveUsersLimitReached'
    error: string
}
export interface ClaimInProgressErrorAlreadyUsed {
    tag: 'AlreadyUsed'
    error: string
}
export interface ClaimInProgressErrorCancelled {
    tag: 'Cancelled'
    error: string
}
export interface ClaimInProgressErrorCorruptedConfirmation {
    tag: 'CorruptedConfirmation'
    error: string
}
export interface ClaimInProgressErrorInternal {
    tag: 'Internal'
    error: string
}
export interface ClaimInProgressErrorNotFound {
    tag: 'NotFound'
    error: string
}
export interface ClaimInProgressErrorOffline {
    tag: 'Offline'
    error: string
}
export interface ClaimInProgressErrorPeerReset {
    tag: 'PeerReset'
    error: string
}
export type ClaimInProgressError =
  | ClaimInProgressErrorActiveUsersLimitReached
  | ClaimInProgressErrorAlreadyUsed
  | ClaimInProgressErrorCancelled
  | ClaimInProgressErrorCorruptedConfirmation
  | ClaimInProgressErrorInternal
  | ClaimInProgressErrorNotFound
  | ClaimInProgressErrorOffline
  | ClaimInProgressErrorPeerReset

// ClaimerGreeterAbortOperationError
export interface ClaimerGreeterAbortOperationErrorInternal {
    tag: 'Internal'
    error: string
}
export type ClaimerGreeterAbortOperationError =
  | ClaimerGreeterAbortOperationErrorInternal

// ClaimerRetrieveInfoError
export interface ClaimerRetrieveInfoErrorAlreadyUsed {
    tag: 'AlreadyUsed'
    error: string
}
export interface ClaimerRetrieveInfoErrorInternal {
    tag: 'Internal'
    error: string
}
export interface ClaimerRetrieveInfoErrorNotFound {
    tag: 'NotFound'
    error: string
}
export interface ClaimerRetrieveInfoErrorOffline {
    tag: 'Offline'
    error: string
}
export type ClaimerRetrieveInfoError =
  | ClaimerRetrieveInfoErrorAlreadyUsed
  | ClaimerRetrieveInfoErrorInternal
  | ClaimerRetrieveInfoErrorNotFound
  | ClaimerRetrieveInfoErrorOffline

// ClientCreateWorkspaceError
export interface ClientCreateWorkspaceErrorInternal {
    tag: 'Internal'
    error: string
}
export type ClientCreateWorkspaceError =
  | ClientCreateWorkspaceErrorInternal

// ClientEvent
export interface ClientEventPing {
    tag: 'Ping'
    ping: string
}
export type ClientEvent =
  | ClientEventPing

// ClientInfoError
export interface ClientInfoErrorInternal {
    tag: 'Internal'
    error: string
}
export type ClientInfoError =
  | ClientInfoErrorInternal

// ClientListWorkspacesError
export interface ClientListWorkspacesErrorInternal {
    tag: 'Internal'
    error: string
}
export type ClientListWorkspacesError =
  | ClientListWorkspacesErrorInternal

// ClientRenameWorkspaceError
export interface ClientRenameWorkspaceErrorInternal {
    tag: 'Internal'
    error: string
}
export interface ClientRenameWorkspaceErrorUnknownWorkspace {
    tag: 'UnknownWorkspace'
    error: string
}
export type ClientRenameWorkspaceError =
  | ClientRenameWorkspaceErrorInternal
  | ClientRenameWorkspaceErrorUnknownWorkspace

// ClientShareWorkspaceError
export interface ClientShareWorkspaceErrorBadTimestamp {
    tag: 'BadTimestamp'
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientShareWorkspaceErrorInternal {
    tag: 'Internal'
    error: string
}
export interface ClientShareWorkspaceErrorNotAllowed {
    tag: 'NotAllowed'
    error: string
}
export interface ClientShareWorkspaceErrorOffline {
    tag: 'Offline'
    error: string
}
export interface ClientShareWorkspaceErrorOutsiderCannotBeManagerOrOwner {
    tag: 'OutsiderCannotBeManagerOrOwner'
    error: string
}
export interface ClientShareWorkspaceErrorRevokedRecipient {
    tag: 'RevokedRecipient'
    error: string
}
export interface ClientShareWorkspaceErrorShareToSelf {
    tag: 'ShareToSelf'
    error: string
}
export interface ClientShareWorkspaceErrorUnknownRecipient {
    tag: 'UnknownRecipient'
    error: string
}
export interface ClientShareWorkspaceErrorUnknownRecipientOrWorkspace {
    tag: 'UnknownRecipientOrWorkspace'
    error: string
}
export interface ClientShareWorkspaceErrorUnknownWorkspace {
    tag: 'UnknownWorkspace'
    error: string
}
export interface ClientShareWorkspaceErrorWorkspaceInMaintenance {
    tag: 'WorkspaceInMaintenance'
    error: string
}
export type ClientShareWorkspaceError =
  | ClientShareWorkspaceErrorBadTimestamp
  | ClientShareWorkspaceErrorInternal
  | ClientShareWorkspaceErrorNotAllowed
  | ClientShareWorkspaceErrorOffline
  | ClientShareWorkspaceErrorOutsiderCannotBeManagerOrOwner
  | ClientShareWorkspaceErrorRevokedRecipient
  | ClientShareWorkspaceErrorShareToSelf
  | ClientShareWorkspaceErrorUnknownRecipient
  | ClientShareWorkspaceErrorUnknownRecipientOrWorkspace
  | ClientShareWorkspaceErrorUnknownWorkspace
  | ClientShareWorkspaceErrorWorkspaceInMaintenance

// ClientStartError
export interface ClientStartErrorDeviceAlreadyRunning {
    tag: 'DeviceAlreadyRunning'
    error: string
}
export interface ClientStartErrorInternal {
    tag: 'Internal'
    error: string
}
export interface ClientStartErrorLoadDeviceDecryptionFailed {
    tag: 'LoadDeviceDecryptionFailed'
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidData {
    tag: 'LoadDeviceInvalidData'
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidPath {
    tag: 'LoadDeviceInvalidPath'
    error: string
}
export type ClientStartError =
  | ClientStartErrorDeviceAlreadyRunning
  | ClientStartErrorInternal
  | ClientStartErrorLoadDeviceDecryptionFailed
  | ClientStartErrorLoadDeviceInvalidData
  | ClientStartErrorLoadDeviceInvalidPath

// ClientStartInvitationGreetError
export interface ClientStartInvitationGreetErrorInternal {
    tag: 'Internal'
    error: string
}
export type ClientStartInvitationGreetError =
  | ClientStartInvitationGreetErrorInternal

// ClientStopError
export interface ClientStopErrorInternal {
    tag: 'Internal'
    error: string
}
export type ClientStopError =
  | ClientStopErrorInternal

// DeleteInvitationError
export interface DeleteInvitationErrorAlreadyDeleted {
    tag: 'AlreadyDeleted'
    error: string
}
export interface DeleteInvitationErrorInternal {
    tag: 'Internal'
    error: string
}
export interface DeleteInvitationErrorNotFound {
    tag: 'NotFound'
    error: string
}
export interface DeleteInvitationErrorOffline {
    tag: 'Offline'
    error: string
}
export type DeleteInvitationError =
  | DeleteInvitationErrorAlreadyDeleted
  | DeleteInvitationErrorInternal
  | DeleteInvitationErrorNotFound
  | DeleteInvitationErrorOffline

// DeviceAccessStrategy
export interface DeviceAccessStrategyPassword {
    tag: 'Password'
    password: Password
    keyFile: Path
}
export interface DeviceAccessStrategySmartcard {
    tag: 'Smartcard'
    keyFile: Path
}
export type DeviceAccessStrategy =
  | DeviceAccessStrategyPassword
  | DeviceAccessStrategySmartcard

// DeviceSaveStrategy
export interface DeviceSaveStrategyPassword {
    tag: 'Password'
    password: Password
}
export interface DeviceSaveStrategySmartcard {
    tag: 'Smartcard'
}
export type DeviceSaveStrategy =
  | DeviceSaveStrategyPassword
  | DeviceSaveStrategySmartcard

// GreetInProgressError
export interface GreetInProgressErrorActiveUsersLimitReached {
    tag: 'ActiveUsersLimitReached'
    error: string
}
export interface GreetInProgressErrorAlreadyUsed {
    tag: 'AlreadyUsed'
    error: string
}
export interface GreetInProgressErrorBadTimestamp {
    tag: 'BadTimestamp'
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface GreetInProgressErrorCancelled {
    tag: 'Cancelled'
    error: string
}
export interface GreetInProgressErrorCorruptedInviteUserData {
    tag: 'CorruptedInviteUserData'
    error: string
}
export interface GreetInProgressErrorDeviceAlreadyExists {
    tag: 'DeviceAlreadyExists'
    error: string
}
export interface GreetInProgressErrorInternal {
    tag: 'Internal'
    error: string
}
export interface GreetInProgressErrorNonceMismatch {
    tag: 'NonceMismatch'
    error: string
}
export interface GreetInProgressErrorNotFound {
    tag: 'NotFound'
    error: string
}
export interface GreetInProgressErrorOffline {
    tag: 'Offline'
    error: string
}
export interface GreetInProgressErrorPeerReset {
    tag: 'PeerReset'
    error: string
}
export interface GreetInProgressErrorUserAlreadyExists {
    tag: 'UserAlreadyExists'
    error: string
}
export interface GreetInProgressErrorUserCreateNotAllowed {
    tag: 'UserCreateNotAllowed'
    error: string
}
export type GreetInProgressError =
  | GreetInProgressErrorActiveUsersLimitReached
  | GreetInProgressErrorAlreadyUsed
  | GreetInProgressErrorBadTimestamp
  | GreetInProgressErrorCancelled
  | GreetInProgressErrorCorruptedInviteUserData
  | GreetInProgressErrorDeviceAlreadyExists
  | GreetInProgressErrorInternal
  | GreetInProgressErrorNonceMismatch
  | GreetInProgressErrorNotFound
  | GreetInProgressErrorOffline
  | GreetInProgressErrorPeerReset
  | GreetInProgressErrorUserAlreadyExists
  | GreetInProgressErrorUserCreateNotAllowed

// InviteListItem
export interface InviteListItemDevice {
    tag: 'Device'
    token: InvitationToken
    createdOn: DateTime
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: 'User'
    token: InvitationToken
    createdOn: DateTime
    claimerEmail: string
    status: InvitationStatus
}
export type InviteListItem =
  | InviteListItemDevice
  | InviteListItemUser

// ListInvitationsError
export interface ListInvitationsErrorInternal {
    tag: 'Internal'
    error: string
}
export interface ListInvitationsErrorOffline {
    tag: 'Offline'
    error: string
}
export type ListInvitationsError =
  | ListInvitationsErrorInternal
  | ListInvitationsErrorOffline

// NewDeviceInvitationError
export interface NewDeviceInvitationErrorInternal {
    tag: 'Internal'
    error: string
}
export interface NewDeviceInvitationErrorOffline {
    tag: 'Offline'
    error: string
}
export interface NewDeviceInvitationErrorSendEmailToUserWithoutEmail {
    tag: 'SendEmailToUserWithoutEmail'
    error: string
}
export type NewDeviceInvitationError =
  | NewDeviceInvitationErrorInternal
  | NewDeviceInvitationErrorOffline
  | NewDeviceInvitationErrorSendEmailToUserWithoutEmail

// NewUserInvitationError
export interface NewUserInvitationErrorAlreadyMember {
    tag: 'AlreadyMember'
    error: string
}
export interface NewUserInvitationErrorInternal {
    tag: 'Internal'
    error: string
}
export interface NewUserInvitationErrorNotAllowed {
    tag: 'NotAllowed'
    error: string
}
export interface NewUserInvitationErrorOffline {
    tag: 'Offline'
    error: string
}
export type NewUserInvitationError =
  | NewUserInvitationErrorAlreadyMember
  | NewUserInvitationErrorInternal
  | NewUserInvitationErrorNotAllowed
  | NewUserInvitationErrorOffline

// ParseBackendAddrError
export interface ParseBackendAddrErrorInvalidUrl {
    tag: 'InvalidUrl'
    error: string
}
export type ParseBackendAddrError =
  | ParseBackendAddrErrorInvalidUrl

// ParsedBackendAddr
export interface ParsedBackendAddrBase {
    tag: 'Base'
    hostname: string
    port: number
    useSsl: boolean
}
export interface ParsedBackendAddrInvitationDevice {
    tag: 'InvitationDevice'
    hostname: string
    port: number
    useSsl: boolean
    organizationId: OrganizationID
    token: InvitationToken
}
export interface ParsedBackendAddrInvitationUser {
    tag: 'InvitationUser'
    hostname: string
    port: number
    useSsl: boolean
    organizationId: OrganizationID
    token: InvitationToken
}
export interface ParsedBackendAddrOrganizationBootstrap {
    tag: 'OrganizationBootstrap'
    hostname: string
    port: number
    useSsl: boolean
    organizationId: OrganizationID
    token: string | null
}
export interface ParsedBackendAddrOrganizationFileLink {
    tag: 'OrganizationFileLink'
    hostname: string
    port: number
    useSsl: boolean
    organizationId: OrganizationID
    workspaceId: VlobID
    encryptedPath: Uint8Array
    encryptedTimestamp: Uint8Array | null
}
export interface ParsedBackendAddrPkiEnrollment {
    tag: 'PkiEnrollment'
    hostname: string
    port: number
    useSsl: boolean
    organizationId: OrganizationID
}
export type ParsedBackendAddr =
  | ParsedBackendAddrBase
  | ParsedBackendAddrInvitationDevice
  | ParsedBackendAddrInvitationUser
  | ParsedBackendAddrOrganizationBootstrap
  | ParsedBackendAddrOrganizationFileLink
  | ParsedBackendAddrPkiEnrollment

// UserOrDeviceClaimInitialInfo
export interface UserOrDeviceClaimInitialInfoDevice {
    tag: 'Device'
    handle: number
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle | null
}
export interface UserOrDeviceClaimInitialInfoUser {
    tag: 'User'
    handle: number
    claimerEmail: string
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle | null
}
export type UserOrDeviceClaimInitialInfo =
  | UserOrDeviceClaimInitialInfoDevice
  | UserOrDeviceClaimInitialInfoUser

// WorkspaceStorageCacheSize
export interface WorkspaceStorageCacheSizeCustom {
    tag: 'Custom'
    size: number
}
export interface WorkspaceStorageCacheSizeDefault {
    tag: 'Default'
}
export type WorkspaceStorageCacheSize =
  | WorkspaceStorageCacheSizeCustom
  | WorkspaceStorageCacheSizeDefault

export interface LibParsecPlugin {
    bootstrapOrganization(
        config: ClientConfig,
        on_event_callback: (event: ClientEvent) => void,
        bootstrap_organization_addr: BackendOrganizationBootstrapAddr,
        save_strategy: DeviceSaveStrategy,
        human_handle: HumanHandle | null,
        device_label: DeviceLabel | null,
        sequester_authority_verify_key: SequesterVerifyKeyDer | null
    ): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
    buildBackendOrganizationBootstrapAddr(
        addr: BackendAddr,
        organization_id: OrganizationID
    ): Promise<BackendOrganizationBootstrapAddr>
    cancel(
        canceller: number
    ): Promise<Result<null, CancelError>>
    claimerDeviceFinalizeSaveLocalDevice(
        handle: number,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimInProgressError>>
    claimerDeviceInProgress1DoSignifyTrust(
        canceller: number,
        handle: number
    ): Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>>
    claimerDeviceInProgress2DoWaitPeerTrust(
        canceller: number,
        handle: number
    ): Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>>
    claimerDeviceInProgress3DoClaim(
        canceller: number,
        handle: number,
        requested_device_label: DeviceLabel | null
    ): Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>>
    claimerDeviceInitialDoWaitPeer(
        canceller: number,
        handle: number
    ): Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>>
    claimerGreeterAbortOperation(
        handle: number
    ): Promise<Result<null, ClaimerGreeterAbortOperationError>>
    claimerRetrieveInfo(
        config: ClientConfig,
        on_event_callback: (event: ClientEvent) => void,
        addr: BackendInvitationAddr
    ): Promise<Result<UserOrDeviceClaimInitialInfo, ClaimerRetrieveInfoError>>
    claimerUserFinalizeSaveLocalDevice(
        handle: number,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimInProgressError>>
    claimerUserInProgress1DoSignifyTrust(
        canceller: number,
        handle: number
    ): Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>>
    claimerUserInProgress2DoWaitPeerTrust(
        canceller: number,
        handle: number
    ): Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>>
    claimerUserInProgress3DoClaim(
        canceller: number,
        handle: number,
        requested_device_label: DeviceLabel | null,
        requested_human_handle: HumanHandle | null
    ): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>>
    claimerUserInitialDoWaitPeer(
        canceller: number,
        handle: number
    ): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
    clientCreateWorkspace(
        client: number,
        name: EntryName
    ): Promise<Result<VlobID, ClientCreateWorkspaceError>>
    clientDeleteInvitation(
        client: number,
        token: InvitationToken
    ): Promise<Result<null, DeleteInvitationError>>
    clientInfo(
        client: number
    ): Promise<Result<ClientInfo, ClientInfoError>>
    clientListInvitations(
        client: number
    ): Promise<Result<Array<InviteListItem>, ListInvitationsError>>
    clientListWorkspaces(
        client: number
    ): Promise<Result<Array<WorkspaceInfo>, ClientListWorkspacesError>>
    clientNewDeviceInvitation(
        client: number,
        send_email: boolean
    ): Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewDeviceInvitationError>>
    clientNewUserInvitation(
        client: number,
        claimer_email: string,
        send_email: boolean
    ): Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewUserInvitationError>>
    clientRenameWorkspace(
        client: number,
        realm_id: VlobID,
        new_name: EntryName
    ): Promise<Result<null, ClientRenameWorkspaceError>>
    clientShareWorkspace(
        client: number,
        realm_id: VlobID,
        recipient: UserID,
        role: RealmRole | null
    ): Promise<Result<null, ClientShareWorkspaceError>>
    clientStart(
        config: ClientConfig,
        on_event_callback: (event: ClientEvent) => void,
        access: DeviceAccessStrategy
    ): Promise<Result<number, ClientStartError>>
    clientStartDeviceInvitationGreet(
        client: number,
        token: InvitationToken
    ): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>>
    clientStartUserInvitationGreet(
        client: number,
        token: InvitationToken
    ): Promise<Result<UserGreetInitialInfo, ClientStartInvitationGreetError>>
    clientStop(
        client: number
    ): Promise<Result<null, ClientStopError>>
    getPlatform(
    ): Promise<Platform>
    greeterDeviceInProgress1DoWaitPeerTrust(
        canceller: number,
        handle: number
    ): Promise<Result<DeviceGreetInProgress2Info, GreetInProgressError>>
    greeterDeviceInProgress2DoSignifyTrust(
        canceller: number,
        handle: number
    ): Promise<Result<DeviceGreetInProgress3Info, GreetInProgressError>>
    greeterDeviceInProgress3DoGetClaimRequests(
        canceller: number,
        handle: number
    ): Promise<Result<DeviceGreetInProgress4Info, GreetInProgressError>>
    greeterDeviceInProgress4DoCreate(
        canceller: number,
        handle: number,
        device_label: DeviceLabel | null
    ): Promise<Result<null, GreetInProgressError>>
    greeterDeviceInitialDoWaitPeer(
        canceller: number,
        handle: number
    ): Promise<Result<DeviceGreetInProgress1Info, GreetInProgressError>>
    greeterUserInProgress1DoWaitPeerTrust(
        canceller: number,
        handle: number
    ): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>
    greeterUserInProgress2DoSignifyTrust(
        canceller: number,
        handle: number
    ): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>>
    greeterUserInProgress3DoGetClaimRequests(
        canceller: number,
        handle: number
    ): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>>
    greeterUserInProgress4DoCreate(
        canceller: number,
        handle: number,
        human_handle: HumanHandle | null,
        device_label: DeviceLabel | null,
        profile: UserProfile
    ): Promise<Result<null, GreetInProgressError>>
    greeterUserInitialDoWaitPeer(
        canceller: number,
        handle: number
    ): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>>
    listAvailableDevices(
        path: Path
    ): Promise<Array<AvailableDevice>>
    newCanceller(
    ): Promise<number>
    parseBackendAddr(
        url: string
    ): Promise<Result<ParsedBackendAddr, ParseBackendAddrError>>
    testDropTestbed(
        path: Path
    ): Promise<null>
    testGetTestbedBootstrapOrganizationAddr(
        discriminant_dir: Path
    ): Promise<BackendOrganizationBootstrapAddr | null>
    testGetTestbedOrganizationId(
        discriminant_dir: Path
    ): Promise<OrganizationID | null>
    testNewTestbed(
        template: string,
        test_server: BackendAddr | null
    ): Promise<Path>
    validateDeviceLabel(
        raw: string
    ): Promise<boolean>
    validateEmail(
        raw: string
    ): Promise<boolean>
    validateEntryName(
        raw: string
    ): Promise<boolean>
    validateHumanHandleLabel(
        raw: string
    ): Promise<boolean>
    validateInvitationToken(
        raw: string
    ): Promise<boolean>
    validatePath(
        raw: string
    ): Promise<boolean>
}
