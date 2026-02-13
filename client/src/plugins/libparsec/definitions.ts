// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
 * /!\ Auto-generated code (see `bindings/generator`), any modification will be lost ! /!\
 */

export type Result<T, E = Error> =
  | { ok: true; value: T }
  | { ok: false; error: E }

export enum CancelledGreetingAttemptReason {
    AutomaticallyCancelled = 'CancelledGreetingAttemptReasonAutomaticallyCancelled',
    InconsistentPayload = 'CancelledGreetingAttemptReasonInconsistentPayload',
    InvalidNonceHash = 'CancelledGreetingAttemptReasonInvalidNonceHash',
    InvalidSasCode = 'CancelledGreetingAttemptReasonInvalidSasCode',
    ManuallyCancelled = 'CancelledGreetingAttemptReasonManuallyCancelled',
    UndecipherablePayload = 'CancelledGreetingAttemptReasonUndecipherablePayload',
    UndeserializablePayload = 'CancelledGreetingAttemptReasonUndeserializablePayload',
}

export enum DevicePurpose {
    PassphraseRecovery = 'DevicePurposePassphraseRecovery',
    Registration = 'DevicePurposeRegistration',
    ShamirRecovery = 'DevicePurposeShamirRecovery',
    Standard = 'DevicePurposeStandard',
}

export enum GreeterOrClaimer {
    Claimer = 'GreeterOrClaimerClaimer',
    Greeter = 'GreeterOrClaimerGreeter',
}

export enum InvitationEmailSentStatus {
    RecipientRefused = 'InvitationEmailSentStatusRecipientRefused',
    ServerUnavailable = 'InvitationEmailSentStatusServerUnavailable',
    Success = 'InvitationEmailSentStatusSuccess',
}

export enum InvitationStatus {
    Cancelled = 'InvitationStatusCancelled',
    Finished = 'InvitationStatusFinished',
    Pending = 'InvitationStatusPending',
}

export enum InvitationType {
    Device = 'InvitationTypeDevice',
    ShamirRecovery = 'InvitationTypeShamirRecovery',
    User = 'InvitationTypeUser',
}

export enum LogLevel {
    Debug = 'LogLevelDebug',
    Error = 'LogLevelError',
    Info = 'LogLevelInfo',
    Trace = 'LogLevelTrace',
    Warn = 'LogLevelWarn',
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

export enum UserOnlineStatus {
    Offline = 'UserOnlineStatusOffline',
    Online = 'UserOnlineStatusOnline',
    Unknown = 'UserOnlineStatusUnknown',
}

export enum UserProfile {
    Admin = 'UserProfileAdmin',
    Outsider = 'UserProfileOutsider',
    Standard = 'UserProfileStandard',
}
export type AccessToken = string
export type AccountAuthMethodID = string
export type AccountVaultItemOpaqueKeyID = string
export type AnyhowError = string
export type ApiVersion = string
export type AsyncEnrollmentID = string
export type DeviceID = string
export type DeviceLabel = string
export type EmailAddress = string
export type EntryName = string
export type FsPath = string
export type GreetingAttemptID = string
export type OrganizationID = string
export type PKIEnrollmentID = string
export type ParsecAddr = string
export type ParsecAsyncEnrollmentAddr = string
export type ParsecInvitationAddr = string
export type ParsecOrganizationAddr = string
export type ParsecOrganizationBootstrapAddr = string
export type ParsecPkiEnrollmentAddr = string
export type ParsecTOTPResetAddr = string
export type ParsecWorkspacePathAddr = string
export type Password = string
export type Path = string
export type SASCode = string
export type SequesterServiceID = string
export type TOTPOpaqueKeyID = string
export type UserID = string
export type VlobID = string
export type X509CertificateHash = string
export type Bytes = Uint8Array
export type BytesVec = Uint8Array
export type KeyDerivation = Uint8Array
export type SecretKey = Uint8Array
export type SequesterVerifyKeyDer = Uint8Array
export type Sha256BoxData = Uint8Array
export type NonZeroU8 = number
export type U8 = number
export type U16 = number
export type I32 = number
export type CacheSize = number
export type FileDescriptor = number
export type Handle = number
export type U32 = number
export type VersionInt = number
export type { DateTime } from 'luxon'; import type { DateTime } from 'luxon';
export type I64 = bigint
export type IndexInt = bigint
export type SizeInt = bigint
export type U64 = bigint

export interface AccountInfo {
    serverAddr: ParsecAddr
    inUseAuthMethod: AccountAuthMethodID
    humanHandle: HumanHandle
}

export interface AccountOrganizations {
    active: Array<AccountOrganizationsActiveUser>
    revoked: Array<AccountOrganizationsRevokedUser>
}

export interface AccountOrganizationsActiveUser {
    organizationId: OrganizationID
    userId: UserID
    createdOn: DateTime
    isFrozen: boolean
    currentProfile: UserProfile
    organizationConfig: AccountOrganizationsOrganizationConfig
}

export interface AccountOrganizationsOrganizationConfig {
    isExpired: boolean
    userProfileOutsiderAllowed: boolean
    activeUsersLimit: ActiveUsersLimit
}

export interface AccountOrganizationsRevokedUser {
    organizationId: OrganizationID
    userId: UserID
    createdOn: DateTime
    revokedOn: DateTime
    currentProfile: UserProfile
}

export interface AsyncEnrollmentUntrusted {
    enrollmentId: AsyncEnrollmentID
    submittedOn: DateTime
    untrustedRequestedDeviceLabel: DeviceLabel
    untrustedRequestedHumanHandle: HumanHandle
    identitySystem: AsyncEnrollmentIdentitySystem
}

export interface AuthMethodInfo {
    authMethodId: AccountAuthMethodID
    createdOn: DateTime
    createdByIp: string
    createdByUserAgent: string
    usePassword: boolean
}

export interface AvailableDevice {
    keyFilePath: Path
    createdOn: DateTime
    protectedOn: DateTime
    serverAddr: ParsecAddr
    organizationId: OrganizationID
    userId: UserID
    deviceId: DeviceID
    humanHandle: HumanHandle
    deviceLabel: DeviceLabel
    ty: AvailableDeviceType
}

export interface AvailablePendingAsyncEnrollment {
    filePath: Path
    submittedOn: DateTime
    addr: ParsecAsyncEnrollmentAddr
    enrollmentId: AsyncEnrollmentID
    requestedDeviceLabel: DeviceLabel
    requestedHumanHandle: HumanHandle
    identitySystem: AvailablePendingAsyncEnrollmentIdentitySystem
}

export interface ClientConfig {
    configDir: Path
    dataBaseDir: Path
    mountpointMountStrategy: MountpointMountStrategy
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
    withMonitors: boolean
    preventSyncPattern: string | null
    logLevel: LogLevel | null
}

export interface ClientInfo {
    organizationAddr: ParsecOrganizationAddr
    organizationId: OrganizationID
    deviceId: DeviceID
    userId: UserID
    deviceLabel: DeviceLabel
    humanHandle: HumanHandle
    currentProfile: UserProfile
    serverOrganizationConfig: ServerOrganizationConfig
    isServerOnline: boolean
    isOrganizationExpired: boolean
    mustAcceptTos: boolean
}

export interface DeviceClaimFinalizeInfo {
    handle: Handle
}

export interface DeviceClaimInProgress1Info {
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface DeviceClaimInProgress2Info {
    handle: Handle
    claimerSas: SASCode
}

export interface DeviceClaimInProgress3Info {
    handle: Handle
}

export interface DeviceGreetInProgress1Info {
    handle: Handle
    greeterSas: SASCode
}

export interface DeviceGreetInProgress2Info {
    handle: Handle
    claimerSas: SASCode
    claimerSasChoices: Array<SASCode>
}

export interface DeviceGreetInProgress3Info {
    handle: Handle
}

export interface DeviceGreetInProgress4Info {
    handle: Handle
    requestedDeviceLabel: DeviceLabel
}

export interface DeviceGreetInitialInfo {
    handle: Handle
}

export interface DeviceInfo {
    id: DeviceID
    purpose: DevicePurpose
    deviceLabel: DeviceLabel
    createdOn: DateTime
    createdBy: DeviceID | null
}

export interface FileStat {
    id: VlobID
    created: DateTime
    updated: DateTime
    baseVersion: VersionInt
    isPlaceholder: boolean
    needSync: boolean
    size: SizeInt
}

export interface HumanHandle {
    email: EmailAddress
    label: string
}

export interface NewInvitationInfo {
    addr: ParsecInvitationAddr
    token: AccessToken
    emailSentStatus: InvitationEmailSentStatus
}

export interface OpenBaoConfig {
    serverUrl: string
    secret: OpenBaoSecretConfig
    transitMountPath: string
    auths: Array<OpenBaoAuthConfig>
}

export interface OpenOptions {
    read: boolean
    write: boolean
    truncate: boolean
    create: boolean
    createNew: boolean
}

export interface OrganizationInfo {
    totalBlockBytes: SizeInt
    totalMetadataBytes: SizeInt
}

export interface ServerConfig {
    account: AccountConfig
    organizationBootstrap: OrganizationBootstrapConfig
    openbao: OpenBaoConfig | null
}

export interface ServerOrganizationConfig {
    userProfileOutsiderAllowed: boolean
    activeUsersLimit: ActiveUsersLimit
}

export interface ShamirRecoveryClaimInProgress1Info {
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface ShamirRecoveryClaimInProgress2Info {
    handle: Handle
    claimerSas: SASCode
}

export interface ShamirRecoveryClaimInProgress3Info {
    handle: Handle
}

export interface ShamirRecoveryClaimInitialInfo {
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
}

export interface ShamirRecoveryClaimShareInfo {
    handle: Handle
}

export interface ShamirRecoveryGreetInProgress1Info {
    handle: Handle
    greeterSas: SASCode
}

export interface ShamirRecoveryGreetInProgress2Info {
    handle: Handle
    claimerSas: SASCode
    claimerSasChoices: Array<SASCode>
}

export interface ShamirRecoveryGreetInProgress3Info {
    handle: Handle
}

export interface ShamirRecoveryGreetInitialInfo {
    handle: Handle
}

export interface ShamirRecoveryRecipient {
    userId: UserID
    humanHandle: HumanHandle
    revokedOn: DateTime | null
    shares: NonZeroU8
    onlineStatus: UserOnlineStatus
}

export interface StartedWorkspaceInfo {
    client: Handle
    id: VlobID
    currentName: EntryName
    currentSelfRole: RealmRole
    mountpoints: Array<[Handle, Path]>
}

export interface Tos {
    perLocaleUrls: Map<string, string>
    updatedOn: DateTime
}

export interface UserClaimFinalizeInfo {
    handle: Handle
}

export interface UserClaimInProgress1Info {
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
    greeterSas: SASCode
    greeterSasChoices: Array<SASCode>
}

export interface UserClaimInProgress2Info {
    handle: Handle
    claimerSas: SASCode
}

export interface UserClaimInProgress3Info {
    handle: Handle
}

export interface UserClaimInitialInfo {
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
    onlineStatus: UserOnlineStatus
    lastGreetingAttemptJoinedOn: DateTime | null
}

export interface UserGreetInProgress1Info {
    handle: Handle
    greeterSas: SASCode
}

export interface UserGreetInProgress2Info {
    handle: Handle
    claimerSas: SASCode
    claimerSasChoices: Array<SASCode>
}

export interface UserGreetInProgress3Info {
    handle: Handle
}

export interface UserGreetInProgress4Info {
    handle: Handle
    requestedHumanHandle: HumanHandle
    requestedDeviceLabel: DeviceLabel
}

export interface UserGreetInitialInfo {
    handle: Handle
}

export interface UserGreetingAdministrator {
    userId: UserID
    humanHandle: HumanHandle
    onlineStatus: UserOnlineStatus
    lastGreetingAttemptJoinedOn: DateTime | null
}

export interface UserInfo {
    id: UserID
    humanHandle: HumanHandle
    currentProfile: UserProfile
    createdOn: DateTime
    createdBy: DeviceID | null
    revokedOn: DateTime | null
    revokedBy: DeviceID | null
}

export interface WorkspaceHistoryFileStat {
    id: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
    size: SizeInt
}

export interface WorkspaceInfo {
    id: VlobID
    currentName: EntryName
    currentSelfRole: RealmRole
    isStarted: boolean
    isBootstrapped: boolean
}

export interface WorkspaceUserAccessInfo {
    userId: UserID
    humanHandle: HumanHandle
    currentProfile: UserProfile
    currentRole: RealmRole
}

export interface X509CertificateReference {
    uris: Array<X509URIFlavorValue>
    hash: X509CertificateHash
}

export interface X509Pkcs11URI {
}

export interface X509WindowsCngURI {
    issuer: BytesVec
    serialNumber: BytesVec
}

// AcceptFinalizeAsyncEnrollmentIdentityStrategy
export enum AcceptFinalizeAsyncEnrollmentIdentityStrategyTag {
    OpenBao = 'AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao',
    PKI = 'AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI',
}

export interface AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao {
    tag: AcceptFinalizeAsyncEnrollmentIdentityStrategyTag.OpenBao
    openbaoServerUrl: string
    openbaoTransitMountPath: string
    openbaoSecretMountPath: string
    openbaoEntityId: string
    openbaoAuthToken: string
}
export interface AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI {
    tag: AcceptFinalizeAsyncEnrollmentIdentityStrategyTag.PKI
    certificateReference: X509CertificateReference
}
export type AcceptFinalizeAsyncEnrollmentIdentityStrategy =
  | AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao
  | AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI

// AccountAuthMethodStrategy
export enum AccountAuthMethodStrategyTag {
    MasterSecret = 'AccountAuthMethodStrategyMasterSecret',
    Password = 'AccountAuthMethodStrategyPassword',
}

export interface AccountAuthMethodStrategyMasterSecret {
    tag: AccountAuthMethodStrategyTag.MasterSecret
    masterSecret: KeyDerivation
}
export interface AccountAuthMethodStrategyPassword {
    tag: AccountAuthMethodStrategyTag.Password
    password: Password
}
export type AccountAuthMethodStrategy =
  | AccountAuthMethodStrategyMasterSecret
  | AccountAuthMethodStrategyPassword

// AccountConfig
export enum AccountConfigTag {
    Disabled = 'AccountConfigDisabled',
    EnabledWithVault = 'AccountConfigEnabledWithVault',
    EnabledWithoutVault = 'AccountConfigEnabledWithoutVault',
}

export interface AccountConfigDisabled {
    tag: AccountConfigTag.Disabled
}
export interface AccountConfigEnabledWithVault {
    tag: AccountConfigTag.EnabledWithVault
}
export interface AccountConfigEnabledWithoutVault {
    tag: AccountConfigTag.EnabledWithoutVault
}
export type AccountConfig =
  | AccountConfigDisabled
  | AccountConfigEnabledWithVault
  | AccountConfigEnabledWithoutVault

// AccountCreateAuthMethodError
export enum AccountCreateAuthMethodErrorTag {
    BadVaultKeyAccess = 'AccountCreateAuthMethodErrorBadVaultKeyAccess',
    Internal = 'AccountCreateAuthMethodErrorInternal',
    Offline = 'AccountCreateAuthMethodErrorOffline',
}

export interface AccountCreateAuthMethodErrorBadVaultKeyAccess {
    tag: AccountCreateAuthMethodErrorTag.BadVaultKeyAccess
    error: string
}
export interface AccountCreateAuthMethodErrorInternal {
    tag: AccountCreateAuthMethodErrorTag.Internal
    error: string
}
export interface AccountCreateAuthMethodErrorOffline {
    tag: AccountCreateAuthMethodErrorTag.Offline
    error: string
}
export type AccountCreateAuthMethodError =
  | AccountCreateAuthMethodErrorBadVaultKeyAccess
  | AccountCreateAuthMethodErrorInternal
  | AccountCreateAuthMethodErrorOffline

// AccountCreateError
export enum AccountCreateErrorTag {
    Internal = 'AccountCreateErrorInternal',
    InvalidValidationCode = 'AccountCreateErrorInvalidValidationCode',
    Offline = 'AccountCreateErrorOffline',
    SendValidationEmailRequired = 'AccountCreateErrorSendValidationEmailRequired',
}

export interface AccountCreateErrorInternal {
    tag: AccountCreateErrorTag.Internal
    error: string
}
export interface AccountCreateErrorInvalidValidationCode {
    tag: AccountCreateErrorTag.InvalidValidationCode
    error: string
}
export interface AccountCreateErrorOffline {
    tag: AccountCreateErrorTag.Offline
    error: string
}
export interface AccountCreateErrorSendValidationEmailRequired {
    tag: AccountCreateErrorTag.SendValidationEmailRequired
    error: string
}
export type AccountCreateError =
  | AccountCreateErrorInternal
  | AccountCreateErrorInvalidValidationCode
  | AccountCreateErrorOffline
  | AccountCreateErrorSendValidationEmailRequired

// AccountCreateRegistrationDeviceError
export enum AccountCreateRegistrationDeviceErrorTag {
    BadVaultKeyAccess = 'AccountCreateRegistrationDeviceErrorBadVaultKeyAccess',
    Internal = 'AccountCreateRegistrationDeviceErrorInternal',
    LoadDeviceDecryptionFailed = 'AccountCreateRegistrationDeviceErrorLoadDeviceDecryptionFailed',
    LoadDeviceInvalidData = 'AccountCreateRegistrationDeviceErrorLoadDeviceInvalidData',
    LoadDeviceInvalidPath = 'AccountCreateRegistrationDeviceErrorLoadDeviceInvalidPath',
    LoadDeviceTOTPDecryptionFailed = 'AccountCreateRegistrationDeviceErrorLoadDeviceTOTPDecryptionFailed',
    Offline = 'AccountCreateRegistrationDeviceErrorOffline',
    RemoteOpaqueKeyFetchFailed = 'AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchFailed',
    RemoteOpaqueKeyFetchOffline = 'AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchOffline',
    TimestampOutOfBallpark = 'AccountCreateRegistrationDeviceErrorTimestampOutOfBallpark',
}

export interface AccountCreateRegistrationDeviceErrorBadVaultKeyAccess {
    tag: AccountCreateRegistrationDeviceErrorTag.BadVaultKeyAccess
    error: string
}
export interface AccountCreateRegistrationDeviceErrorInternal {
    tag: AccountCreateRegistrationDeviceErrorTag.Internal
    error: string
}
export interface AccountCreateRegistrationDeviceErrorLoadDeviceDecryptionFailed {
    tag: AccountCreateRegistrationDeviceErrorTag.LoadDeviceDecryptionFailed
    error: string
}
export interface AccountCreateRegistrationDeviceErrorLoadDeviceInvalidData {
    tag: AccountCreateRegistrationDeviceErrorTag.LoadDeviceInvalidData
    error: string
}
export interface AccountCreateRegistrationDeviceErrorLoadDeviceInvalidPath {
    tag: AccountCreateRegistrationDeviceErrorTag.LoadDeviceInvalidPath
    error: string
}
export interface AccountCreateRegistrationDeviceErrorLoadDeviceTOTPDecryptionFailed {
    tag: AccountCreateRegistrationDeviceErrorTag.LoadDeviceTOTPDecryptionFailed
    error: string
}
export interface AccountCreateRegistrationDeviceErrorOffline {
    tag: AccountCreateRegistrationDeviceErrorTag.Offline
    error: string
}
export interface AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchFailed {
    tag: AccountCreateRegistrationDeviceErrorTag.RemoteOpaqueKeyFetchFailed
    error: string
}
export interface AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchOffline {
    tag: AccountCreateRegistrationDeviceErrorTag.RemoteOpaqueKeyFetchOffline
    error: string
}
export interface AccountCreateRegistrationDeviceErrorTimestampOutOfBallpark {
    tag: AccountCreateRegistrationDeviceErrorTag.TimestampOutOfBallpark
    error: string
}
export type AccountCreateRegistrationDeviceError =
  | AccountCreateRegistrationDeviceErrorBadVaultKeyAccess
  | AccountCreateRegistrationDeviceErrorInternal
  | AccountCreateRegistrationDeviceErrorLoadDeviceDecryptionFailed
  | AccountCreateRegistrationDeviceErrorLoadDeviceInvalidData
  | AccountCreateRegistrationDeviceErrorLoadDeviceInvalidPath
  | AccountCreateRegistrationDeviceErrorLoadDeviceTOTPDecryptionFailed
  | AccountCreateRegistrationDeviceErrorOffline
  | AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchFailed
  | AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchOffline
  | AccountCreateRegistrationDeviceErrorTimestampOutOfBallpark

// AccountCreateSendValidationEmailError
export enum AccountCreateSendValidationEmailErrorTag {
    EmailRecipientRefused = 'AccountCreateSendValidationEmailErrorEmailRecipientRefused',
    EmailSendingRateLimited = 'AccountCreateSendValidationEmailErrorEmailSendingRateLimited',
    EmailServerUnavailable = 'AccountCreateSendValidationEmailErrorEmailServerUnavailable',
    Internal = 'AccountCreateSendValidationEmailErrorInternal',
    Offline = 'AccountCreateSendValidationEmailErrorOffline',
}

export interface AccountCreateSendValidationEmailErrorEmailRecipientRefused {
    tag: AccountCreateSendValidationEmailErrorTag.EmailRecipientRefused
    error: string
}
export interface AccountCreateSendValidationEmailErrorEmailSendingRateLimited {
    tag: AccountCreateSendValidationEmailErrorTag.EmailSendingRateLimited
    error: string
    waitUntil: DateTime
}
export interface AccountCreateSendValidationEmailErrorEmailServerUnavailable {
    tag: AccountCreateSendValidationEmailErrorTag.EmailServerUnavailable
    error: string
}
export interface AccountCreateSendValidationEmailErrorInternal {
    tag: AccountCreateSendValidationEmailErrorTag.Internal
    error: string
}
export interface AccountCreateSendValidationEmailErrorOffline {
    tag: AccountCreateSendValidationEmailErrorTag.Offline
    error: string
}
export type AccountCreateSendValidationEmailError =
  | AccountCreateSendValidationEmailErrorEmailRecipientRefused
  | AccountCreateSendValidationEmailErrorEmailSendingRateLimited
  | AccountCreateSendValidationEmailErrorEmailServerUnavailable
  | AccountCreateSendValidationEmailErrorInternal
  | AccountCreateSendValidationEmailErrorOffline

// AccountDeleteProceedError
export enum AccountDeleteProceedErrorTag {
    Internal = 'AccountDeleteProceedErrorInternal',
    InvalidValidationCode = 'AccountDeleteProceedErrorInvalidValidationCode',
    Offline = 'AccountDeleteProceedErrorOffline',
    SendValidationEmailRequired = 'AccountDeleteProceedErrorSendValidationEmailRequired',
}

export interface AccountDeleteProceedErrorInternal {
    tag: AccountDeleteProceedErrorTag.Internal
    error: string
}
export interface AccountDeleteProceedErrorInvalidValidationCode {
    tag: AccountDeleteProceedErrorTag.InvalidValidationCode
    error: string
}
export interface AccountDeleteProceedErrorOffline {
    tag: AccountDeleteProceedErrorTag.Offline
    error: string
}
export interface AccountDeleteProceedErrorSendValidationEmailRequired {
    tag: AccountDeleteProceedErrorTag.SendValidationEmailRequired
    error: string
}
export type AccountDeleteProceedError =
  | AccountDeleteProceedErrorInternal
  | AccountDeleteProceedErrorInvalidValidationCode
  | AccountDeleteProceedErrorOffline
  | AccountDeleteProceedErrorSendValidationEmailRequired

// AccountDeleteSendValidationEmailError
export enum AccountDeleteSendValidationEmailErrorTag {
    EmailRecipientRefused = 'AccountDeleteSendValidationEmailErrorEmailRecipientRefused',
    EmailSendingRateLimited = 'AccountDeleteSendValidationEmailErrorEmailSendingRateLimited',
    EmailServerUnavailable = 'AccountDeleteSendValidationEmailErrorEmailServerUnavailable',
    Internal = 'AccountDeleteSendValidationEmailErrorInternal',
    Offline = 'AccountDeleteSendValidationEmailErrorOffline',
}

export interface AccountDeleteSendValidationEmailErrorEmailRecipientRefused {
    tag: AccountDeleteSendValidationEmailErrorTag.EmailRecipientRefused
    error: string
}
export interface AccountDeleteSendValidationEmailErrorEmailSendingRateLimited {
    tag: AccountDeleteSendValidationEmailErrorTag.EmailSendingRateLimited
    error: string
    waitUntil: DateTime
}
export interface AccountDeleteSendValidationEmailErrorEmailServerUnavailable {
    tag: AccountDeleteSendValidationEmailErrorTag.EmailServerUnavailable
    error: string
}
export interface AccountDeleteSendValidationEmailErrorInternal {
    tag: AccountDeleteSendValidationEmailErrorTag.Internal
    error: string
}
export interface AccountDeleteSendValidationEmailErrorOffline {
    tag: AccountDeleteSendValidationEmailErrorTag.Offline
    error: string
}
export type AccountDeleteSendValidationEmailError =
  | AccountDeleteSendValidationEmailErrorEmailRecipientRefused
  | AccountDeleteSendValidationEmailErrorEmailSendingRateLimited
  | AccountDeleteSendValidationEmailErrorEmailServerUnavailable
  | AccountDeleteSendValidationEmailErrorInternal
  | AccountDeleteSendValidationEmailErrorOffline

// AccountDisableAuthMethodError
export enum AccountDisableAuthMethodErrorTag {
    AuthMethodAlreadyDisabled = 'AccountDisableAuthMethodErrorAuthMethodAlreadyDisabled',
    AuthMethodNotFound = 'AccountDisableAuthMethodErrorAuthMethodNotFound',
    Internal = 'AccountDisableAuthMethodErrorInternal',
    Offline = 'AccountDisableAuthMethodErrorOffline',
    SelfDisableNotAllowed = 'AccountDisableAuthMethodErrorSelfDisableNotAllowed',
}

export interface AccountDisableAuthMethodErrorAuthMethodAlreadyDisabled {
    tag: AccountDisableAuthMethodErrorTag.AuthMethodAlreadyDisabled
    error: string
}
export interface AccountDisableAuthMethodErrorAuthMethodNotFound {
    tag: AccountDisableAuthMethodErrorTag.AuthMethodNotFound
    error: string
}
export interface AccountDisableAuthMethodErrorInternal {
    tag: AccountDisableAuthMethodErrorTag.Internal
    error: string
}
export interface AccountDisableAuthMethodErrorOffline {
    tag: AccountDisableAuthMethodErrorTag.Offline
    error: string
}
export interface AccountDisableAuthMethodErrorSelfDisableNotAllowed {
    tag: AccountDisableAuthMethodErrorTag.SelfDisableNotAllowed
    error: string
}
export type AccountDisableAuthMethodError =
  | AccountDisableAuthMethodErrorAuthMethodAlreadyDisabled
  | AccountDisableAuthMethodErrorAuthMethodNotFound
  | AccountDisableAuthMethodErrorInternal
  | AccountDisableAuthMethodErrorOffline
  | AccountDisableAuthMethodErrorSelfDisableNotAllowed

// AccountInfoError
export enum AccountInfoErrorTag {
    Internal = 'AccountInfoErrorInternal',
}

export interface AccountInfoErrorInternal {
    tag: AccountInfoErrorTag.Internal
    error: string
}
export type AccountInfoError =
  | AccountInfoErrorInternal

// AccountListAuthMethodsError
export enum AccountListAuthMethodsErrorTag {
    Internal = 'AccountListAuthMethodsErrorInternal',
    Offline = 'AccountListAuthMethodsErrorOffline',
}

export interface AccountListAuthMethodsErrorInternal {
    tag: AccountListAuthMethodsErrorTag.Internal
    error: string
}
export interface AccountListAuthMethodsErrorOffline {
    tag: AccountListAuthMethodsErrorTag.Offline
    error: string
}
export type AccountListAuthMethodsError =
  | AccountListAuthMethodsErrorInternal
  | AccountListAuthMethodsErrorOffline

// AccountListInvitationsError
export enum AccountListInvitationsErrorTag {
    Internal = 'AccountListInvitationsErrorInternal',
    Offline = 'AccountListInvitationsErrorOffline',
}

export interface AccountListInvitationsErrorInternal {
    tag: AccountListInvitationsErrorTag.Internal
    error: string
}
export interface AccountListInvitationsErrorOffline {
    tag: AccountListInvitationsErrorTag.Offline
    error: string
}
export type AccountListInvitationsError =
  | AccountListInvitationsErrorInternal
  | AccountListInvitationsErrorOffline

// AccountListOrganizationsError
export enum AccountListOrganizationsErrorTag {
    Internal = 'AccountListOrganizationsErrorInternal',
    Offline = 'AccountListOrganizationsErrorOffline',
}

export interface AccountListOrganizationsErrorInternal {
    tag: AccountListOrganizationsErrorTag.Internal
    error: string
}
export interface AccountListOrganizationsErrorOffline {
    tag: AccountListOrganizationsErrorTag.Offline
    error: string
}
export type AccountListOrganizationsError =
  | AccountListOrganizationsErrorInternal
  | AccountListOrganizationsErrorOffline

// AccountListRegistrationDevicesError
export enum AccountListRegistrationDevicesErrorTag {
    BadVaultKeyAccess = 'AccountListRegistrationDevicesErrorBadVaultKeyAccess',
    Internal = 'AccountListRegistrationDevicesErrorInternal',
    Offline = 'AccountListRegistrationDevicesErrorOffline',
}

export interface AccountListRegistrationDevicesErrorBadVaultKeyAccess {
    tag: AccountListRegistrationDevicesErrorTag.BadVaultKeyAccess
    error: string
}
export interface AccountListRegistrationDevicesErrorInternal {
    tag: AccountListRegistrationDevicesErrorTag.Internal
    error: string
}
export interface AccountListRegistrationDevicesErrorOffline {
    tag: AccountListRegistrationDevicesErrorTag.Offline
    error: string
}
export type AccountListRegistrationDevicesError =
  | AccountListRegistrationDevicesErrorBadVaultKeyAccess
  | AccountListRegistrationDevicesErrorInternal
  | AccountListRegistrationDevicesErrorOffline

// AccountLoginError
export enum AccountLoginErrorTag {
    BadPasswordAlgorithm = 'AccountLoginErrorBadPasswordAlgorithm',
    Internal = 'AccountLoginErrorInternal',
    Offline = 'AccountLoginErrorOffline',
}

export interface AccountLoginErrorBadPasswordAlgorithm {
    tag: AccountLoginErrorTag.BadPasswordAlgorithm
    error: string
}
export interface AccountLoginErrorInternal {
    tag: AccountLoginErrorTag.Internal
    error: string
}
export interface AccountLoginErrorOffline {
    tag: AccountLoginErrorTag.Offline
    error: string
}
export type AccountLoginError =
  | AccountLoginErrorBadPasswordAlgorithm
  | AccountLoginErrorInternal
  | AccountLoginErrorOffline

// AccountLoginStrategy
export enum AccountLoginStrategyTag {
    MasterSecret = 'AccountLoginStrategyMasterSecret',
    Password = 'AccountLoginStrategyPassword',
}

export interface AccountLoginStrategyMasterSecret {
    tag: AccountLoginStrategyTag.MasterSecret
    masterSecret: KeyDerivation
}
export interface AccountLoginStrategyPassword {
    tag: AccountLoginStrategyTag.Password
    email: EmailAddress
    password: Password
}
export type AccountLoginStrategy =
  | AccountLoginStrategyMasterSecret
  | AccountLoginStrategyPassword

// AccountLogoutError
export enum AccountLogoutErrorTag {
    Internal = 'AccountLogoutErrorInternal',
}

export interface AccountLogoutErrorInternal {
    tag: AccountLogoutErrorTag.Internal
    error: string
}
export type AccountLogoutError =
  | AccountLogoutErrorInternal

// AccountRecoverProceedError
export enum AccountRecoverProceedErrorTag {
    Internal = 'AccountRecoverProceedErrorInternal',
    InvalidValidationCode = 'AccountRecoverProceedErrorInvalidValidationCode',
    Offline = 'AccountRecoverProceedErrorOffline',
    SendValidationEmailRequired = 'AccountRecoverProceedErrorSendValidationEmailRequired',
}

export interface AccountRecoverProceedErrorInternal {
    tag: AccountRecoverProceedErrorTag.Internal
    error: string
}
export interface AccountRecoverProceedErrorInvalidValidationCode {
    tag: AccountRecoverProceedErrorTag.InvalidValidationCode
    error: string
}
export interface AccountRecoverProceedErrorOffline {
    tag: AccountRecoverProceedErrorTag.Offline
    error: string
}
export interface AccountRecoverProceedErrorSendValidationEmailRequired {
    tag: AccountRecoverProceedErrorTag.SendValidationEmailRequired
    error: string
}
export type AccountRecoverProceedError =
  | AccountRecoverProceedErrorInternal
  | AccountRecoverProceedErrorInvalidValidationCode
  | AccountRecoverProceedErrorOffline
  | AccountRecoverProceedErrorSendValidationEmailRequired

// AccountRecoverSendValidationEmailError
export enum AccountRecoverSendValidationEmailErrorTag {
    EmailRecipientRefused = 'AccountRecoverSendValidationEmailErrorEmailRecipientRefused',
    EmailSendingRateLimited = 'AccountRecoverSendValidationEmailErrorEmailSendingRateLimited',
    EmailServerUnavailable = 'AccountRecoverSendValidationEmailErrorEmailServerUnavailable',
    Internal = 'AccountRecoverSendValidationEmailErrorInternal',
    Offline = 'AccountRecoverSendValidationEmailErrorOffline',
}

export interface AccountRecoverSendValidationEmailErrorEmailRecipientRefused {
    tag: AccountRecoverSendValidationEmailErrorTag.EmailRecipientRefused
    error: string
}
export interface AccountRecoverSendValidationEmailErrorEmailSendingRateLimited {
    tag: AccountRecoverSendValidationEmailErrorTag.EmailSendingRateLimited
    error: string
    waitUntil: DateTime
}
export interface AccountRecoverSendValidationEmailErrorEmailServerUnavailable {
    tag: AccountRecoverSendValidationEmailErrorTag.EmailServerUnavailable
    error: string
}
export interface AccountRecoverSendValidationEmailErrorInternal {
    tag: AccountRecoverSendValidationEmailErrorTag.Internal
    error: string
}
export interface AccountRecoverSendValidationEmailErrorOffline {
    tag: AccountRecoverSendValidationEmailErrorTag.Offline
    error: string
}
export type AccountRecoverSendValidationEmailError =
  | AccountRecoverSendValidationEmailErrorEmailRecipientRefused
  | AccountRecoverSendValidationEmailErrorEmailSendingRateLimited
  | AccountRecoverSendValidationEmailErrorEmailServerUnavailable
  | AccountRecoverSendValidationEmailErrorInternal
  | AccountRecoverSendValidationEmailErrorOffline

// AccountRegisterNewDeviceError
export enum AccountRegisterNewDeviceErrorTag {
    BadVaultKeyAccess = 'AccountRegisterNewDeviceErrorBadVaultKeyAccess',
    CorruptedRegistrationDevice = 'AccountRegisterNewDeviceErrorCorruptedRegistrationDevice',
    Internal = 'AccountRegisterNewDeviceErrorInternal',
    InvalidPath = 'AccountRegisterNewDeviceErrorInvalidPath',
    NoSpaceAvailable = 'AccountRegisterNewDeviceErrorNoSpaceAvailable',
    Offline = 'AccountRegisterNewDeviceErrorOffline',
    RemoteOpaqueKeyUploadFailed = 'AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadFailed',
    RemoteOpaqueKeyUploadOffline = 'AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadOffline',
    TimestampOutOfBallpark = 'AccountRegisterNewDeviceErrorTimestampOutOfBallpark',
    UnknownRegistrationDevice = 'AccountRegisterNewDeviceErrorUnknownRegistrationDevice',
}

export interface AccountRegisterNewDeviceErrorBadVaultKeyAccess {
    tag: AccountRegisterNewDeviceErrorTag.BadVaultKeyAccess
    error: string
}
export interface AccountRegisterNewDeviceErrorCorruptedRegistrationDevice {
    tag: AccountRegisterNewDeviceErrorTag.CorruptedRegistrationDevice
    error: string
}
export interface AccountRegisterNewDeviceErrorInternal {
    tag: AccountRegisterNewDeviceErrorTag.Internal
    error: string
}
export interface AccountRegisterNewDeviceErrorInvalidPath {
    tag: AccountRegisterNewDeviceErrorTag.InvalidPath
    error: string
}
export interface AccountRegisterNewDeviceErrorNoSpaceAvailable {
    tag: AccountRegisterNewDeviceErrorTag.NoSpaceAvailable
    error: string
}
export interface AccountRegisterNewDeviceErrorOffline {
    tag: AccountRegisterNewDeviceErrorTag.Offline
    error: string
}
export interface AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadFailed {
    tag: AccountRegisterNewDeviceErrorTag.RemoteOpaqueKeyUploadFailed
    error: string
}
export interface AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadOffline {
    tag: AccountRegisterNewDeviceErrorTag.RemoteOpaqueKeyUploadOffline
    error: string
}
export interface AccountRegisterNewDeviceErrorTimestampOutOfBallpark {
    tag: AccountRegisterNewDeviceErrorTag.TimestampOutOfBallpark
    error: string
}
export interface AccountRegisterNewDeviceErrorUnknownRegistrationDevice {
    tag: AccountRegisterNewDeviceErrorTag.UnknownRegistrationDevice
    error: string
}
export type AccountRegisterNewDeviceError =
  | AccountRegisterNewDeviceErrorBadVaultKeyAccess
  | AccountRegisterNewDeviceErrorCorruptedRegistrationDevice
  | AccountRegisterNewDeviceErrorInternal
  | AccountRegisterNewDeviceErrorInvalidPath
  | AccountRegisterNewDeviceErrorNoSpaceAvailable
  | AccountRegisterNewDeviceErrorOffline
  | AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadFailed
  | AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadOffline
  | AccountRegisterNewDeviceErrorTimestampOutOfBallpark
  | AccountRegisterNewDeviceErrorUnknownRegistrationDevice

// ActiveUsersLimit
export enum ActiveUsersLimitTag {
    LimitedTo = 'ActiveUsersLimitLimitedTo',
    NoLimit = 'ActiveUsersLimitNoLimit',
}

export interface ActiveUsersLimitLimitedTo {
    tag: ActiveUsersLimitTag.LimitedTo
    x1: U64
}
export interface ActiveUsersLimitNoLimit {
    tag: ActiveUsersLimitTag.NoLimit
}
export type ActiveUsersLimit =
  | ActiveUsersLimitLimitedTo
  | ActiveUsersLimitNoLimit

// AnyClaimRetrievedInfo
export enum AnyClaimRetrievedInfoTag {
    Device = 'AnyClaimRetrievedInfoDevice',
    ShamirRecovery = 'AnyClaimRetrievedInfoShamirRecovery',
    User = 'AnyClaimRetrievedInfoUser',
}

export interface AnyClaimRetrievedInfoDevice {
    tag: AnyClaimRetrievedInfoTag.Device
    handle: Handle
    greeterUserId: UserID
    greeterHumanHandle: HumanHandle
}
export interface AnyClaimRetrievedInfoShamirRecovery {
    tag: AnyClaimRetrievedInfoTag.ShamirRecovery
    handle: Handle
    claimerUserId: UserID
    claimerHumanHandle: HumanHandle
    invitationCreatedBy: InviteInfoInvitationCreatedBy
    shamirRecoveryCreatedOn: DateTime
    recipients: Array<ShamirRecoveryRecipient>
    threshold: NonZeroU8
    isRecoverable: boolean
}
export interface AnyClaimRetrievedInfoUser {
    tag: AnyClaimRetrievedInfoTag.User
    handle: Handle
    claimerEmail: EmailAddress
    createdBy: InviteInfoInvitationCreatedBy
    administrators: Array<UserGreetingAdministrator>
    preferredGreeter: UserGreetingAdministrator | null
}
export type AnyClaimRetrievedInfo =
  | AnyClaimRetrievedInfoDevice
  | AnyClaimRetrievedInfoShamirRecovery
  | AnyClaimRetrievedInfoUser

// ArchiveDeviceError
export enum ArchiveDeviceErrorTag {
    Internal = 'ArchiveDeviceErrorInternal',
    NoSpaceAvailable = 'ArchiveDeviceErrorNoSpaceAvailable',
}

export interface ArchiveDeviceErrorInternal {
    tag: ArchiveDeviceErrorTag.Internal
    error: string
}
export interface ArchiveDeviceErrorNoSpaceAvailable {
    tag: ArchiveDeviceErrorTag.NoSpaceAvailable
    error: string
}
export type ArchiveDeviceError =
  | ArchiveDeviceErrorInternal
  | ArchiveDeviceErrorNoSpaceAvailable

// AsyncEnrollmentIdentitySystem
export enum AsyncEnrollmentIdentitySystemTag {
    OpenBao = 'AsyncEnrollmentIdentitySystemOpenBao',
    PKI = 'AsyncEnrollmentIdentitySystemPKI',
    PKICorrupted = 'AsyncEnrollmentIdentitySystemPKICorrupted',
}

export interface AsyncEnrollmentIdentitySystemOpenBao {
    tag: AsyncEnrollmentIdentitySystemTag.OpenBao
}
export interface AsyncEnrollmentIdentitySystemPKI {
    tag: AsyncEnrollmentIdentitySystemTag.PKI
    x509RootCertificateCommonName: string
    x509RootCertificateSubject: Uint8Array
}
export interface AsyncEnrollmentIdentitySystemPKICorrupted {
    tag: AsyncEnrollmentIdentitySystemTag.PKICorrupted
    reason: string
}
export type AsyncEnrollmentIdentitySystem =
  | AsyncEnrollmentIdentitySystemOpenBao
  | AsyncEnrollmentIdentitySystemPKI
  | AsyncEnrollmentIdentitySystemPKICorrupted

// AvailableDeviceType
export enum AvailableDeviceTypeTag {
    AccountVault = 'AvailableDeviceTypeAccountVault',
    Keyring = 'AvailableDeviceTypeKeyring',
    OpenBao = 'AvailableDeviceTypeOpenBao',
    PKI = 'AvailableDeviceTypePKI',
    Password = 'AvailableDeviceTypePassword',
    Recovery = 'AvailableDeviceTypeRecovery',
    TOTP = 'AvailableDeviceTypeTOTP',
}

export interface AvailableDeviceTypeAccountVault {
    tag: AvailableDeviceTypeTag.AccountVault
}
export interface AvailableDeviceTypeKeyring {
    tag: AvailableDeviceTypeTag.Keyring
}
export interface AvailableDeviceTypeOpenBao {
    tag: AvailableDeviceTypeTag.OpenBao
    openbaoPreferredAuthId: string
    openbaoEntityId: string
}
export interface AvailableDeviceTypePKI {
    tag: AvailableDeviceTypeTag.PKI
    certificateRef: X509CertificateReference
}
export interface AvailableDeviceTypePassword {
    tag: AvailableDeviceTypeTag.Password
}
export interface AvailableDeviceTypeRecovery {
    tag: AvailableDeviceTypeTag.Recovery
}
export interface AvailableDeviceTypeTOTP {
    tag: AvailableDeviceTypeTag.TOTP
    totpOpaqueKeyId: TOTPOpaqueKeyID
    next: AvailableDeviceType
}
export type AvailableDeviceType =
  | AvailableDeviceTypeAccountVault
  | AvailableDeviceTypeKeyring
  | AvailableDeviceTypeOpenBao
  | AvailableDeviceTypePKI
  | AvailableDeviceTypePassword
  | AvailableDeviceTypeRecovery
  | AvailableDeviceTypeTOTP

// AvailablePendingAsyncEnrollmentIdentitySystem
export enum AvailablePendingAsyncEnrollmentIdentitySystemTag {
    OpenBao = 'AvailablePendingAsyncEnrollmentIdentitySystemOpenBao',
    PKI = 'AvailablePendingAsyncEnrollmentIdentitySystemPKI',
}

export interface AvailablePendingAsyncEnrollmentIdentitySystemOpenBao {
    tag: AvailablePendingAsyncEnrollmentIdentitySystemTag.OpenBao
    openbaoEntityId: string
    openbaoPreferredAuthId: string
}
export interface AvailablePendingAsyncEnrollmentIdentitySystemPKI {
    tag: AvailablePendingAsyncEnrollmentIdentitySystemTag.PKI
    certificateRef: X509CertificateReference
}
export type AvailablePendingAsyncEnrollmentIdentitySystem =
  | AvailablePendingAsyncEnrollmentIdentitySystemOpenBao
  | AvailablePendingAsyncEnrollmentIdentitySystemPKI

// BootstrapOrganizationError
export enum BootstrapOrganizationErrorTag {
    AlreadyUsedToken = 'BootstrapOrganizationErrorAlreadyUsedToken',
    Internal = 'BootstrapOrganizationErrorInternal',
    InvalidSequesterAuthorityVerifyKey = 'BootstrapOrganizationErrorInvalidSequesterAuthorityVerifyKey',
    InvalidToken = 'BootstrapOrganizationErrorInvalidToken',
    Offline = 'BootstrapOrganizationErrorOffline',
    OrganizationExpired = 'BootstrapOrganizationErrorOrganizationExpired',
    SaveDeviceInvalidPath = 'BootstrapOrganizationErrorSaveDeviceInvalidPath',
    SaveDeviceNoSpaceAvailable = 'BootstrapOrganizationErrorSaveDeviceNoSpaceAvailable',
    SaveDeviceRemoteOpaqueKeyUploadFailed = 'BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadFailed',
    SaveDeviceRemoteOpaqueKeyUploadOffline = 'BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadOffline',
    TimestampOutOfBallpark = 'BootstrapOrganizationErrorTimestampOutOfBallpark',
}

export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: BootstrapOrganizationErrorTag.AlreadyUsedToken
    error: string
}
export interface BootstrapOrganizationErrorInternal {
    tag: BootstrapOrganizationErrorTag.Internal
    error: string
}
export interface BootstrapOrganizationErrorInvalidSequesterAuthorityVerifyKey {
    tag: BootstrapOrganizationErrorTag.InvalidSequesterAuthorityVerifyKey
    error: string
}
export interface BootstrapOrganizationErrorInvalidToken {
    tag: BootstrapOrganizationErrorTag.InvalidToken
    error: string
}
export interface BootstrapOrganizationErrorOffline {
    tag: BootstrapOrganizationErrorTag.Offline
    error: string
}
export interface BootstrapOrganizationErrorOrganizationExpired {
    tag: BootstrapOrganizationErrorTag.OrganizationExpired
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceInvalidPath {
    tag: BootstrapOrganizationErrorTag.SaveDeviceInvalidPath
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceNoSpaceAvailable {
    tag: BootstrapOrganizationErrorTag.SaveDeviceNoSpaceAvailable
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadFailed {
    tag: BootstrapOrganizationErrorTag.SaveDeviceRemoteOpaqueKeyUploadFailed
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadOffline {
    tag: BootstrapOrganizationErrorTag.SaveDeviceRemoteOpaqueKeyUploadOffline
    error: string
}
export interface BootstrapOrganizationErrorTimestampOutOfBallpark {
    tag: BootstrapOrganizationErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type BootstrapOrganizationError =
  | BootstrapOrganizationErrorAlreadyUsedToken
  | BootstrapOrganizationErrorInternal
  | BootstrapOrganizationErrorInvalidSequesterAuthorityVerifyKey
  | BootstrapOrganizationErrorInvalidToken
  | BootstrapOrganizationErrorOffline
  | BootstrapOrganizationErrorOrganizationExpired
  | BootstrapOrganizationErrorSaveDeviceInvalidPath
  | BootstrapOrganizationErrorSaveDeviceNoSpaceAvailable
  | BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadFailed
  | BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadOffline
  | BootstrapOrganizationErrorTimestampOutOfBallpark

// CancelError
export enum CancelErrorTag {
    Internal = 'CancelErrorInternal',
    NotBound = 'CancelErrorNotBound',
}

export interface CancelErrorInternal {
    tag: CancelErrorTag.Internal
    error: string
}
export interface CancelErrorNotBound {
    tag: CancelErrorTag.NotBound
    error: string
}
export type CancelError =
  | CancelErrorInternal
  | CancelErrorNotBound

// ClaimFinalizeError
export enum ClaimFinalizeErrorTag {
    Internal = 'ClaimFinalizeErrorInternal',
    InvalidPath = 'ClaimFinalizeErrorInvalidPath',
    NoSpaceAvailable = 'ClaimFinalizeErrorNoSpaceAvailable',
    RemoteOpaqueKeyUploadFailed = 'ClaimFinalizeErrorRemoteOpaqueKeyUploadFailed',
    RemoteOpaqueKeyUploadOffline = 'ClaimFinalizeErrorRemoteOpaqueKeyUploadOffline',
}

export interface ClaimFinalizeErrorInternal {
    tag: ClaimFinalizeErrorTag.Internal
    error: string
}
export interface ClaimFinalizeErrorInvalidPath {
    tag: ClaimFinalizeErrorTag.InvalidPath
    error: string
}
export interface ClaimFinalizeErrorNoSpaceAvailable {
    tag: ClaimFinalizeErrorTag.NoSpaceAvailable
    error: string
}
export interface ClaimFinalizeErrorRemoteOpaqueKeyUploadFailed {
    tag: ClaimFinalizeErrorTag.RemoteOpaqueKeyUploadFailed
    error: string
}
export interface ClaimFinalizeErrorRemoteOpaqueKeyUploadOffline {
    tag: ClaimFinalizeErrorTag.RemoteOpaqueKeyUploadOffline
    error: string
}
export type ClaimFinalizeError =
  | ClaimFinalizeErrorInternal
  | ClaimFinalizeErrorInvalidPath
  | ClaimFinalizeErrorNoSpaceAvailable
  | ClaimFinalizeErrorRemoteOpaqueKeyUploadFailed
  | ClaimFinalizeErrorRemoteOpaqueKeyUploadOffline

// ClaimInProgressError
export enum ClaimInProgressErrorTag {
    ActiveUsersLimitReached = 'ClaimInProgressErrorActiveUsersLimitReached',
    AlreadyUsedOrDeleted = 'ClaimInProgressErrorAlreadyUsedOrDeleted',
    Cancelled = 'ClaimInProgressErrorCancelled',
    CorruptedConfirmation = 'ClaimInProgressErrorCorruptedConfirmation',
    CorruptedSharedSecretKey = 'ClaimInProgressErrorCorruptedSharedSecretKey',
    GreeterNotAllowed = 'ClaimInProgressErrorGreeterNotAllowed',
    GreetingAttemptCancelled = 'ClaimInProgressErrorGreetingAttemptCancelled',
    Internal = 'ClaimInProgressErrorInternal',
    NotFound = 'ClaimInProgressErrorNotFound',
    Offline = 'ClaimInProgressErrorOffline',
    OrganizationExpired = 'ClaimInProgressErrorOrganizationExpired',
    PeerReset = 'ClaimInProgressErrorPeerReset',
}

export interface ClaimInProgressErrorActiveUsersLimitReached {
    tag: ClaimInProgressErrorTag.ActiveUsersLimitReached
    error: string
}
export interface ClaimInProgressErrorAlreadyUsedOrDeleted {
    tag: ClaimInProgressErrorTag.AlreadyUsedOrDeleted
    error: string
}
export interface ClaimInProgressErrorCancelled {
    tag: ClaimInProgressErrorTag.Cancelled
    error: string
}
export interface ClaimInProgressErrorCorruptedConfirmation {
    tag: ClaimInProgressErrorTag.CorruptedConfirmation
    error: string
}
export interface ClaimInProgressErrorCorruptedSharedSecretKey {
    tag: ClaimInProgressErrorTag.CorruptedSharedSecretKey
    error: string
}
export interface ClaimInProgressErrorGreeterNotAllowed {
    tag: ClaimInProgressErrorTag.GreeterNotAllowed
    error: string
}
export interface ClaimInProgressErrorGreetingAttemptCancelled {
    tag: ClaimInProgressErrorTag.GreetingAttemptCancelled
    error: string
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: DateTime
}
export interface ClaimInProgressErrorInternal {
    tag: ClaimInProgressErrorTag.Internal
    error: string
}
export interface ClaimInProgressErrorNotFound {
    tag: ClaimInProgressErrorTag.NotFound
    error: string
}
export interface ClaimInProgressErrorOffline {
    tag: ClaimInProgressErrorTag.Offline
    error: string
}
export interface ClaimInProgressErrorOrganizationExpired {
    tag: ClaimInProgressErrorTag.OrganizationExpired
    error: string
}
export interface ClaimInProgressErrorPeerReset {
    tag: ClaimInProgressErrorTag.PeerReset
    error: string
}
export type ClaimInProgressError =
  | ClaimInProgressErrorActiveUsersLimitReached
  | ClaimInProgressErrorAlreadyUsedOrDeleted
  | ClaimInProgressErrorCancelled
  | ClaimInProgressErrorCorruptedConfirmation
  | ClaimInProgressErrorCorruptedSharedSecretKey
  | ClaimInProgressErrorGreeterNotAllowed
  | ClaimInProgressErrorGreetingAttemptCancelled
  | ClaimInProgressErrorInternal
  | ClaimInProgressErrorNotFound
  | ClaimInProgressErrorOffline
  | ClaimInProgressErrorOrganizationExpired
  | ClaimInProgressErrorPeerReset

// ClaimerGreeterAbortOperationError
export enum ClaimerGreeterAbortOperationErrorTag {
    Internal = 'ClaimerGreeterAbortOperationErrorInternal',
}

export interface ClaimerGreeterAbortOperationErrorInternal {
    tag: ClaimerGreeterAbortOperationErrorTag.Internal
    error: string
}
export type ClaimerGreeterAbortOperationError =
  | ClaimerGreeterAbortOperationErrorInternal

// ClaimerRetrieveInfoError
export enum ClaimerRetrieveInfoErrorTag {
    AlreadyUsedOrDeleted = 'ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted',
    Internal = 'ClaimerRetrieveInfoErrorInternal',
    NotFound = 'ClaimerRetrieveInfoErrorNotFound',
    Offline = 'ClaimerRetrieveInfoErrorOffline',
    OrganizationExpired = 'ClaimerRetrieveInfoErrorOrganizationExpired',
}

export interface ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted {
    tag: ClaimerRetrieveInfoErrorTag.AlreadyUsedOrDeleted
    error: string
}
export interface ClaimerRetrieveInfoErrorInternal {
    tag: ClaimerRetrieveInfoErrorTag.Internal
    error: string
}
export interface ClaimerRetrieveInfoErrorNotFound {
    tag: ClaimerRetrieveInfoErrorTag.NotFound
    error: string
}
export interface ClaimerRetrieveInfoErrorOffline {
    tag: ClaimerRetrieveInfoErrorTag.Offline
    error: string
}
export interface ClaimerRetrieveInfoErrorOrganizationExpired {
    tag: ClaimerRetrieveInfoErrorTag.OrganizationExpired
    error: string
}
export type ClaimerRetrieveInfoError =
  | ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted
  | ClaimerRetrieveInfoErrorInternal
  | ClaimerRetrieveInfoErrorNotFound
  | ClaimerRetrieveInfoErrorOffline
  | ClaimerRetrieveInfoErrorOrganizationExpired

// ClientAcceptAsyncEnrollmentError
export enum ClientAcceptAsyncEnrollmentErrorTag {
    ActiveUsersLimitReached = 'ClientAcceptAsyncEnrollmentErrorActiveUsersLimitReached',
    AuthorNotAllowed = 'ClientAcceptAsyncEnrollmentErrorAuthorNotAllowed',
    BadSubmitPayload = 'ClientAcceptAsyncEnrollmentErrorBadSubmitPayload',
    EnrollmentNoLongerAvailable = 'ClientAcceptAsyncEnrollmentErrorEnrollmentNoLongerAvailable',
    EnrollmentNotFound = 'ClientAcceptAsyncEnrollmentErrorEnrollmentNotFound',
    HumanHandleAlreadyTaken = 'ClientAcceptAsyncEnrollmentErrorHumanHandleAlreadyTaken',
    IdentityStrategyMismatch = 'ClientAcceptAsyncEnrollmentErrorIdentityStrategyMismatch',
    Internal = 'ClientAcceptAsyncEnrollmentErrorInternal',
    Offline = 'ClientAcceptAsyncEnrollmentErrorOffline',
    OpenBaoBadServerResponse = 'ClientAcceptAsyncEnrollmentErrorOpenBaoBadServerResponse',
    OpenBaoBadURL = 'ClientAcceptAsyncEnrollmentErrorOpenBaoBadURL',
    OpenBaoNoServerResponse = 'ClientAcceptAsyncEnrollmentErrorOpenBaoNoServerResponse',
    PKICannotOpenCertificateStore = 'ClientAcceptAsyncEnrollmentErrorPKICannotOpenCertificateStore',
    PKIServerInvalidX509Trustchain = 'ClientAcceptAsyncEnrollmentErrorPKIServerInvalidX509Trustchain',
    PKIUnusableX509CertificateReference = 'ClientAcceptAsyncEnrollmentErrorPKIUnusableX509CertificateReference',
    TimestampOutOfBallpark = 'ClientAcceptAsyncEnrollmentErrorTimestampOutOfBallpark',
}

export interface ClientAcceptAsyncEnrollmentErrorActiveUsersLimitReached {
    tag: ClientAcceptAsyncEnrollmentErrorTag.ActiveUsersLimitReached
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorAuthorNotAllowed {
    tag: ClientAcceptAsyncEnrollmentErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorBadSubmitPayload {
    tag: ClientAcceptAsyncEnrollmentErrorTag.BadSubmitPayload
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorEnrollmentNoLongerAvailable {
    tag: ClientAcceptAsyncEnrollmentErrorTag.EnrollmentNoLongerAvailable
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorEnrollmentNotFound {
    tag: ClientAcceptAsyncEnrollmentErrorTag.EnrollmentNotFound
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorHumanHandleAlreadyTaken {
    tag: ClientAcceptAsyncEnrollmentErrorTag.HumanHandleAlreadyTaken
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorIdentityStrategyMismatch {
    tag: ClientAcceptAsyncEnrollmentErrorTag.IdentityStrategyMismatch
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorInternal {
    tag: ClientAcceptAsyncEnrollmentErrorTag.Internal
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorOffline {
    tag: ClientAcceptAsyncEnrollmentErrorTag.Offline
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorOpenBaoBadServerResponse {
    tag: ClientAcceptAsyncEnrollmentErrorTag.OpenBaoBadServerResponse
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorOpenBaoBadURL {
    tag: ClientAcceptAsyncEnrollmentErrorTag.OpenBaoBadURL
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorOpenBaoNoServerResponse {
    tag: ClientAcceptAsyncEnrollmentErrorTag.OpenBaoNoServerResponse
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorPKICannotOpenCertificateStore {
    tag: ClientAcceptAsyncEnrollmentErrorTag.PKICannotOpenCertificateStore
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorPKIServerInvalidX509Trustchain {
    tag: ClientAcceptAsyncEnrollmentErrorTag.PKIServerInvalidX509Trustchain
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorPKIUnusableX509CertificateReference {
    tag: ClientAcceptAsyncEnrollmentErrorTag.PKIUnusableX509CertificateReference
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorTimestampOutOfBallpark {
    tag: ClientAcceptAsyncEnrollmentErrorTag.TimestampOutOfBallpark
    error: string
}
export type ClientAcceptAsyncEnrollmentError =
  | ClientAcceptAsyncEnrollmentErrorActiveUsersLimitReached
  | ClientAcceptAsyncEnrollmentErrorAuthorNotAllowed
  | ClientAcceptAsyncEnrollmentErrorBadSubmitPayload
  | ClientAcceptAsyncEnrollmentErrorEnrollmentNoLongerAvailable
  | ClientAcceptAsyncEnrollmentErrorEnrollmentNotFound
  | ClientAcceptAsyncEnrollmentErrorHumanHandleAlreadyTaken
  | ClientAcceptAsyncEnrollmentErrorIdentityStrategyMismatch
  | ClientAcceptAsyncEnrollmentErrorInternal
  | ClientAcceptAsyncEnrollmentErrorOffline
  | ClientAcceptAsyncEnrollmentErrorOpenBaoBadServerResponse
  | ClientAcceptAsyncEnrollmentErrorOpenBaoBadURL
  | ClientAcceptAsyncEnrollmentErrorOpenBaoNoServerResponse
  | ClientAcceptAsyncEnrollmentErrorPKICannotOpenCertificateStore
  | ClientAcceptAsyncEnrollmentErrorPKIServerInvalidX509Trustchain
  | ClientAcceptAsyncEnrollmentErrorPKIUnusableX509CertificateReference
  | ClientAcceptAsyncEnrollmentErrorTimestampOutOfBallpark

// ClientAcceptTosError
export enum ClientAcceptTosErrorTag {
    Internal = 'ClientAcceptTosErrorInternal',
    NoTos = 'ClientAcceptTosErrorNoTos',
    Offline = 'ClientAcceptTosErrorOffline',
    TosMismatch = 'ClientAcceptTosErrorTosMismatch',
}

export interface ClientAcceptTosErrorInternal {
    tag: ClientAcceptTosErrorTag.Internal
    error: string
}
export interface ClientAcceptTosErrorNoTos {
    tag: ClientAcceptTosErrorTag.NoTos
    error: string
}
export interface ClientAcceptTosErrorOffline {
    tag: ClientAcceptTosErrorTag.Offline
    error: string
}
export interface ClientAcceptTosErrorTosMismatch {
    tag: ClientAcceptTosErrorTag.TosMismatch
    error: string
}
export type ClientAcceptTosError =
  | ClientAcceptTosErrorInternal
  | ClientAcceptTosErrorNoTos
  | ClientAcceptTosErrorOffline
  | ClientAcceptTosErrorTosMismatch

// ClientCancelInvitationError
export enum ClientCancelInvitationErrorTag {
    AlreadyCancelled = 'ClientCancelInvitationErrorAlreadyCancelled',
    Completed = 'ClientCancelInvitationErrorCompleted',
    Internal = 'ClientCancelInvitationErrorInternal',
    NotAllowed = 'ClientCancelInvitationErrorNotAllowed',
    NotFound = 'ClientCancelInvitationErrorNotFound',
    Offline = 'ClientCancelInvitationErrorOffline',
}

export interface ClientCancelInvitationErrorAlreadyCancelled {
    tag: ClientCancelInvitationErrorTag.AlreadyCancelled
    error: string
}
export interface ClientCancelInvitationErrorCompleted {
    tag: ClientCancelInvitationErrorTag.Completed
    error: string
}
export interface ClientCancelInvitationErrorInternal {
    tag: ClientCancelInvitationErrorTag.Internal
    error: string
}
export interface ClientCancelInvitationErrorNotAllowed {
    tag: ClientCancelInvitationErrorTag.NotAllowed
    error: string
}
export interface ClientCancelInvitationErrorNotFound {
    tag: ClientCancelInvitationErrorTag.NotFound
    error: string
}
export interface ClientCancelInvitationErrorOffline {
    tag: ClientCancelInvitationErrorTag.Offline
    error: string
}
export type ClientCancelInvitationError =
  | ClientCancelInvitationErrorAlreadyCancelled
  | ClientCancelInvitationErrorCompleted
  | ClientCancelInvitationErrorInternal
  | ClientCancelInvitationErrorNotAllowed
  | ClientCancelInvitationErrorNotFound
  | ClientCancelInvitationErrorOffline

// ClientCreateWorkspaceError
export enum ClientCreateWorkspaceErrorTag {
    Internal = 'ClientCreateWorkspaceErrorInternal',
    Stopped = 'ClientCreateWorkspaceErrorStopped',
}

export interface ClientCreateWorkspaceErrorInternal {
    tag: ClientCreateWorkspaceErrorTag.Internal
    error: string
}
export interface ClientCreateWorkspaceErrorStopped {
    tag: ClientCreateWorkspaceErrorTag.Stopped
    error: string
}
export type ClientCreateWorkspaceError =
  | ClientCreateWorkspaceErrorInternal
  | ClientCreateWorkspaceErrorStopped

// ClientDeleteShamirRecoveryError
export enum ClientDeleteShamirRecoveryErrorTag {
    Internal = 'ClientDeleteShamirRecoveryErrorInternal',
    InvalidCertificate = 'ClientDeleteShamirRecoveryErrorInvalidCertificate',
    Offline = 'ClientDeleteShamirRecoveryErrorOffline',
    Stopped = 'ClientDeleteShamirRecoveryErrorStopped',
    TimestampOutOfBallpark = 'ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark',
}

export interface ClientDeleteShamirRecoveryErrorInternal {
    tag: ClientDeleteShamirRecoveryErrorTag.Internal
    error: string
}
export interface ClientDeleteShamirRecoveryErrorInvalidCertificate {
    tag: ClientDeleteShamirRecoveryErrorTag.InvalidCertificate
    error: string
}
export interface ClientDeleteShamirRecoveryErrorOffline {
    tag: ClientDeleteShamirRecoveryErrorTag.Offline
    error: string
}
export interface ClientDeleteShamirRecoveryErrorStopped {
    tag: ClientDeleteShamirRecoveryErrorTag.Stopped
    error: string
}
export interface ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark {
    tag: ClientDeleteShamirRecoveryErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type ClientDeleteShamirRecoveryError =
  | ClientDeleteShamirRecoveryErrorInternal
  | ClientDeleteShamirRecoveryErrorInvalidCertificate
  | ClientDeleteShamirRecoveryErrorOffline
  | ClientDeleteShamirRecoveryErrorStopped
  | ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark

// ClientEvent
export enum ClientEventTag {
    AsyncEnrollmentUpdated = 'ClientEventAsyncEnrollmentUpdated',
    ClientErrorResponse = 'ClientEventClientErrorResponse',
    ClientStarted = 'ClientEventClientStarted',
    ClientStopped = 'ClientEventClientStopped',
    ExpiredOrganization = 'ClientEventExpiredOrganization',
    FrozenSelfUser = 'ClientEventFrozenSelfUser',
    GreetingAttemptCancelled = 'ClientEventGreetingAttemptCancelled',
    GreetingAttemptJoined = 'ClientEventGreetingAttemptJoined',
    GreetingAttemptReady = 'ClientEventGreetingAttemptReady',
    IncompatibleServer = 'ClientEventIncompatibleServer',
    InvalidCertificate = 'ClientEventInvalidCertificate',
    InvitationAlreadyUsedOrDeleted = 'ClientEventInvitationAlreadyUsedOrDeleted',
    InvitationChanged = 'ClientEventInvitationChanged',
    MustAcceptTos = 'ClientEventMustAcceptTos',
    Offline = 'ClientEventOffline',
    Online = 'ClientEventOnline',
    OrganizationNotFound = 'ClientEventOrganizationNotFound',
    Ping = 'ClientEventPing',
    RevokedSelfUser = 'ClientEventRevokedSelfUser',
    ServerConfigChanged = 'ClientEventServerConfigChanged',
    ServerInvalidResponseContent = 'ClientEventServerInvalidResponseContent',
    ServerInvalidResponseStatus = 'ClientEventServerInvalidResponseStatus',
    TooMuchDriftWithServerClock = 'ClientEventTooMuchDriftWithServerClock',
    WebClientNotAllowedByOrganization = 'ClientEventWebClientNotAllowedByOrganization',
    WorkspaceLocallyCreated = 'ClientEventWorkspaceLocallyCreated',
    WorkspaceOpsInboundSyncDone = 'ClientEventWorkspaceOpsInboundSyncDone',
    WorkspaceOpsOutboundSyncAborted = 'ClientEventWorkspaceOpsOutboundSyncAborted',
    WorkspaceOpsOutboundSyncDone = 'ClientEventWorkspaceOpsOutboundSyncDone',
    WorkspaceOpsOutboundSyncProgress = 'ClientEventWorkspaceOpsOutboundSyncProgress',
    WorkspaceOpsOutboundSyncStarted = 'ClientEventWorkspaceOpsOutboundSyncStarted',
    WorkspaceWatchedEntryChanged = 'ClientEventWorkspaceWatchedEntryChanged',
    WorkspacesSelfListChanged = 'ClientEventWorkspacesSelfListChanged',
}

export interface ClientEventAsyncEnrollmentUpdated {
    tag: ClientEventTag.AsyncEnrollmentUpdated
}
export interface ClientEventClientErrorResponse {
    tag: ClientEventTag.ClientErrorResponse
    errorType: string
}
export interface ClientEventClientStarted {
    tag: ClientEventTag.ClientStarted
    deviceId: DeviceID
}
export interface ClientEventClientStopped {
    tag: ClientEventTag.ClientStopped
    deviceId: DeviceID
}
export interface ClientEventExpiredOrganization {
    tag: ClientEventTag.ExpiredOrganization
}
export interface ClientEventFrozenSelfUser {
    tag: ClientEventTag.FrozenSelfUser
}
export interface ClientEventGreetingAttemptCancelled {
    tag: ClientEventTag.GreetingAttemptCancelled
    token: AccessToken
    greetingAttempt: GreetingAttemptID
}
export interface ClientEventGreetingAttemptJoined {
    tag: ClientEventTag.GreetingAttemptJoined
    token: AccessToken
    greetingAttempt: GreetingAttemptID
}
export interface ClientEventGreetingAttemptReady {
    tag: ClientEventTag.GreetingAttemptReady
    token: AccessToken
    greetingAttempt: GreetingAttemptID
}
export interface ClientEventIncompatibleServer {
    tag: ClientEventTag.IncompatibleServer
    apiVersion: ApiVersion
    supportedApiVersion: Array<ApiVersion>
}
export interface ClientEventInvalidCertificate {
    tag: ClientEventTag.InvalidCertificate
    detail: string
}
export interface ClientEventInvitationAlreadyUsedOrDeleted {
    tag: ClientEventTag.InvitationAlreadyUsedOrDeleted
}
export interface ClientEventInvitationChanged {
    tag: ClientEventTag.InvitationChanged
    token: AccessToken
    status: InvitationStatus
}
export interface ClientEventMustAcceptTos {
    tag: ClientEventTag.MustAcceptTos
}
export interface ClientEventOffline {
    tag: ClientEventTag.Offline
}
export interface ClientEventOnline {
    tag: ClientEventTag.Online
}
export interface ClientEventOrganizationNotFound {
    tag: ClientEventTag.OrganizationNotFound
}
export interface ClientEventPing {
    tag: ClientEventTag.Ping
    ping: string
}
export interface ClientEventRevokedSelfUser {
    tag: ClientEventTag.RevokedSelfUser
}
export interface ClientEventServerConfigChanged {
    tag: ClientEventTag.ServerConfigChanged
}
export interface ClientEventServerInvalidResponseContent {
    tag: ClientEventTag.ServerInvalidResponseContent
    protocolDecodeError: string
}
export interface ClientEventServerInvalidResponseStatus {
    tag: ClientEventTag.ServerInvalidResponseStatus
    statusCode: string
}
export interface ClientEventTooMuchDriftWithServerClock {
    tag: ClientEventTag.TooMuchDriftWithServerClock
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientEventWebClientNotAllowedByOrganization {
    tag: ClientEventTag.WebClientNotAllowedByOrganization
}
export interface ClientEventWorkspaceLocallyCreated {
    tag: ClientEventTag.WorkspaceLocallyCreated
}
export interface ClientEventWorkspaceOpsInboundSyncDone {
    tag: ClientEventTag.WorkspaceOpsInboundSyncDone
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspaceOpsOutboundSyncAborted {
    tag: ClientEventTag.WorkspaceOpsOutboundSyncAborted
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspaceOpsOutboundSyncDone {
    tag: ClientEventTag.WorkspaceOpsOutboundSyncDone
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspaceOpsOutboundSyncProgress {
    tag: ClientEventTag.WorkspaceOpsOutboundSyncProgress
    realmId: VlobID
    entryId: VlobID
    blocks: IndexInt
    blockIndex: IndexInt
    blocksize: SizeInt
}
export interface ClientEventWorkspaceOpsOutboundSyncStarted {
    tag: ClientEventTag.WorkspaceOpsOutboundSyncStarted
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspaceWatchedEntryChanged {
    tag: ClientEventTag.WorkspaceWatchedEntryChanged
    realmId: VlobID
    entryId: VlobID
}
export interface ClientEventWorkspacesSelfListChanged {
    tag: ClientEventTag.WorkspacesSelfListChanged
}
export type ClientEvent =
  | ClientEventAsyncEnrollmentUpdated
  | ClientEventClientErrorResponse
  | ClientEventClientStarted
  | ClientEventClientStopped
  | ClientEventExpiredOrganization
  | ClientEventFrozenSelfUser
  | ClientEventGreetingAttemptCancelled
  | ClientEventGreetingAttemptJoined
  | ClientEventGreetingAttemptReady
  | ClientEventIncompatibleServer
  | ClientEventInvalidCertificate
  | ClientEventInvitationAlreadyUsedOrDeleted
  | ClientEventInvitationChanged
  | ClientEventMustAcceptTos
  | ClientEventOffline
  | ClientEventOnline
  | ClientEventOrganizationNotFound
  | ClientEventPing
  | ClientEventRevokedSelfUser
  | ClientEventServerConfigChanged
  | ClientEventServerInvalidResponseContent
  | ClientEventServerInvalidResponseStatus
  | ClientEventTooMuchDriftWithServerClock
  | ClientEventWebClientNotAllowedByOrganization
  | ClientEventWorkspaceLocallyCreated
  | ClientEventWorkspaceOpsInboundSyncDone
  | ClientEventWorkspaceOpsOutboundSyncAborted
  | ClientEventWorkspaceOpsOutboundSyncDone
  | ClientEventWorkspaceOpsOutboundSyncProgress
  | ClientEventWorkspaceOpsOutboundSyncStarted
  | ClientEventWorkspaceWatchedEntryChanged
  | ClientEventWorkspacesSelfListChanged

// ClientExportRecoveryDeviceError
export enum ClientExportRecoveryDeviceErrorTag {
    Internal = 'ClientExportRecoveryDeviceErrorInternal',
    InvalidCertificate = 'ClientExportRecoveryDeviceErrorInvalidCertificate',
    Offline = 'ClientExportRecoveryDeviceErrorOffline',
    Stopped = 'ClientExportRecoveryDeviceErrorStopped',
    TimestampOutOfBallpark = 'ClientExportRecoveryDeviceErrorTimestampOutOfBallpark',
}

export interface ClientExportRecoveryDeviceErrorInternal {
    tag: ClientExportRecoveryDeviceErrorTag.Internal
    error: string
}
export interface ClientExportRecoveryDeviceErrorInvalidCertificate {
    tag: ClientExportRecoveryDeviceErrorTag.InvalidCertificate
    error: string
}
export interface ClientExportRecoveryDeviceErrorOffline {
    tag: ClientExportRecoveryDeviceErrorTag.Offline
    error: string
}
export interface ClientExportRecoveryDeviceErrorStopped {
    tag: ClientExportRecoveryDeviceErrorTag.Stopped
    error: string
}
export interface ClientExportRecoveryDeviceErrorTimestampOutOfBallpark {
    tag: ClientExportRecoveryDeviceErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type ClientExportRecoveryDeviceError =
  | ClientExportRecoveryDeviceErrorInternal
  | ClientExportRecoveryDeviceErrorInvalidCertificate
  | ClientExportRecoveryDeviceErrorOffline
  | ClientExportRecoveryDeviceErrorStopped
  | ClientExportRecoveryDeviceErrorTimestampOutOfBallpark

// ClientForgetAllCertificatesError
export enum ClientForgetAllCertificatesErrorTag {
    Internal = 'ClientForgetAllCertificatesErrorInternal',
    Stopped = 'ClientForgetAllCertificatesErrorStopped',
}

export interface ClientForgetAllCertificatesErrorInternal {
    tag: ClientForgetAllCertificatesErrorTag.Internal
    error: string
}
export interface ClientForgetAllCertificatesErrorStopped {
    tag: ClientForgetAllCertificatesErrorTag.Stopped
    error: string
}
export type ClientForgetAllCertificatesError =
  | ClientForgetAllCertificatesErrorInternal
  | ClientForgetAllCertificatesErrorStopped

// ClientGetAsyncEnrollmentAddrError
export enum ClientGetAsyncEnrollmentAddrErrorTag {
    Internal = 'ClientGetAsyncEnrollmentAddrErrorInternal',
}

export interface ClientGetAsyncEnrollmentAddrErrorInternal {
    tag: ClientGetAsyncEnrollmentAddrErrorTag.Internal
    error: string
}
export type ClientGetAsyncEnrollmentAddrError =
  | ClientGetAsyncEnrollmentAddrErrorInternal

// ClientGetOrganizationBootstrapDateError
export enum ClientGetOrganizationBootstrapDateErrorTag {
    BootstrapDateNotFound = 'ClientGetOrganizationBootstrapDateErrorBootstrapDateNotFound',
    Internal = 'ClientGetOrganizationBootstrapDateErrorInternal',
    InvalidCertificate = 'ClientGetOrganizationBootstrapDateErrorInvalidCertificate',
    Offline = 'ClientGetOrganizationBootstrapDateErrorOffline',
    Stopped = 'ClientGetOrganizationBootstrapDateErrorStopped',
}

export interface ClientGetOrganizationBootstrapDateErrorBootstrapDateNotFound {
    tag: ClientGetOrganizationBootstrapDateErrorTag.BootstrapDateNotFound
    error: string
}
export interface ClientGetOrganizationBootstrapDateErrorInternal {
    tag: ClientGetOrganizationBootstrapDateErrorTag.Internal
    error: string
}
export interface ClientGetOrganizationBootstrapDateErrorInvalidCertificate {
    tag: ClientGetOrganizationBootstrapDateErrorTag.InvalidCertificate
    error: string
}
export interface ClientGetOrganizationBootstrapDateErrorOffline {
    tag: ClientGetOrganizationBootstrapDateErrorTag.Offline
    error: string
}
export interface ClientGetOrganizationBootstrapDateErrorStopped {
    tag: ClientGetOrganizationBootstrapDateErrorTag.Stopped
    error: string
}
export type ClientGetOrganizationBootstrapDateError =
  | ClientGetOrganizationBootstrapDateErrorBootstrapDateNotFound
  | ClientGetOrganizationBootstrapDateErrorInternal
  | ClientGetOrganizationBootstrapDateErrorInvalidCertificate
  | ClientGetOrganizationBootstrapDateErrorOffline
  | ClientGetOrganizationBootstrapDateErrorStopped

// ClientGetSelfShamirRecoveryError
export enum ClientGetSelfShamirRecoveryErrorTag {
    Internal = 'ClientGetSelfShamirRecoveryErrorInternal',
    Stopped = 'ClientGetSelfShamirRecoveryErrorStopped',
}

export interface ClientGetSelfShamirRecoveryErrorInternal {
    tag: ClientGetSelfShamirRecoveryErrorTag.Internal
    error: string
}
export interface ClientGetSelfShamirRecoveryErrorStopped {
    tag: ClientGetSelfShamirRecoveryErrorTag.Stopped
    error: string
}
export type ClientGetSelfShamirRecoveryError =
  | ClientGetSelfShamirRecoveryErrorInternal
  | ClientGetSelfShamirRecoveryErrorStopped

// ClientGetTosError
export enum ClientGetTosErrorTag {
    Internal = 'ClientGetTosErrorInternal',
    NoTos = 'ClientGetTosErrorNoTos',
    Offline = 'ClientGetTosErrorOffline',
}

export interface ClientGetTosErrorInternal {
    tag: ClientGetTosErrorTag.Internal
    error: string
}
export interface ClientGetTosErrorNoTos {
    tag: ClientGetTosErrorTag.NoTos
    error: string
}
export interface ClientGetTosErrorOffline {
    tag: ClientGetTosErrorTag.Offline
    error: string
}
export type ClientGetTosError =
  | ClientGetTosErrorInternal
  | ClientGetTosErrorNoTos
  | ClientGetTosErrorOffline

// ClientGetUserDeviceError
export enum ClientGetUserDeviceErrorTag {
    Internal = 'ClientGetUserDeviceErrorInternal',
    NonExisting = 'ClientGetUserDeviceErrorNonExisting',
    Stopped = 'ClientGetUserDeviceErrorStopped',
}

export interface ClientGetUserDeviceErrorInternal {
    tag: ClientGetUserDeviceErrorTag.Internal
    error: string
}
export interface ClientGetUserDeviceErrorNonExisting {
    tag: ClientGetUserDeviceErrorTag.NonExisting
    error: string
}
export interface ClientGetUserDeviceErrorStopped {
    tag: ClientGetUserDeviceErrorTag.Stopped
    error: string
}
export type ClientGetUserDeviceError =
  | ClientGetUserDeviceErrorInternal
  | ClientGetUserDeviceErrorNonExisting
  | ClientGetUserDeviceErrorStopped

// ClientGetUserInfoError
export enum ClientGetUserInfoErrorTag {
    Internal = 'ClientGetUserInfoErrorInternal',
    NonExisting = 'ClientGetUserInfoErrorNonExisting',
    Stopped = 'ClientGetUserInfoErrorStopped',
}

export interface ClientGetUserInfoErrorInternal {
    tag: ClientGetUserInfoErrorTag.Internal
    error: string
}
export interface ClientGetUserInfoErrorNonExisting {
    tag: ClientGetUserInfoErrorTag.NonExisting
    error: string
}
export interface ClientGetUserInfoErrorStopped {
    tag: ClientGetUserInfoErrorTag.Stopped
    error: string
}
export type ClientGetUserInfoError =
  | ClientGetUserInfoErrorInternal
  | ClientGetUserInfoErrorNonExisting
  | ClientGetUserInfoErrorStopped

// ClientInfoError
export enum ClientInfoErrorTag {
    Internal = 'ClientInfoErrorInternal',
    Stopped = 'ClientInfoErrorStopped',
}

export interface ClientInfoErrorInternal {
    tag: ClientInfoErrorTag.Internal
    error: string
}
export interface ClientInfoErrorStopped {
    tag: ClientInfoErrorTag.Stopped
    error: string
}
export type ClientInfoError =
  | ClientInfoErrorInternal
  | ClientInfoErrorStopped

// ClientListAsyncEnrollmentsError
export enum ClientListAsyncEnrollmentsErrorTag {
    AuthorNotAllowed = 'ClientListAsyncEnrollmentsErrorAuthorNotAllowed',
    Internal = 'ClientListAsyncEnrollmentsErrorInternal',
    Offline = 'ClientListAsyncEnrollmentsErrorOffline',
}

export interface ClientListAsyncEnrollmentsErrorAuthorNotAllowed {
    tag: ClientListAsyncEnrollmentsErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientListAsyncEnrollmentsErrorInternal {
    tag: ClientListAsyncEnrollmentsErrorTag.Internal
    error: string
}
export interface ClientListAsyncEnrollmentsErrorOffline {
    tag: ClientListAsyncEnrollmentsErrorTag.Offline
    error: string
}
export type ClientListAsyncEnrollmentsError =
  | ClientListAsyncEnrollmentsErrorAuthorNotAllowed
  | ClientListAsyncEnrollmentsErrorInternal
  | ClientListAsyncEnrollmentsErrorOffline

// ClientListFrozenUsersError
export enum ClientListFrozenUsersErrorTag {
    AuthorNotAllowed = 'ClientListFrozenUsersErrorAuthorNotAllowed',
    Internal = 'ClientListFrozenUsersErrorInternal',
    Offline = 'ClientListFrozenUsersErrorOffline',
}

export interface ClientListFrozenUsersErrorAuthorNotAllowed {
    tag: ClientListFrozenUsersErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientListFrozenUsersErrorInternal {
    tag: ClientListFrozenUsersErrorTag.Internal
    error: string
}
export interface ClientListFrozenUsersErrorOffline {
    tag: ClientListFrozenUsersErrorTag.Offline
    error: string
}
export type ClientListFrozenUsersError =
  | ClientListFrozenUsersErrorAuthorNotAllowed
  | ClientListFrozenUsersErrorInternal
  | ClientListFrozenUsersErrorOffline

// ClientListShamirRecoveriesForOthersError
export enum ClientListShamirRecoveriesForOthersErrorTag {
    Internal = 'ClientListShamirRecoveriesForOthersErrorInternal',
    Stopped = 'ClientListShamirRecoveriesForOthersErrorStopped',
}

export interface ClientListShamirRecoveriesForOthersErrorInternal {
    tag: ClientListShamirRecoveriesForOthersErrorTag.Internal
    error: string
}
export interface ClientListShamirRecoveriesForOthersErrorStopped {
    tag: ClientListShamirRecoveriesForOthersErrorTag.Stopped
    error: string
}
export type ClientListShamirRecoveriesForOthersError =
  | ClientListShamirRecoveriesForOthersErrorInternal
  | ClientListShamirRecoveriesForOthersErrorStopped

// ClientListUserDevicesError
export enum ClientListUserDevicesErrorTag {
    Internal = 'ClientListUserDevicesErrorInternal',
    Stopped = 'ClientListUserDevicesErrorStopped',
}

export interface ClientListUserDevicesErrorInternal {
    tag: ClientListUserDevicesErrorTag.Internal
    error: string
}
export interface ClientListUserDevicesErrorStopped {
    tag: ClientListUserDevicesErrorTag.Stopped
    error: string
}
export type ClientListUserDevicesError =
  | ClientListUserDevicesErrorInternal
  | ClientListUserDevicesErrorStopped

// ClientListUsersError
export enum ClientListUsersErrorTag {
    Internal = 'ClientListUsersErrorInternal',
    Stopped = 'ClientListUsersErrorStopped',
}

export interface ClientListUsersErrorInternal {
    tag: ClientListUsersErrorTag.Internal
    error: string
}
export interface ClientListUsersErrorStopped {
    tag: ClientListUsersErrorTag.Stopped
    error: string
}
export type ClientListUsersError =
  | ClientListUsersErrorInternal
  | ClientListUsersErrorStopped

// ClientListWorkspaceUsersError
export enum ClientListWorkspaceUsersErrorTag {
    Internal = 'ClientListWorkspaceUsersErrorInternal',
    Stopped = 'ClientListWorkspaceUsersErrorStopped',
}

export interface ClientListWorkspaceUsersErrorInternal {
    tag: ClientListWorkspaceUsersErrorTag.Internal
    error: string
}
export interface ClientListWorkspaceUsersErrorStopped {
    tag: ClientListWorkspaceUsersErrorTag.Stopped
    error: string
}
export type ClientListWorkspaceUsersError =
  | ClientListWorkspaceUsersErrorInternal
  | ClientListWorkspaceUsersErrorStopped

// ClientListWorkspacesError
export enum ClientListWorkspacesErrorTag {
    Internal = 'ClientListWorkspacesErrorInternal',
}

export interface ClientListWorkspacesErrorInternal {
    tag: ClientListWorkspacesErrorTag.Internal
    error: string
}
export type ClientListWorkspacesError =
  | ClientListWorkspacesErrorInternal

// ClientNewDeviceInvitationError
export enum ClientNewDeviceInvitationErrorTag {
    Internal = 'ClientNewDeviceInvitationErrorInternal',
    Offline = 'ClientNewDeviceInvitationErrorOffline',
}

export interface ClientNewDeviceInvitationErrorInternal {
    tag: ClientNewDeviceInvitationErrorTag.Internal
    error: string
}
export interface ClientNewDeviceInvitationErrorOffline {
    tag: ClientNewDeviceInvitationErrorTag.Offline
    error: string
}
export type ClientNewDeviceInvitationError =
  | ClientNewDeviceInvitationErrorInternal
  | ClientNewDeviceInvitationErrorOffline

// ClientNewShamirRecoveryInvitationError
export enum ClientNewShamirRecoveryInvitationErrorTag {
    Internal = 'ClientNewShamirRecoveryInvitationErrorInternal',
    NotAllowed = 'ClientNewShamirRecoveryInvitationErrorNotAllowed',
    Offline = 'ClientNewShamirRecoveryInvitationErrorOffline',
    UserNotFound = 'ClientNewShamirRecoveryInvitationErrorUserNotFound',
}

export interface ClientNewShamirRecoveryInvitationErrorInternal {
    tag: ClientNewShamirRecoveryInvitationErrorTag.Internal
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorNotAllowed {
    tag: ClientNewShamirRecoveryInvitationErrorTag.NotAllowed
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorOffline {
    tag: ClientNewShamirRecoveryInvitationErrorTag.Offline
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorUserNotFound {
    tag: ClientNewShamirRecoveryInvitationErrorTag.UserNotFound
    error: string
}
export type ClientNewShamirRecoveryInvitationError =
  | ClientNewShamirRecoveryInvitationErrorInternal
  | ClientNewShamirRecoveryInvitationErrorNotAllowed
  | ClientNewShamirRecoveryInvitationErrorOffline
  | ClientNewShamirRecoveryInvitationErrorUserNotFound

// ClientNewUserInvitationError
export enum ClientNewUserInvitationErrorTag {
    AlreadyMember = 'ClientNewUserInvitationErrorAlreadyMember',
    Internal = 'ClientNewUserInvitationErrorInternal',
    NotAllowed = 'ClientNewUserInvitationErrorNotAllowed',
    Offline = 'ClientNewUserInvitationErrorOffline',
}

export interface ClientNewUserInvitationErrorAlreadyMember {
    tag: ClientNewUserInvitationErrorTag.AlreadyMember
    error: string
}
export interface ClientNewUserInvitationErrorInternal {
    tag: ClientNewUserInvitationErrorTag.Internal
    error: string
}
export interface ClientNewUserInvitationErrorNotAllowed {
    tag: ClientNewUserInvitationErrorTag.NotAllowed
    error: string
}
export interface ClientNewUserInvitationErrorOffline {
    tag: ClientNewUserInvitationErrorTag.Offline
    error: string
}
export type ClientNewUserInvitationError =
  | ClientNewUserInvitationErrorAlreadyMember
  | ClientNewUserInvitationErrorInternal
  | ClientNewUserInvitationErrorNotAllowed
  | ClientNewUserInvitationErrorOffline

// ClientOrganizationInfoError
export enum ClientOrganizationInfoErrorTag {
    Internal = 'ClientOrganizationInfoErrorInternal',
    Offline = 'ClientOrganizationInfoErrorOffline',
}

export interface ClientOrganizationInfoErrorInternal {
    tag: ClientOrganizationInfoErrorTag.Internal
    error: string
}
export interface ClientOrganizationInfoErrorOffline {
    tag: ClientOrganizationInfoErrorTag.Offline
    error: string
}
export type ClientOrganizationInfoError =
  | ClientOrganizationInfoErrorInternal
  | ClientOrganizationInfoErrorOffline

// ClientRejectAsyncEnrollmentError
export enum ClientRejectAsyncEnrollmentErrorTag {
    AuthorNotAllowed = 'ClientRejectAsyncEnrollmentErrorAuthorNotAllowed',
    EnrollmentNoLongerAvailable = 'ClientRejectAsyncEnrollmentErrorEnrollmentNoLongerAvailable',
    EnrollmentNotFound = 'ClientRejectAsyncEnrollmentErrorEnrollmentNotFound',
    Internal = 'ClientRejectAsyncEnrollmentErrorInternal',
    Offline = 'ClientRejectAsyncEnrollmentErrorOffline',
}

export interface ClientRejectAsyncEnrollmentErrorAuthorNotAllowed {
    tag: ClientRejectAsyncEnrollmentErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientRejectAsyncEnrollmentErrorEnrollmentNoLongerAvailable {
    tag: ClientRejectAsyncEnrollmentErrorTag.EnrollmentNoLongerAvailable
    error: string
}
export interface ClientRejectAsyncEnrollmentErrorEnrollmentNotFound {
    tag: ClientRejectAsyncEnrollmentErrorTag.EnrollmentNotFound
    error: string
}
export interface ClientRejectAsyncEnrollmentErrorInternal {
    tag: ClientRejectAsyncEnrollmentErrorTag.Internal
    error: string
}
export interface ClientRejectAsyncEnrollmentErrorOffline {
    tag: ClientRejectAsyncEnrollmentErrorTag.Offline
    error: string
}
export type ClientRejectAsyncEnrollmentError =
  | ClientRejectAsyncEnrollmentErrorAuthorNotAllowed
  | ClientRejectAsyncEnrollmentErrorEnrollmentNoLongerAvailable
  | ClientRejectAsyncEnrollmentErrorEnrollmentNotFound
  | ClientRejectAsyncEnrollmentErrorInternal
  | ClientRejectAsyncEnrollmentErrorOffline

// ClientRenameWorkspaceError
export enum ClientRenameWorkspaceErrorTag {
    AuthorNotAllowed = 'ClientRenameWorkspaceErrorAuthorNotAllowed',
    Internal = 'ClientRenameWorkspaceErrorInternal',
    InvalidCertificate = 'ClientRenameWorkspaceErrorInvalidCertificate',
    InvalidEncryptedRealmName = 'ClientRenameWorkspaceErrorInvalidEncryptedRealmName',
    InvalidKeysBundle = 'ClientRenameWorkspaceErrorInvalidKeysBundle',
    NoKey = 'ClientRenameWorkspaceErrorNoKey',
    Offline = 'ClientRenameWorkspaceErrorOffline',
    Stopped = 'ClientRenameWorkspaceErrorStopped',
    TimestampOutOfBallpark = 'ClientRenameWorkspaceErrorTimestampOutOfBallpark',
    WorkspaceNotFound = 'ClientRenameWorkspaceErrorWorkspaceNotFound',
}

export interface ClientRenameWorkspaceErrorAuthorNotAllowed {
    tag: ClientRenameWorkspaceErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientRenameWorkspaceErrorInternal {
    tag: ClientRenameWorkspaceErrorTag.Internal
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidCertificate {
    tag: ClientRenameWorkspaceErrorTag.InvalidCertificate
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidEncryptedRealmName {
    tag: ClientRenameWorkspaceErrorTag.InvalidEncryptedRealmName
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidKeysBundle {
    tag: ClientRenameWorkspaceErrorTag.InvalidKeysBundle
    error: string
}
export interface ClientRenameWorkspaceErrorNoKey {
    tag: ClientRenameWorkspaceErrorTag.NoKey
    error: string
}
export interface ClientRenameWorkspaceErrorOffline {
    tag: ClientRenameWorkspaceErrorTag.Offline
    error: string
}
export interface ClientRenameWorkspaceErrorStopped {
    tag: ClientRenameWorkspaceErrorTag.Stopped
    error: string
}
export interface ClientRenameWorkspaceErrorTimestampOutOfBallpark {
    tag: ClientRenameWorkspaceErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientRenameWorkspaceErrorWorkspaceNotFound {
    tag: ClientRenameWorkspaceErrorTag.WorkspaceNotFound
    error: string
}
export type ClientRenameWorkspaceError =
  | ClientRenameWorkspaceErrorAuthorNotAllowed
  | ClientRenameWorkspaceErrorInternal
  | ClientRenameWorkspaceErrorInvalidCertificate
  | ClientRenameWorkspaceErrorInvalidEncryptedRealmName
  | ClientRenameWorkspaceErrorInvalidKeysBundle
  | ClientRenameWorkspaceErrorNoKey
  | ClientRenameWorkspaceErrorOffline
  | ClientRenameWorkspaceErrorStopped
  | ClientRenameWorkspaceErrorTimestampOutOfBallpark
  | ClientRenameWorkspaceErrorWorkspaceNotFound

// ClientRevokeUserError
export enum ClientRevokeUserErrorTag {
    AuthorNotAllowed = 'ClientRevokeUserErrorAuthorNotAllowed',
    Internal = 'ClientRevokeUserErrorInternal',
    InvalidCertificate = 'ClientRevokeUserErrorInvalidCertificate',
    InvalidKeysBundle = 'ClientRevokeUserErrorInvalidKeysBundle',
    NoKey = 'ClientRevokeUserErrorNoKey',
    Offline = 'ClientRevokeUserErrorOffline',
    Stopped = 'ClientRevokeUserErrorStopped',
    TimestampOutOfBallpark = 'ClientRevokeUserErrorTimestampOutOfBallpark',
    UserIsSelf = 'ClientRevokeUserErrorUserIsSelf',
    UserNotFound = 'ClientRevokeUserErrorUserNotFound',
}

export interface ClientRevokeUserErrorAuthorNotAllowed {
    tag: ClientRevokeUserErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientRevokeUserErrorInternal {
    tag: ClientRevokeUserErrorTag.Internal
    error: string
}
export interface ClientRevokeUserErrorInvalidCertificate {
    tag: ClientRevokeUserErrorTag.InvalidCertificate
    error: string
}
export interface ClientRevokeUserErrorInvalidKeysBundle {
    tag: ClientRevokeUserErrorTag.InvalidKeysBundle
    error: string
}
export interface ClientRevokeUserErrorNoKey {
    tag: ClientRevokeUserErrorTag.NoKey
    error: string
}
export interface ClientRevokeUserErrorOffline {
    tag: ClientRevokeUserErrorTag.Offline
    error: string
}
export interface ClientRevokeUserErrorStopped {
    tag: ClientRevokeUserErrorTag.Stopped
    error: string
}
export interface ClientRevokeUserErrorTimestampOutOfBallpark {
    tag: ClientRevokeUserErrorTag.TimestampOutOfBallpark
    error: string
}
export interface ClientRevokeUserErrorUserIsSelf {
    tag: ClientRevokeUserErrorTag.UserIsSelf
    error: string
}
export interface ClientRevokeUserErrorUserNotFound {
    tag: ClientRevokeUserErrorTag.UserNotFound
    error: string
}
export type ClientRevokeUserError =
  | ClientRevokeUserErrorAuthorNotAllowed
  | ClientRevokeUserErrorInternal
  | ClientRevokeUserErrorInvalidCertificate
  | ClientRevokeUserErrorInvalidKeysBundle
  | ClientRevokeUserErrorNoKey
  | ClientRevokeUserErrorOffline
  | ClientRevokeUserErrorStopped
  | ClientRevokeUserErrorTimestampOutOfBallpark
  | ClientRevokeUserErrorUserIsSelf
  | ClientRevokeUserErrorUserNotFound

// ClientSetupShamirRecoveryError
export enum ClientSetupShamirRecoveryErrorTag {
    AuthorAmongRecipients = 'ClientSetupShamirRecoveryErrorAuthorAmongRecipients',
    Internal = 'ClientSetupShamirRecoveryErrorInternal',
    InvalidCertificate = 'ClientSetupShamirRecoveryErrorInvalidCertificate',
    Offline = 'ClientSetupShamirRecoveryErrorOffline',
    RecipientNotFound = 'ClientSetupShamirRecoveryErrorRecipientNotFound',
    RecipientRevoked = 'ClientSetupShamirRecoveryErrorRecipientRevoked',
    ShamirRecoveryAlreadyExists = 'ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists',
    Stopped = 'ClientSetupShamirRecoveryErrorStopped',
    ThresholdBiggerThanSumOfShares = 'ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares',
    TimestampOutOfBallpark = 'ClientSetupShamirRecoveryErrorTimestampOutOfBallpark',
    TooManyShares = 'ClientSetupShamirRecoveryErrorTooManyShares',
}

export interface ClientSetupShamirRecoveryErrorAuthorAmongRecipients {
    tag: ClientSetupShamirRecoveryErrorTag.AuthorAmongRecipients
    error: string
}
export interface ClientSetupShamirRecoveryErrorInternal {
    tag: ClientSetupShamirRecoveryErrorTag.Internal
    error: string
}
export interface ClientSetupShamirRecoveryErrorInvalidCertificate {
    tag: ClientSetupShamirRecoveryErrorTag.InvalidCertificate
    error: string
}
export interface ClientSetupShamirRecoveryErrorOffline {
    tag: ClientSetupShamirRecoveryErrorTag.Offline
    error: string
}
export interface ClientSetupShamirRecoveryErrorRecipientNotFound {
    tag: ClientSetupShamirRecoveryErrorTag.RecipientNotFound
    error: string
}
export interface ClientSetupShamirRecoveryErrorRecipientRevoked {
    tag: ClientSetupShamirRecoveryErrorTag.RecipientRevoked
    error: string
}
export interface ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists {
    tag: ClientSetupShamirRecoveryErrorTag.ShamirRecoveryAlreadyExists
    error: string
}
export interface ClientSetupShamirRecoveryErrorStopped {
    tag: ClientSetupShamirRecoveryErrorTag.Stopped
    error: string
}
export interface ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares {
    tag: ClientSetupShamirRecoveryErrorTag.ThresholdBiggerThanSumOfShares
    error: string
}
export interface ClientSetupShamirRecoveryErrorTimestampOutOfBallpark {
    tag: ClientSetupShamirRecoveryErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientSetupShamirRecoveryErrorTooManyShares {
    tag: ClientSetupShamirRecoveryErrorTag.TooManyShares
    error: string
}
export type ClientSetupShamirRecoveryError =
  | ClientSetupShamirRecoveryErrorAuthorAmongRecipients
  | ClientSetupShamirRecoveryErrorInternal
  | ClientSetupShamirRecoveryErrorInvalidCertificate
  | ClientSetupShamirRecoveryErrorOffline
  | ClientSetupShamirRecoveryErrorRecipientNotFound
  | ClientSetupShamirRecoveryErrorRecipientRevoked
  | ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists
  | ClientSetupShamirRecoveryErrorStopped
  | ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares
  | ClientSetupShamirRecoveryErrorTimestampOutOfBallpark
  | ClientSetupShamirRecoveryErrorTooManyShares

// ClientShareWorkspaceError
export enum ClientShareWorkspaceErrorTag {
    AuthorNotAllowed = 'ClientShareWorkspaceErrorAuthorNotAllowed',
    Internal = 'ClientShareWorkspaceErrorInternal',
    InvalidCertificate = 'ClientShareWorkspaceErrorInvalidCertificate',
    InvalidKeysBundle = 'ClientShareWorkspaceErrorInvalidKeysBundle',
    Offline = 'ClientShareWorkspaceErrorOffline',
    RecipientIsSelf = 'ClientShareWorkspaceErrorRecipientIsSelf',
    RecipientNotFound = 'ClientShareWorkspaceErrorRecipientNotFound',
    RecipientRevoked = 'ClientShareWorkspaceErrorRecipientRevoked',
    RoleIncompatibleWithOutsider = 'ClientShareWorkspaceErrorRoleIncompatibleWithOutsider',
    Stopped = 'ClientShareWorkspaceErrorStopped',
    TimestampOutOfBallpark = 'ClientShareWorkspaceErrorTimestampOutOfBallpark',
    WorkspaceNotFound = 'ClientShareWorkspaceErrorWorkspaceNotFound',
}

export interface ClientShareWorkspaceErrorAuthorNotAllowed {
    tag: ClientShareWorkspaceErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientShareWorkspaceErrorInternal {
    tag: ClientShareWorkspaceErrorTag.Internal
    error: string
}
export interface ClientShareWorkspaceErrorInvalidCertificate {
    tag: ClientShareWorkspaceErrorTag.InvalidCertificate
    error: string
}
export interface ClientShareWorkspaceErrorInvalidKeysBundle {
    tag: ClientShareWorkspaceErrorTag.InvalidKeysBundle
    error: string
}
export interface ClientShareWorkspaceErrorOffline {
    tag: ClientShareWorkspaceErrorTag.Offline
    error: string
}
export interface ClientShareWorkspaceErrorRecipientIsSelf {
    tag: ClientShareWorkspaceErrorTag.RecipientIsSelf
    error: string
}
export interface ClientShareWorkspaceErrorRecipientNotFound {
    tag: ClientShareWorkspaceErrorTag.RecipientNotFound
    error: string
}
export interface ClientShareWorkspaceErrorRecipientRevoked {
    tag: ClientShareWorkspaceErrorTag.RecipientRevoked
    error: string
}
export interface ClientShareWorkspaceErrorRoleIncompatibleWithOutsider {
    tag: ClientShareWorkspaceErrorTag.RoleIncompatibleWithOutsider
    error: string
}
export interface ClientShareWorkspaceErrorStopped {
    tag: ClientShareWorkspaceErrorTag.Stopped
    error: string
}
export interface ClientShareWorkspaceErrorTimestampOutOfBallpark {
    tag: ClientShareWorkspaceErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface ClientShareWorkspaceErrorWorkspaceNotFound {
    tag: ClientShareWorkspaceErrorTag.WorkspaceNotFound
    error: string
}
export type ClientShareWorkspaceError =
  | ClientShareWorkspaceErrorAuthorNotAllowed
  | ClientShareWorkspaceErrorInternal
  | ClientShareWorkspaceErrorInvalidCertificate
  | ClientShareWorkspaceErrorInvalidKeysBundle
  | ClientShareWorkspaceErrorOffline
  | ClientShareWorkspaceErrorRecipientIsSelf
  | ClientShareWorkspaceErrorRecipientNotFound
  | ClientShareWorkspaceErrorRecipientRevoked
  | ClientShareWorkspaceErrorRoleIncompatibleWithOutsider
  | ClientShareWorkspaceErrorStopped
  | ClientShareWorkspaceErrorTimestampOutOfBallpark
  | ClientShareWorkspaceErrorWorkspaceNotFound

// ClientStartError
export enum ClientStartErrorTag {
    DeviceUsedByAnotherProcess = 'ClientStartErrorDeviceUsedByAnotherProcess',
    Internal = 'ClientStartErrorInternal',
    LoadDeviceDecryptionFailed = 'ClientStartErrorLoadDeviceDecryptionFailed',
    LoadDeviceInvalidData = 'ClientStartErrorLoadDeviceInvalidData',
    LoadDeviceInvalidPath = 'ClientStartErrorLoadDeviceInvalidPath',
    LoadDeviceRemoteOpaqueKeyFetchFailed = 'ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchFailed',
    LoadDeviceRemoteOpaqueKeyFetchOffline = 'ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchOffline',
    LoadDeviceTOTPDecryptionFailed = 'ClientStartErrorLoadDeviceTOTPDecryptionFailed',
}

export interface ClientStartErrorDeviceUsedByAnotherProcess {
    tag: ClientStartErrorTag.DeviceUsedByAnotherProcess
    error: string
}
export interface ClientStartErrorInternal {
    tag: ClientStartErrorTag.Internal
    error: string
}
export interface ClientStartErrorLoadDeviceDecryptionFailed {
    tag: ClientStartErrorTag.LoadDeviceDecryptionFailed
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidData {
    tag: ClientStartErrorTag.LoadDeviceInvalidData
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidPath {
    tag: ClientStartErrorTag.LoadDeviceInvalidPath
    error: string
}
export interface ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchFailed {
    tag: ClientStartErrorTag.LoadDeviceRemoteOpaqueKeyFetchFailed
    error: string
}
export interface ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchOffline {
    tag: ClientStartErrorTag.LoadDeviceRemoteOpaqueKeyFetchOffline
    error: string
}
export interface ClientStartErrorLoadDeviceTOTPDecryptionFailed {
    tag: ClientStartErrorTag.LoadDeviceTOTPDecryptionFailed
    error: string
}
export type ClientStartError =
  | ClientStartErrorDeviceUsedByAnotherProcess
  | ClientStartErrorInternal
  | ClientStartErrorLoadDeviceDecryptionFailed
  | ClientStartErrorLoadDeviceInvalidData
  | ClientStartErrorLoadDeviceInvalidPath
  | ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchFailed
  | ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchOffline
  | ClientStartErrorLoadDeviceTOTPDecryptionFailed

// ClientStartInvitationGreetError
export enum ClientStartInvitationGreetErrorTag {
    Internal = 'ClientStartInvitationGreetErrorInternal',
}

export interface ClientStartInvitationGreetErrorInternal {
    tag: ClientStartInvitationGreetErrorTag.Internal
    error: string
}
export type ClientStartInvitationGreetError =
  | ClientStartInvitationGreetErrorInternal

// ClientStartShamirRecoveryInvitationGreetError
export enum ClientStartShamirRecoveryInvitationGreetErrorTag {
    CorruptedShareData = 'ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData',
    Internal = 'ClientStartShamirRecoveryInvitationGreetErrorInternal',
    InvalidCertificate = 'ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate',
    InvitationNotFound = 'ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound',
    Offline = 'ClientStartShamirRecoveryInvitationGreetErrorOffline',
    ShamirRecoveryDeleted = 'ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted',
    ShamirRecoveryNotFound = 'ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound',
    ShamirRecoveryUnusable = 'ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable',
    Stopped = 'ClientStartShamirRecoveryInvitationGreetErrorStopped',
}

export interface ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.CorruptedShareData
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInternal {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.Internal
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.InvalidCertificate
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.InvitationNotFound
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorOffline {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.Offline
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.ShamirRecoveryDeleted
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.ShamirRecoveryNotFound
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.ShamirRecoveryUnusable
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorStopped {
    tag: ClientStartShamirRecoveryInvitationGreetErrorTag.Stopped
    error: string
}
export type ClientStartShamirRecoveryInvitationGreetError =
  | ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData
  | ClientStartShamirRecoveryInvitationGreetErrorInternal
  | ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate
  | ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound
  | ClientStartShamirRecoveryInvitationGreetErrorOffline
  | ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted
  | ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound
  | ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable
  | ClientStartShamirRecoveryInvitationGreetErrorStopped

// ClientStartWorkspaceError
export enum ClientStartWorkspaceErrorTag {
    Internal = 'ClientStartWorkspaceErrorInternal',
    WorkspaceNotFound = 'ClientStartWorkspaceErrorWorkspaceNotFound',
}

export interface ClientStartWorkspaceErrorInternal {
    tag: ClientStartWorkspaceErrorTag.Internal
    error: string
}
export interface ClientStartWorkspaceErrorWorkspaceNotFound {
    tag: ClientStartWorkspaceErrorTag.WorkspaceNotFound
    error: string
}
export type ClientStartWorkspaceError =
  | ClientStartWorkspaceErrorInternal
  | ClientStartWorkspaceErrorWorkspaceNotFound

// ClientStopError
export enum ClientStopErrorTag {
    Internal = 'ClientStopErrorInternal',
}

export interface ClientStopErrorInternal {
    tag: ClientStopErrorTag.Internal
    error: string
}
export type ClientStopError =
  | ClientStopErrorInternal

// ClientTOTPSetupConfirmError
export enum ClientTOTPSetupConfirmErrorTag {
    AlreadySetup = 'ClientTOTPSetupConfirmErrorAlreadySetup',
    Internal = 'ClientTOTPSetupConfirmErrorInternal',
    InvalidOneTimePassword = 'ClientTOTPSetupConfirmErrorInvalidOneTimePassword',
    Offline = 'ClientTOTPSetupConfirmErrorOffline',
}

export interface ClientTOTPSetupConfirmErrorAlreadySetup {
    tag: ClientTOTPSetupConfirmErrorTag.AlreadySetup
    error: string
}
export interface ClientTOTPSetupConfirmErrorInternal {
    tag: ClientTOTPSetupConfirmErrorTag.Internal
    error: string
}
export interface ClientTOTPSetupConfirmErrorInvalidOneTimePassword {
    tag: ClientTOTPSetupConfirmErrorTag.InvalidOneTimePassword
    error: string
}
export interface ClientTOTPSetupConfirmErrorOffline {
    tag: ClientTOTPSetupConfirmErrorTag.Offline
    error: string
}
export type ClientTOTPSetupConfirmError =
  | ClientTOTPSetupConfirmErrorAlreadySetup
  | ClientTOTPSetupConfirmErrorInternal
  | ClientTOTPSetupConfirmErrorInvalidOneTimePassword
  | ClientTOTPSetupConfirmErrorOffline

// ClientTotpCreateOpaqueKeyError
export enum ClientTotpCreateOpaqueKeyErrorTag {
    Internal = 'ClientTotpCreateOpaqueKeyErrorInternal',
    Offline = 'ClientTotpCreateOpaqueKeyErrorOffline',
}

export interface ClientTotpCreateOpaqueKeyErrorInternal {
    tag: ClientTotpCreateOpaqueKeyErrorTag.Internal
    error: string
}
export interface ClientTotpCreateOpaqueKeyErrorOffline {
    tag: ClientTotpCreateOpaqueKeyErrorTag.Offline
    error: string
}
export type ClientTotpCreateOpaqueKeyError =
  | ClientTotpCreateOpaqueKeyErrorInternal
  | ClientTotpCreateOpaqueKeyErrorOffline

// ClientTotpSetupStatusError
export enum ClientTotpSetupStatusErrorTag {
    Internal = 'ClientTotpSetupStatusErrorInternal',
    Offline = 'ClientTotpSetupStatusErrorOffline',
}

export interface ClientTotpSetupStatusErrorInternal {
    tag: ClientTotpSetupStatusErrorTag.Internal
    error: string
}
export interface ClientTotpSetupStatusErrorOffline {
    tag: ClientTotpSetupStatusErrorTag.Offline
    error: string
}
export type ClientTotpSetupStatusError =
  | ClientTotpSetupStatusErrorInternal
  | ClientTotpSetupStatusErrorOffline

// ClientUserUpdateProfileError
export enum ClientUserUpdateProfileErrorTag {
    AuthorNotAllowed = 'ClientUserUpdateProfileErrorAuthorNotAllowed',
    Internal = 'ClientUserUpdateProfileErrorInternal',
    InvalidCertificate = 'ClientUserUpdateProfileErrorInvalidCertificate',
    Offline = 'ClientUserUpdateProfileErrorOffline',
    Stopped = 'ClientUserUpdateProfileErrorStopped',
    TimestampOutOfBallpark = 'ClientUserUpdateProfileErrorTimestampOutOfBallpark',
    UserIsSelf = 'ClientUserUpdateProfileErrorUserIsSelf',
    UserNotFound = 'ClientUserUpdateProfileErrorUserNotFound',
    UserRevoked = 'ClientUserUpdateProfileErrorUserRevoked',
}

export interface ClientUserUpdateProfileErrorAuthorNotAllowed {
    tag: ClientUserUpdateProfileErrorTag.AuthorNotAllowed
    error: string
}
export interface ClientUserUpdateProfileErrorInternal {
    tag: ClientUserUpdateProfileErrorTag.Internal
    error: string
}
export interface ClientUserUpdateProfileErrorInvalidCertificate {
    tag: ClientUserUpdateProfileErrorTag.InvalidCertificate
    error: string
}
export interface ClientUserUpdateProfileErrorOffline {
    tag: ClientUserUpdateProfileErrorTag.Offline
    error: string
}
export interface ClientUserUpdateProfileErrorStopped {
    tag: ClientUserUpdateProfileErrorTag.Stopped
    error: string
}
export interface ClientUserUpdateProfileErrorTimestampOutOfBallpark {
    tag: ClientUserUpdateProfileErrorTag.TimestampOutOfBallpark
    error: string
}
export interface ClientUserUpdateProfileErrorUserIsSelf {
    tag: ClientUserUpdateProfileErrorTag.UserIsSelf
    error: string
}
export interface ClientUserUpdateProfileErrorUserNotFound {
    tag: ClientUserUpdateProfileErrorTag.UserNotFound
    error: string
}
export interface ClientUserUpdateProfileErrorUserRevoked {
    tag: ClientUserUpdateProfileErrorTag.UserRevoked
    error: string
}
export type ClientUserUpdateProfileError =
  | ClientUserUpdateProfileErrorAuthorNotAllowed
  | ClientUserUpdateProfileErrorInternal
  | ClientUserUpdateProfileErrorInvalidCertificate
  | ClientUserUpdateProfileErrorOffline
  | ClientUserUpdateProfileErrorStopped
  | ClientUserUpdateProfileErrorTimestampOutOfBallpark
  | ClientUserUpdateProfileErrorUserIsSelf
  | ClientUserUpdateProfileErrorUserNotFound
  | ClientUserUpdateProfileErrorUserRevoked

// DeviceAccessStrategy
export enum DeviceAccessStrategyTag {
    AccountVault = 'DeviceAccessStrategyAccountVault',
    Keyring = 'DeviceAccessStrategyKeyring',
    OpenBao = 'DeviceAccessStrategyOpenBao',
    PKI = 'DeviceAccessStrategyPKI',
    Password = 'DeviceAccessStrategyPassword',
    TOTP = 'DeviceAccessStrategyTOTP',
}

export interface DeviceAccessStrategyAccountVault {
    tag: DeviceAccessStrategyTag.AccountVault
    keyFile: Path
    accountHandle: Handle
}
export interface DeviceAccessStrategyKeyring {
    tag: DeviceAccessStrategyTag.Keyring
    keyFile: Path
}
export interface DeviceAccessStrategyOpenBao {
    tag: DeviceAccessStrategyTag.OpenBao
    keyFile: Path
    openbaoServerUrl: string
    openbaoSecretMountPath: string
    openbaoTransitMountPath: string
    openbaoEntityId: string
    openbaoAuthToken: string
}
export interface DeviceAccessStrategyPKI {
    tag: DeviceAccessStrategyTag.PKI
    keyFile: Path
}
export interface DeviceAccessStrategyPassword {
    tag: DeviceAccessStrategyTag.Password
    keyFile: Path
    password: Password
}
export interface DeviceAccessStrategyTOTP {
    tag: DeviceAccessStrategyTag.TOTP
    totpOpaqueKey: SecretKey
    next: DeviceAccessStrategy
}
export type DeviceAccessStrategy =
  | DeviceAccessStrategyAccountVault
  | DeviceAccessStrategyKeyring
  | DeviceAccessStrategyOpenBao
  | DeviceAccessStrategyPKI
  | DeviceAccessStrategyPassword
  | DeviceAccessStrategyTOTP

// DeviceSaveStrategy
export enum DeviceSaveStrategyTag {
    AccountVault = 'DeviceSaveStrategyAccountVault',
    Keyring = 'DeviceSaveStrategyKeyring',
    OpenBao = 'DeviceSaveStrategyOpenBao',
    PKI = 'DeviceSaveStrategyPKI',
    Password = 'DeviceSaveStrategyPassword',
    TOTP = 'DeviceSaveStrategyTOTP',
}

export interface DeviceSaveStrategyAccountVault {
    tag: DeviceSaveStrategyTag.AccountVault
    accountHandle: Handle
}
export interface DeviceSaveStrategyKeyring {
    tag: DeviceSaveStrategyTag.Keyring
}
export interface DeviceSaveStrategyOpenBao {
    tag: DeviceSaveStrategyTag.OpenBao
    openbaoServerUrl: string
    openbaoSecretMountPath: string
    openbaoTransitMountPath: string
    openbaoEntityId: string
    openbaoAuthToken: string
    openbaoPreferredAuthId: string
}
export interface DeviceSaveStrategyPKI {
    tag: DeviceSaveStrategyTag.PKI
    certificateRef: X509CertificateReference
}
export interface DeviceSaveStrategyPassword {
    tag: DeviceSaveStrategyTag.Password
    password: Password
}
export interface DeviceSaveStrategyTOTP {
    tag: DeviceSaveStrategyTag.TOTP
    totpOpaqueKeyId: TOTPOpaqueKeyID
    totpOpaqueKey: SecretKey
    next: DeviceSaveStrategy
}
export type DeviceSaveStrategy =
  | DeviceSaveStrategyAccountVault
  | DeviceSaveStrategyKeyring
  | DeviceSaveStrategyOpenBao
  | DeviceSaveStrategyPKI
  | DeviceSaveStrategyPassword
  | DeviceSaveStrategyTOTP

// EntryStat
export enum EntryStatTag {
    File = 'EntryStatFile',
    Folder = 'EntryStatFolder',
}

export interface EntryStatFile {
    tag: EntryStatTag.File
    confinementPoint: VlobID | null
    id: VlobID
    parent: VlobID
    created: DateTime
    updated: DateTime
    baseVersion: VersionInt
    isPlaceholder: boolean
    needSync: boolean
    size: SizeInt
    lastUpdater: DeviceID
}
export interface EntryStatFolder {
    tag: EntryStatTag.Folder
    confinementPoint: VlobID | null
    id: VlobID
    parent: VlobID
    created: DateTime
    updated: DateTime
    baseVersion: VersionInt
    isPlaceholder: boolean
    needSync: boolean
    lastUpdater: DeviceID
}
export type EntryStat =
  | EntryStatFile
  | EntryStatFolder

// GetServerConfigError
export enum GetServerConfigErrorTag {
    Internal = 'GetServerConfigErrorInternal',
    Offline = 'GetServerConfigErrorOffline',
}

export interface GetServerConfigErrorInternal {
    tag: GetServerConfigErrorTag.Internal
    error: string
}
export interface GetServerConfigErrorOffline {
    tag: GetServerConfigErrorTag.Offline
    error: string
}
export type GetServerConfigError =
  | GetServerConfigErrorInternal
  | GetServerConfigErrorOffline

// GreetInProgressError
export enum GreetInProgressErrorTag {
    ActiveUsersLimitReached = 'GreetInProgressErrorActiveUsersLimitReached',
    AlreadyDeleted = 'GreetInProgressErrorAlreadyDeleted',
    Cancelled = 'GreetInProgressErrorCancelled',
    CorruptedInviteUserData = 'GreetInProgressErrorCorruptedInviteUserData',
    CorruptedSharedSecretKey = 'GreetInProgressErrorCorruptedSharedSecretKey',
    DeviceAlreadyExists = 'GreetInProgressErrorDeviceAlreadyExists',
    GreeterNotAllowed = 'GreetInProgressErrorGreeterNotAllowed',
    GreetingAttemptCancelled = 'GreetInProgressErrorGreetingAttemptCancelled',
    HumanHandleAlreadyTaken = 'GreetInProgressErrorHumanHandleAlreadyTaken',
    Internal = 'GreetInProgressErrorInternal',
    NonceMismatch = 'GreetInProgressErrorNonceMismatch',
    NotFound = 'GreetInProgressErrorNotFound',
    Offline = 'GreetInProgressErrorOffline',
    PeerReset = 'GreetInProgressErrorPeerReset',
    TimestampOutOfBallpark = 'GreetInProgressErrorTimestampOutOfBallpark',
    UserAlreadyExists = 'GreetInProgressErrorUserAlreadyExists',
    UserCreateNotAllowed = 'GreetInProgressErrorUserCreateNotAllowed',
}

export interface GreetInProgressErrorActiveUsersLimitReached {
    tag: GreetInProgressErrorTag.ActiveUsersLimitReached
    error: string
}
export interface GreetInProgressErrorAlreadyDeleted {
    tag: GreetInProgressErrorTag.AlreadyDeleted
    error: string
}
export interface GreetInProgressErrorCancelled {
    tag: GreetInProgressErrorTag.Cancelled
    error: string
}
export interface GreetInProgressErrorCorruptedInviteUserData {
    tag: GreetInProgressErrorTag.CorruptedInviteUserData
    error: string
}
export interface GreetInProgressErrorCorruptedSharedSecretKey {
    tag: GreetInProgressErrorTag.CorruptedSharedSecretKey
    error: string
}
export interface GreetInProgressErrorDeviceAlreadyExists {
    tag: GreetInProgressErrorTag.DeviceAlreadyExists
    error: string
}
export interface GreetInProgressErrorGreeterNotAllowed {
    tag: GreetInProgressErrorTag.GreeterNotAllowed
    error: string
}
export interface GreetInProgressErrorGreetingAttemptCancelled {
    tag: GreetInProgressErrorTag.GreetingAttemptCancelled
    error: string
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: DateTime
}
export interface GreetInProgressErrorHumanHandleAlreadyTaken {
    tag: GreetInProgressErrorTag.HumanHandleAlreadyTaken
    error: string
}
export interface GreetInProgressErrorInternal {
    tag: GreetInProgressErrorTag.Internal
    error: string
}
export interface GreetInProgressErrorNonceMismatch {
    tag: GreetInProgressErrorTag.NonceMismatch
    error: string
}
export interface GreetInProgressErrorNotFound {
    tag: GreetInProgressErrorTag.NotFound
    error: string
}
export interface GreetInProgressErrorOffline {
    tag: GreetInProgressErrorTag.Offline
    error: string
}
export interface GreetInProgressErrorPeerReset {
    tag: GreetInProgressErrorTag.PeerReset
    error: string
}
export interface GreetInProgressErrorTimestampOutOfBallpark {
    tag: GreetInProgressErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export interface GreetInProgressErrorUserAlreadyExists {
    tag: GreetInProgressErrorTag.UserAlreadyExists
    error: string
}
export interface GreetInProgressErrorUserCreateNotAllowed {
    tag: GreetInProgressErrorTag.UserCreateNotAllowed
    error: string
}
export type GreetInProgressError =
  | GreetInProgressErrorActiveUsersLimitReached
  | GreetInProgressErrorAlreadyDeleted
  | GreetInProgressErrorCancelled
  | GreetInProgressErrorCorruptedInviteUserData
  | GreetInProgressErrorCorruptedSharedSecretKey
  | GreetInProgressErrorDeviceAlreadyExists
  | GreetInProgressErrorGreeterNotAllowed
  | GreetInProgressErrorGreetingAttemptCancelled
  | GreetInProgressErrorHumanHandleAlreadyTaken
  | GreetInProgressErrorInternal
  | GreetInProgressErrorNonceMismatch
  | GreetInProgressErrorNotFound
  | GreetInProgressErrorOffline
  | GreetInProgressErrorPeerReset
  | GreetInProgressErrorTimestampOutOfBallpark
  | GreetInProgressErrorUserAlreadyExists
  | GreetInProgressErrorUserCreateNotAllowed

// ImportRecoveryDeviceError
export enum ImportRecoveryDeviceErrorTag {
    DecryptionFailed = 'ImportRecoveryDeviceErrorDecryptionFailed',
    Internal = 'ImportRecoveryDeviceErrorInternal',
    InvalidCertificate = 'ImportRecoveryDeviceErrorInvalidCertificate',
    InvalidData = 'ImportRecoveryDeviceErrorInvalidData',
    InvalidPassphrase = 'ImportRecoveryDeviceErrorInvalidPassphrase',
    InvalidPath = 'ImportRecoveryDeviceErrorInvalidPath',
    NoSpaceAvailable = 'ImportRecoveryDeviceErrorNoSpaceAvailable',
    Offline = 'ImportRecoveryDeviceErrorOffline',
    RemoteOpaqueKeyUploadFailed = 'ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadFailed',
    RemoteOpaqueKeyUploadOffline = 'ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadOffline',
    Stopped = 'ImportRecoveryDeviceErrorStopped',
    TimestampOutOfBallpark = 'ImportRecoveryDeviceErrorTimestampOutOfBallpark',
}

export interface ImportRecoveryDeviceErrorDecryptionFailed {
    tag: ImportRecoveryDeviceErrorTag.DecryptionFailed
    error: string
}
export interface ImportRecoveryDeviceErrorInternal {
    tag: ImportRecoveryDeviceErrorTag.Internal
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidCertificate {
    tag: ImportRecoveryDeviceErrorTag.InvalidCertificate
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidData {
    tag: ImportRecoveryDeviceErrorTag.InvalidData
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidPassphrase {
    tag: ImportRecoveryDeviceErrorTag.InvalidPassphrase
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidPath {
    tag: ImportRecoveryDeviceErrorTag.InvalidPath
    error: string
}
export interface ImportRecoveryDeviceErrorNoSpaceAvailable {
    tag: ImportRecoveryDeviceErrorTag.NoSpaceAvailable
    error: string
}
export interface ImportRecoveryDeviceErrorOffline {
    tag: ImportRecoveryDeviceErrorTag.Offline
    error: string
}
export interface ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadFailed {
    tag: ImportRecoveryDeviceErrorTag.RemoteOpaqueKeyUploadFailed
    error: string
}
export interface ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadOffline {
    tag: ImportRecoveryDeviceErrorTag.RemoteOpaqueKeyUploadOffline
    error: string
}
export interface ImportRecoveryDeviceErrorStopped {
    tag: ImportRecoveryDeviceErrorTag.Stopped
    error: string
}
export interface ImportRecoveryDeviceErrorTimestampOutOfBallpark {
    tag: ImportRecoveryDeviceErrorTag.TimestampOutOfBallpark
    error: string
    serverTimestamp: DateTime
    clientTimestamp: DateTime
    ballparkClientEarlyOffset: number
    ballparkClientLateOffset: number
}
export type ImportRecoveryDeviceError =
  | ImportRecoveryDeviceErrorDecryptionFailed
  | ImportRecoveryDeviceErrorInternal
  | ImportRecoveryDeviceErrorInvalidCertificate
  | ImportRecoveryDeviceErrorInvalidData
  | ImportRecoveryDeviceErrorInvalidPassphrase
  | ImportRecoveryDeviceErrorInvalidPath
  | ImportRecoveryDeviceErrorNoSpaceAvailable
  | ImportRecoveryDeviceErrorOffline
  | ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadFailed
  | ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadOffline
  | ImportRecoveryDeviceErrorStopped
  | ImportRecoveryDeviceErrorTimestampOutOfBallpark

// InviteInfoInvitationCreatedBy
export enum InviteInfoInvitationCreatedByTag {
    ExternalService = 'InviteInfoInvitationCreatedByExternalService',
    User = 'InviteInfoInvitationCreatedByUser',
}

export interface InviteInfoInvitationCreatedByExternalService {
    tag: InviteInfoInvitationCreatedByTag.ExternalService
    serviceLabel: string
}
export interface InviteInfoInvitationCreatedByUser {
    tag: InviteInfoInvitationCreatedByTag.User
    userId: UserID
    humanHandle: HumanHandle
}
export type InviteInfoInvitationCreatedBy =
  | InviteInfoInvitationCreatedByExternalService
  | InviteInfoInvitationCreatedByUser

// InviteListInvitationCreatedBy
export enum InviteListInvitationCreatedByTag {
    ExternalService = 'InviteListInvitationCreatedByExternalService',
    User = 'InviteListInvitationCreatedByUser',
}

export interface InviteListInvitationCreatedByExternalService {
    tag: InviteListInvitationCreatedByTag.ExternalService
    serviceLabel: string
}
export interface InviteListInvitationCreatedByUser {
    tag: InviteListInvitationCreatedByTag.User
    userId: UserID
    humanHandle: HumanHandle
}
export type InviteListInvitationCreatedBy =
  | InviteListInvitationCreatedByExternalService
  | InviteListInvitationCreatedByUser

// InviteListItem
export enum InviteListItemTag {
    Device = 'InviteListItemDevice',
    ShamirRecovery = 'InviteListItemShamirRecovery',
    User = 'InviteListItemUser',
}

export interface InviteListItemDevice {
    tag: InviteListItemTag.Device
    addr: ParsecInvitationAddr
    token: AccessToken
    createdOn: DateTime
    createdBy: InviteListInvitationCreatedBy
    status: InvitationStatus
}
export interface InviteListItemShamirRecovery {
    tag: InviteListItemTag.ShamirRecovery
    addr: ParsecInvitationAddr
    token: AccessToken
    createdOn: DateTime
    createdBy: InviteListInvitationCreatedBy
    claimerUserId: UserID
    shamirRecoveryCreatedOn: DateTime
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: InviteListItemTag.User
    addr: ParsecInvitationAddr
    token: AccessToken
    createdOn: DateTime
    createdBy: InviteListInvitationCreatedBy
    claimerEmail: EmailAddress
    status: InvitationStatus
}
export type InviteListItem =
  | InviteListItemDevice
  | InviteListItemShamirRecovery
  | InviteListItemUser

// ListAvailableDeviceError
export enum ListAvailableDeviceErrorTag {
    Internal = 'ListAvailableDeviceErrorInternal',
    StorageNotAvailable = 'ListAvailableDeviceErrorStorageNotAvailable',
}

export interface ListAvailableDeviceErrorInternal {
    tag: ListAvailableDeviceErrorTag.Internal
    error: string
}
export interface ListAvailableDeviceErrorStorageNotAvailable {
    tag: ListAvailableDeviceErrorTag.StorageNotAvailable
    error: string
}
export type ListAvailableDeviceError =
  | ListAvailableDeviceErrorInternal
  | ListAvailableDeviceErrorStorageNotAvailable

// ListInvitationsError
export enum ListInvitationsErrorTag {
    Internal = 'ListInvitationsErrorInternal',
    Offline = 'ListInvitationsErrorOffline',
}

export interface ListInvitationsErrorInternal {
    tag: ListInvitationsErrorTag.Internal
    error: string
}
export interface ListInvitationsErrorOffline {
    tag: ListInvitationsErrorTag.Offline
    error: string
}
export type ListInvitationsError =
  | ListInvitationsErrorInternal
  | ListInvitationsErrorOffline

// MountpointMountStrategy
export enum MountpointMountStrategyTag {
    Directory = 'MountpointMountStrategyDirectory',
    Disabled = 'MountpointMountStrategyDisabled',
    DriveLetter = 'MountpointMountStrategyDriveLetter',
}

export interface MountpointMountStrategyDirectory {
    tag: MountpointMountStrategyTag.Directory
    baseDir: Path
}
export interface MountpointMountStrategyDisabled {
    tag: MountpointMountStrategyTag.Disabled
}
export interface MountpointMountStrategyDriveLetter {
    tag: MountpointMountStrategyTag.DriveLetter
}
export type MountpointMountStrategy =
  | MountpointMountStrategyDirectory
  | MountpointMountStrategyDisabled
  | MountpointMountStrategyDriveLetter

// MountpointToOsPathError
export enum MountpointToOsPathErrorTag {
    Internal = 'MountpointToOsPathErrorInternal',
}

export interface MountpointToOsPathErrorInternal {
    tag: MountpointToOsPathErrorTag.Internal
    error: string
}
export type MountpointToOsPathError =
  | MountpointToOsPathErrorInternal

// MountpointUnmountError
export enum MountpointUnmountErrorTag {
    Internal = 'MountpointUnmountErrorInternal',
}

export interface MountpointUnmountErrorInternal {
    tag: MountpointUnmountErrorTag.Internal
    error: string
}
export type MountpointUnmountError =
  | MountpointUnmountErrorInternal

// MoveEntryMode
export enum MoveEntryModeTag {
    CanReplace = 'MoveEntryModeCanReplace',
    CanReplaceFileOnly = 'MoveEntryModeCanReplaceFileOnly',
    Exchange = 'MoveEntryModeExchange',
    NoReplace = 'MoveEntryModeNoReplace',
}

export interface MoveEntryModeCanReplace {
    tag: MoveEntryModeTag.CanReplace
}
export interface MoveEntryModeCanReplaceFileOnly {
    tag: MoveEntryModeTag.CanReplaceFileOnly
}
export interface MoveEntryModeExchange {
    tag: MoveEntryModeTag.Exchange
}
export interface MoveEntryModeNoReplace {
    tag: MoveEntryModeTag.NoReplace
}
export type MoveEntryMode =
  | MoveEntryModeCanReplace
  | MoveEntryModeCanReplaceFileOnly
  | MoveEntryModeExchange
  | MoveEntryModeNoReplace

// OpenBaoAuthConfig
export enum OpenBaoAuthConfigTag {
    OIDCHexagone = 'OpenBaoAuthConfigOIDCHexagone',
    OIDCProConnect = 'OpenBaoAuthConfigOIDCProConnect',
}

export interface OpenBaoAuthConfigOIDCHexagone {
    tag: OpenBaoAuthConfigTag.OIDCHexagone
    mountPath: string
}
export interface OpenBaoAuthConfigOIDCProConnect {
    tag: OpenBaoAuthConfigTag.OIDCProConnect
    mountPath: string
}
export type OpenBaoAuthConfig =
  | OpenBaoAuthConfigOIDCHexagone
  | OpenBaoAuthConfigOIDCProConnect

// OpenBaoListSelfEmailsError
export enum OpenBaoListSelfEmailsErrorTag {
    BadServerResponse = 'OpenBaoListSelfEmailsErrorBadServerResponse',
    BadURL = 'OpenBaoListSelfEmailsErrorBadURL',
    Internal = 'OpenBaoListSelfEmailsErrorInternal',
    NoServerResponse = 'OpenBaoListSelfEmailsErrorNoServerResponse',
}

export interface OpenBaoListSelfEmailsErrorBadServerResponse {
    tag: OpenBaoListSelfEmailsErrorTag.BadServerResponse
    error: string
}
export interface OpenBaoListSelfEmailsErrorBadURL {
    tag: OpenBaoListSelfEmailsErrorTag.BadURL
    error: string
}
export interface OpenBaoListSelfEmailsErrorInternal {
    tag: OpenBaoListSelfEmailsErrorTag.Internal
    error: string
}
export interface OpenBaoListSelfEmailsErrorNoServerResponse {
    tag: OpenBaoListSelfEmailsErrorTag.NoServerResponse
    error: string
}
export type OpenBaoListSelfEmailsError =
  | OpenBaoListSelfEmailsErrorBadServerResponse
  | OpenBaoListSelfEmailsErrorBadURL
  | OpenBaoListSelfEmailsErrorInternal
  | OpenBaoListSelfEmailsErrorNoServerResponse

// OpenBaoSecretConfig
export enum OpenBaoSecretConfigTag {
    KV2 = 'OpenBaoSecretConfigKV2',
}

export interface OpenBaoSecretConfigKV2 {
    tag: OpenBaoSecretConfigTag.KV2
    mountPath: string
}
export type OpenBaoSecretConfig =
  | OpenBaoSecretConfigKV2

// OrganizationBootstrapConfig
export enum OrganizationBootstrapConfigTag {
    Spontaneous = 'OrganizationBootstrapConfigSpontaneous',
    WithBootstrapToken = 'OrganizationBootstrapConfigWithBootstrapToken',
}

export interface OrganizationBootstrapConfigSpontaneous {
    tag: OrganizationBootstrapConfigTag.Spontaneous
}
export interface OrganizationBootstrapConfigWithBootstrapToken {
    tag: OrganizationBootstrapConfigTag.WithBootstrapToken
}
export type OrganizationBootstrapConfig =
  | OrganizationBootstrapConfigSpontaneous
  | OrganizationBootstrapConfigWithBootstrapToken

// OtherShamirRecoveryInfo
export enum OtherShamirRecoveryInfoTag {
    Deleted = 'OtherShamirRecoveryInfoDeleted',
    SetupAllValid = 'OtherShamirRecoveryInfoSetupAllValid',
    SetupButUnusable = 'OtherShamirRecoveryInfoSetupButUnusable',
    SetupWithRevokedRecipients = 'OtherShamirRecoveryInfoSetupWithRevokedRecipients',
}

export interface OtherShamirRecoveryInfoDeleted {
    tag: OtherShamirRecoveryInfoTag.Deleted
    userId: UserID
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    deletedOn: DateTime
    deletedBy: DeviceID
}
export interface OtherShamirRecoveryInfoSetupAllValid {
    tag: OtherShamirRecoveryInfoTag.SetupAllValid
    userId: UserID
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
}
export interface OtherShamirRecoveryInfoSetupButUnusable {
    tag: OtherShamirRecoveryInfoTag.SetupButUnusable
    userId: UserID
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    revokedRecipients: Array<UserID>
}
export interface OtherShamirRecoveryInfoSetupWithRevokedRecipients {
    tag: OtherShamirRecoveryInfoTag.SetupWithRevokedRecipients
    userId: UserID
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    revokedRecipients: Array<UserID>
}
export type OtherShamirRecoveryInfo =
  | OtherShamirRecoveryInfoDeleted
  | OtherShamirRecoveryInfoSetupAllValid
  | OtherShamirRecoveryInfoSetupButUnusable
  | OtherShamirRecoveryInfoSetupWithRevokedRecipients

// ParseParsecAddrError
export enum ParseParsecAddrErrorTag {
    InvalidUrl = 'ParseParsecAddrErrorInvalidUrl',
}

export interface ParseParsecAddrErrorInvalidUrl {
    tag: ParseParsecAddrErrorTag.InvalidUrl
    error: string
}
export type ParseParsecAddrError =
  | ParseParsecAddrErrorInvalidUrl

// ParsedParsecAddr
export enum ParsedParsecAddrTag {
    AsyncEnrollment = 'ParsedParsecAddrAsyncEnrollment',
    InvitationDevice = 'ParsedParsecAddrInvitationDevice',
    InvitationShamirRecovery = 'ParsedParsecAddrInvitationShamirRecovery',
    InvitationUser = 'ParsedParsecAddrInvitationUser',
    Organization = 'ParsedParsecAddrOrganization',
    OrganizationBootstrap = 'ParsedParsecAddrOrganizationBootstrap',
    PkiEnrollment = 'ParsedParsecAddrPkiEnrollment',
    Server = 'ParsedParsecAddrServer',
    TOTPReset = 'ParsedParsecAddrTOTPReset',
    WorkspacePath = 'ParsedParsecAddrWorkspacePath',
}

export interface ParsedParsecAddrAsyncEnrollment {
    tag: ParsedParsecAddrTag.AsyncEnrollment
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
}
export interface ParsedParsecAddrInvitationDevice {
    tag: ParsedParsecAddrTag.InvitationDevice
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
    token: AccessToken
}
export interface ParsedParsecAddrInvitationShamirRecovery {
    tag: ParsedParsecAddrTag.InvitationShamirRecovery
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
    token: AccessToken
}
export interface ParsedParsecAddrInvitationUser {
    tag: ParsedParsecAddrTag.InvitationUser
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
    token: AccessToken
}
export interface ParsedParsecAddrOrganization {
    tag: ParsedParsecAddrTag.Organization
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
}
export interface ParsedParsecAddrOrganizationBootstrap {
    tag: ParsedParsecAddrTag.OrganizationBootstrap
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
    token: string | null
}
export interface ParsedParsecAddrPkiEnrollment {
    tag: ParsedParsecAddrTag.PkiEnrollment
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
}
export interface ParsedParsecAddrServer {
    tag: ParsedParsecAddrTag.Server
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
}
export interface ParsedParsecAddrTOTPReset {
    tag: ParsedParsecAddrTag.TOTPReset
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
    userId: UserID
    token: AccessToken
}
export interface ParsedParsecAddrWorkspacePath {
    tag: ParsedParsecAddrTag.WorkspacePath
    hostname: string
    port: U16
    isDefaultPort: boolean
    useSsl: boolean
    organizationId: OrganizationID
    workspaceId: VlobID
    keyIndex: IndexInt
    encryptedPath: Uint8Array
}
export type ParsedParsecAddr =
  | ParsedParsecAddrAsyncEnrollment
  | ParsedParsecAddrInvitationDevice
  | ParsedParsecAddrInvitationShamirRecovery
  | ParsedParsecAddrInvitationUser
  | ParsedParsecAddrOrganization
  | ParsedParsecAddrOrganizationBootstrap
  | ParsedParsecAddrPkiEnrollment
  | ParsedParsecAddrServer
  | ParsedParsecAddrTOTPReset
  | ParsedParsecAddrWorkspacePath

// PendingAsyncEnrollmentInfo
export enum PendingAsyncEnrollmentInfoTag {
    Accepted = 'PendingAsyncEnrollmentInfoAccepted',
    Cancelled = 'PendingAsyncEnrollmentInfoCancelled',
    Rejected = 'PendingAsyncEnrollmentInfoRejected',
    Submitted = 'PendingAsyncEnrollmentInfoSubmitted',
}

export interface PendingAsyncEnrollmentInfoAccepted {
    tag: PendingAsyncEnrollmentInfoTag.Accepted
    submittedOn: DateTime
    acceptedOn: DateTime
}
export interface PendingAsyncEnrollmentInfoCancelled {
    tag: PendingAsyncEnrollmentInfoTag.Cancelled
    submittedOn: DateTime
    cancelledOn: DateTime
}
export interface PendingAsyncEnrollmentInfoRejected {
    tag: PendingAsyncEnrollmentInfoTag.Rejected
    submittedOn: DateTime
    rejectedOn: DateTime
}
export interface PendingAsyncEnrollmentInfoSubmitted {
    tag: PendingAsyncEnrollmentInfoTag.Submitted
    submittedOn: DateTime
}
export type PendingAsyncEnrollmentInfo =
  | PendingAsyncEnrollmentInfoAccepted
  | PendingAsyncEnrollmentInfoCancelled
  | PendingAsyncEnrollmentInfoRejected
  | PendingAsyncEnrollmentInfoSubmitted

// RemoveDeviceDataError
export enum RemoveDeviceDataErrorTag {
    FailedToRemoveData = 'RemoveDeviceDataErrorFailedToRemoveData',
}

export interface RemoveDeviceDataErrorFailedToRemoveData {
    tag: RemoveDeviceDataErrorTag.FailedToRemoveData
    error: string
}
export type RemoveDeviceDataError =
  | RemoveDeviceDataErrorFailedToRemoveData

// SelfShamirRecoveryInfo
export enum SelfShamirRecoveryInfoTag {
    Deleted = 'SelfShamirRecoveryInfoDeleted',
    NeverSetup = 'SelfShamirRecoveryInfoNeverSetup',
    SetupAllValid = 'SelfShamirRecoveryInfoSetupAllValid',
    SetupButUnusable = 'SelfShamirRecoveryInfoSetupButUnusable',
    SetupWithRevokedRecipients = 'SelfShamirRecoveryInfoSetupWithRevokedRecipients',
}

export interface SelfShamirRecoveryInfoDeleted {
    tag: SelfShamirRecoveryInfoTag.Deleted
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    deletedOn: DateTime
    deletedBy: DeviceID
}
export interface SelfShamirRecoveryInfoNeverSetup {
    tag: SelfShamirRecoveryInfoTag.NeverSetup
}
export interface SelfShamirRecoveryInfoSetupAllValid {
    tag: SelfShamirRecoveryInfoTag.SetupAllValid
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
}
export interface SelfShamirRecoveryInfoSetupButUnusable {
    tag: SelfShamirRecoveryInfoTag.SetupButUnusable
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    revokedRecipients: Array<UserID>
}
export interface SelfShamirRecoveryInfoSetupWithRevokedRecipients {
    tag: SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients
    createdOn: DateTime
    createdBy: DeviceID
    threshold: NonZeroU8
    perRecipientShares: Map<UserID, NonZeroU8>
    revokedRecipients: Array<UserID>
}
export type SelfShamirRecoveryInfo =
  | SelfShamirRecoveryInfoDeleted
  | SelfShamirRecoveryInfoNeverSetup
  | SelfShamirRecoveryInfoSetupAllValid
  | SelfShamirRecoveryInfoSetupButUnusable
  | SelfShamirRecoveryInfoSetupWithRevokedRecipients

// ShamirRecoveryClaimAddShareError
export enum ShamirRecoveryClaimAddShareErrorTag {
    CorruptedSecret = 'ShamirRecoveryClaimAddShareErrorCorruptedSecret',
    Internal = 'ShamirRecoveryClaimAddShareErrorInternal',
    RecipientNotFound = 'ShamirRecoveryClaimAddShareErrorRecipientNotFound',
}

export interface ShamirRecoveryClaimAddShareErrorCorruptedSecret {
    tag: ShamirRecoveryClaimAddShareErrorTag.CorruptedSecret
    error: string
}
export interface ShamirRecoveryClaimAddShareErrorInternal {
    tag: ShamirRecoveryClaimAddShareErrorTag.Internal
    error: string
}
export interface ShamirRecoveryClaimAddShareErrorRecipientNotFound {
    tag: ShamirRecoveryClaimAddShareErrorTag.RecipientNotFound
    error: string
}
export type ShamirRecoveryClaimAddShareError =
  | ShamirRecoveryClaimAddShareErrorCorruptedSecret
  | ShamirRecoveryClaimAddShareErrorInternal
  | ShamirRecoveryClaimAddShareErrorRecipientNotFound

// ShamirRecoveryClaimMaybeFinalizeInfo
export enum ShamirRecoveryClaimMaybeFinalizeInfoTag {
    Finalize = 'ShamirRecoveryClaimMaybeFinalizeInfoFinalize',
    Offline = 'ShamirRecoveryClaimMaybeFinalizeInfoOffline',
}

export interface ShamirRecoveryClaimMaybeFinalizeInfoFinalize {
    tag: ShamirRecoveryClaimMaybeFinalizeInfoTag.Finalize
    handle: Handle
}
export interface ShamirRecoveryClaimMaybeFinalizeInfoOffline {
    tag: ShamirRecoveryClaimMaybeFinalizeInfoTag.Offline
    handle: Handle
}
export type ShamirRecoveryClaimMaybeFinalizeInfo =
  | ShamirRecoveryClaimMaybeFinalizeInfoFinalize
  | ShamirRecoveryClaimMaybeFinalizeInfoOffline

// ShamirRecoveryClaimMaybeRecoverDeviceInfo
export enum ShamirRecoveryClaimMaybeRecoverDeviceInfoTag {
    PickRecipient = 'ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient',
    RecoverDevice = 'ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice',
}

export interface ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient {
    tag: ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.PickRecipient
    handle: Handle
    claimerUserId: UserID
    claimerHumanHandle: HumanHandle
    shamirRecoveryCreatedOn: DateTime
    recipients: Array<ShamirRecoveryRecipient>
    threshold: NonZeroU8
    recoveredShares: Map<UserID, NonZeroU8>
    isRecoverable: boolean
}
export interface ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice {
    tag: ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.RecoverDevice
    handle: Handle
    claimerUserId: UserID
    claimerHumanHandle: HumanHandle
}
export type ShamirRecoveryClaimMaybeRecoverDeviceInfo =
  | ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient
  | ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice

// ShamirRecoveryClaimPickRecipientError
export enum ShamirRecoveryClaimPickRecipientErrorTag {
    Internal = 'ShamirRecoveryClaimPickRecipientErrorInternal',
    RecipientAlreadyPicked = 'ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked',
    RecipientNotFound = 'ShamirRecoveryClaimPickRecipientErrorRecipientNotFound',
    RecipientRevoked = 'ShamirRecoveryClaimPickRecipientErrorRecipientRevoked',
}

export interface ShamirRecoveryClaimPickRecipientErrorInternal {
    tag: ShamirRecoveryClaimPickRecipientErrorTag.Internal
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked {
    tag: ShamirRecoveryClaimPickRecipientErrorTag.RecipientAlreadyPicked
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientNotFound {
    tag: ShamirRecoveryClaimPickRecipientErrorTag.RecipientNotFound
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientRevoked {
    tag: ShamirRecoveryClaimPickRecipientErrorTag.RecipientRevoked
    error: string
}
export type ShamirRecoveryClaimPickRecipientError =
  | ShamirRecoveryClaimPickRecipientErrorInternal
  | ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked
  | ShamirRecoveryClaimPickRecipientErrorRecipientNotFound
  | ShamirRecoveryClaimPickRecipientErrorRecipientRevoked

// ShamirRecoveryClaimRecoverDeviceError
export enum ShamirRecoveryClaimRecoverDeviceErrorTag {
    AlreadyUsed = 'ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed',
    CipheredDataNotFound = 'ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound',
    CorruptedCipheredData = 'ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData',
    Internal = 'ShamirRecoveryClaimRecoverDeviceErrorInternal',
    NotFound = 'ShamirRecoveryClaimRecoverDeviceErrorNotFound',
    OrganizationExpired = 'ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired',
    RegisterNewDeviceError = 'ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError',
}

export interface ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.AlreadyUsed
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.CipheredDataNotFound
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.CorruptedCipheredData
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorInternal {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.Internal
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorNotFound {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.NotFound
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.OrganizationExpired
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError {
    tag: ShamirRecoveryClaimRecoverDeviceErrorTag.RegisterNewDeviceError
    error: string
}
export type ShamirRecoveryClaimRecoverDeviceError =
  | ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed
  | ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound
  | ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData
  | ShamirRecoveryClaimRecoverDeviceErrorInternal
  | ShamirRecoveryClaimRecoverDeviceErrorNotFound
  | ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired
  | ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError

// ShowCertificateSelectionDialogError
export enum ShowCertificateSelectionDialogErrorTag {
    CannotGetCertificateInfo = 'ShowCertificateSelectionDialogErrorCannotGetCertificateInfo',
    CannotOpenStore = 'ShowCertificateSelectionDialogErrorCannotOpenStore',
}

export interface ShowCertificateSelectionDialogErrorCannotGetCertificateInfo {
    tag: ShowCertificateSelectionDialogErrorTag.CannotGetCertificateInfo
    error: string
}
export interface ShowCertificateSelectionDialogErrorCannotOpenStore {
    tag: ShowCertificateSelectionDialogErrorTag.CannotOpenStore
    error: string
}
export type ShowCertificateSelectionDialogError =
  | ShowCertificateSelectionDialogErrorCannotGetCertificateInfo
  | ShowCertificateSelectionDialogErrorCannotOpenStore

// SubmitAsyncEnrollmentError
export enum SubmitAsyncEnrollmentErrorTag {
    EmailAlreadyEnrolled = 'SubmitAsyncEnrollmentErrorEmailAlreadyEnrolled',
    EmailAlreadySubmitted = 'SubmitAsyncEnrollmentErrorEmailAlreadySubmitted',
    Internal = 'SubmitAsyncEnrollmentErrorInternal',
    InvalidPath = 'SubmitAsyncEnrollmentErrorInvalidPath',
    NoSpaceAvailable = 'SubmitAsyncEnrollmentErrorNoSpaceAvailable',
    Offline = 'SubmitAsyncEnrollmentErrorOffline',
    OpenBaoBadServerResponse = 'SubmitAsyncEnrollmentErrorOpenBaoBadServerResponse',
    OpenBaoBadURL = 'SubmitAsyncEnrollmentErrorOpenBaoBadURL',
    OpenBaoNoServerResponse = 'SubmitAsyncEnrollmentErrorOpenBaoNoServerResponse',
    PKICannotOpenCertificateStore = 'SubmitAsyncEnrollmentErrorPKICannotOpenCertificateStore',
    PKIServerInvalidX509Trustchain = 'SubmitAsyncEnrollmentErrorPKIServerInvalidX509Trustchain',
    PKIUnusableX509CertificateReference = 'SubmitAsyncEnrollmentErrorPKIUnusableX509CertificateReference',
}

export interface SubmitAsyncEnrollmentErrorEmailAlreadyEnrolled {
    tag: SubmitAsyncEnrollmentErrorTag.EmailAlreadyEnrolled
    error: string
}
export interface SubmitAsyncEnrollmentErrorEmailAlreadySubmitted {
    tag: SubmitAsyncEnrollmentErrorTag.EmailAlreadySubmitted
    error: string
}
export interface SubmitAsyncEnrollmentErrorInternal {
    tag: SubmitAsyncEnrollmentErrorTag.Internal
    error: string
}
export interface SubmitAsyncEnrollmentErrorInvalidPath {
    tag: SubmitAsyncEnrollmentErrorTag.InvalidPath
    error: string
}
export interface SubmitAsyncEnrollmentErrorNoSpaceAvailable {
    tag: SubmitAsyncEnrollmentErrorTag.NoSpaceAvailable
    error: string
}
export interface SubmitAsyncEnrollmentErrorOffline {
    tag: SubmitAsyncEnrollmentErrorTag.Offline
    error: string
}
export interface SubmitAsyncEnrollmentErrorOpenBaoBadServerResponse {
    tag: SubmitAsyncEnrollmentErrorTag.OpenBaoBadServerResponse
    error: string
}
export interface SubmitAsyncEnrollmentErrorOpenBaoBadURL {
    tag: SubmitAsyncEnrollmentErrorTag.OpenBaoBadURL
    error: string
}
export interface SubmitAsyncEnrollmentErrorOpenBaoNoServerResponse {
    tag: SubmitAsyncEnrollmentErrorTag.OpenBaoNoServerResponse
    error: string
}
export interface SubmitAsyncEnrollmentErrorPKICannotOpenCertificateStore {
    tag: SubmitAsyncEnrollmentErrorTag.PKICannotOpenCertificateStore
    error: string
}
export interface SubmitAsyncEnrollmentErrorPKIServerInvalidX509Trustchain {
    tag: SubmitAsyncEnrollmentErrorTag.PKIServerInvalidX509Trustchain
    error: string
}
export interface SubmitAsyncEnrollmentErrorPKIUnusableX509CertificateReference {
    tag: SubmitAsyncEnrollmentErrorTag.PKIUnusableX509CertificateReference
    error: string
}
export type SubmitAsyncEnrollmentError =
  | SubmitAsyncEnrollmentErrorEmailAlreadyEnrolled
  | SubmitAsyncEnrollmentErrorEmailAlreadySubmitted
  | SubmitAsyncEnrollmentErrorInternal
  | SubmitAsyncEnrollmentErrorInvalidPath
  | SubmitAsyncEnrollmentErrorNoSpaceAvailable
  | SubmitAsyncEnrollmentErrorOffline
  | SubmitAsyncEnrollmentErrorOpenBaoBadServerResponse
  | SubmitAsyncEnrollmentErrorOpenBaoBadURL
  | SubmitAsyncEnrollmentErrorOpenBaoNoServerResponse
  | SubmitAsyncEnrollmentErrorPKICannotOpenCertificateStore
  | SubmitAsyncEnrollmentErrorPKIServerInvalidX509Trustchain
  | SubmitAsyncEnrollmentErrorPKIUnusableX509CertificateReference

// SubmitAsyncEnrollmentIdentityStrategy
export enum SubmitAsyncEnrollmentIdentityStrategyTag {
    OpenBao = 'SubmitAsyncEnrollmentIdentityStrategyOpenBao',
    PKI = 'SubmitAsyncEnrollmentIdentityStrategyPKI',
}

export interface SubmitAsyncEnrollmentIdentityStrategyOpenBao {
    tag: SubmitAsyncEnrollmentIdentityStrategyTag.OpenBao
    requestedHumanHandle: HumanHandle
    openbaoServerUrl: string
    openbaoTransitMountPath: string
    openbaoSecretMountPath: string
    openbaoEntityId: string
    openbaoAuthToken: string
    openbaoPreferredAuthId: string
}
export interface SubmitAsyncEnrollmentIdentityStrategyPKI {
    tag: SubmitAsyncEnrollmentIdentityStrategyTag.PKI
    certificateReference: X509CertificateReference
}
export type SubmitAsyncEnrollmentIdentityStrategy =
  | SubmitAsyncEnrollmentIdentityStrategyOpenBao
  | SubmitAsyncEnrollmentIdentityStrategyPKI

// SubmitterCancelAsyncEnrollmentError
export enum SubmitterCancelAsyncEnrollmentErrorTag {
    Internal = 'SubmitterCancelAsyncEnrollmentErrorInternal',
    NotFound = 'SubmitterCancelAsyncEnrollmentErrorNotFound',
    Offline = 'SubmitterCancelAsyncEnrollmentErrorOffline',
    StorageNotAvailable = 'SubmitterCancelAsyncEnrollmentErrorStorageNotAvailable',
}

export interface SubmitterCancelAsyncEnrollmentErrorInternal {
    tag: SubmitterCancelAsyncEnrollmentErrorTag.Internal
    error: string
}
export interface SubmitterCancelAsyncEnrollmentErrorNotFound {
    tag: SubmitterCancelAsyncEnrollmentErrorTag.NotFound
    error: string
}
export interface SubmitterCancelAsyncEnrollmentErrorOffline {
    tag: SubmitterCancelAsyncEnrollmentErrorTag.Offline
    error: string
}
export interface SubmitterCancelAsyncEnrollmentErrorStorageNotAvailable {
    tag: SubmitterCancelAsyncEnrollmentErrorTag.StorageNotAvailable
    error: string
}
export type SubmitterCancelAsyncEnrollmentError =
  | SubmitterCancelAsyncEnrollmentErrorInternal
  | SubmitterCancelAsyncEnrollmentErrorNotFound
  | SubmitterCancelAsyncEnrollmentErrorOffline
  | SubmitterCancelAsyncEnrollmentErrorStorageNotAvailable

// SubmitterFinalizeAsyncEnrollmentError
export enum SubmitterFinalizeAsyncEnrollmentErrorTag {
    BadAcceptPayload = 'SubmitterFinalizeAsyncEnrollmentErrorBadAcceptPayload',
    EnrollmentFileCannotRetrieveCiphertextKey = 'SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileCannotRetrieveCiphertextKey',
    EnrollmentFileInvalidData = 'SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidData',
    EnrollmentFileInvalidPath = 'SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidPath',
    EnrollmentNotFoundOnServer = 'SubmitterFinalizeAsyncEnrollmentErrorEnrollmentNotFoundOnServer',
    IdentityStrategyMismatch = 'SubmitterFinalizeAsyncEnrollmentErrorIdentityStrategyMismatch',
    Internal = 'SubmitterFinalizeAsyncEnrollmentErrorInternal',
    NoSpaceAvailable = 'SubmitterFinalizeAsyncEnrollmentErrorNoSpaceAvailable',
    NotAccepted = 'SubmitterFinalizeAsyncEnrollmentErrorNotAccepted',
    Offline = 'SubmitterFinalizeAsyncEnrollmentErrorOffline',
    OpenBaoBadServerResponse = 'SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadServerResponse',
    OpenBaoBadURL = 'SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadURL',
    OpenBaoNoServerResponse = 'SubmitterFinalizeAsyncEnrollmentErrorOpenBaoNoServerResponse',
    PKICannotOpenCertificateStore = 'SubmitterFinalizeAsyncEnrollmentErrorPKICannotOpenCertificateStore',
    PKIUnusableX509CertificateReference = 'SubmitterFinalizeAsyncEnrollmentErrorPKIUnusableX509CertificateReference',
    SaveDeviceInvalidPath = 'SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceInvalidPath',
    SaveDeviceRemoteOpaqueKeyUploadFailed = 'SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadFailed',
    SaveDeviceRemoteOpaqueKeyUploadOffline = 'SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadOffline',
}

export interface SubmitterFinalizeAsyncEnrollmentErrorBadAcceptPayload {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.BadAcceptPayload
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileCannotRetrieveCiphertextKey {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.EnrollmentFileCannotRetrieveCiphertextKey
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidData {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.EnrollmentFileInvalidData
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidPath {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.EnrollmentFileInvalidPath
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorEnrollmentNotFoundOnServer {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.EnrollmentNotFoundOnServer
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorIdentityStrategyMismatch {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.IdentityStrategyMismatch
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorInternal {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.Internal
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorNoSpaceAvailable {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.NoSpaceAvailable
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorNotAccepted {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.NotAccepted
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorOffline {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.Offline
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadServerResponse {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.OpenBaoBadServerResponse
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadURL {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.OpenBaoBadURL
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorOpenBaoNoServerResponse {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.OpenBaoNoServerResponse
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorPKICannotOpenCertificateStore {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.PKICannotOpenCertificateStore
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorPKIUnusableX509CertificateReference {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.PKIUnusableX509CertificateReference
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceInvalidPath {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.SaveDeviceInvalidPath
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadFailed {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.SaveDeviceRemoteOpaqueKeyUploadFailed
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadOffline {
    tag: SubmitterFinalizeAsyncEnrollmentErrorTag.SaveDeviceRemoteOpaqueKeyUploadOffline
    error: string
}
export type SubmitterFinalizeAsyncEnrollmentError =
  | SubmitterFinalizeAsyncEnrollmentErrorBadAcceptPayload
  | SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileCannotRetrieveCiphertextKey
  | SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidData
  | SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidPath
  | SubmitterFinalizeAsyncEnrollmentErrorEnrollmentNotFoundOnServer
  | SubmitterFinalizeAsyncEnrollmentErrorIdentityStrategyMismatch
  | SubmitterFinalizeAsyncEnrollmentErrorInternal
  | SubmitterFinalizeAsyncEnrollmentErrorNoSpaceAvailable
  | SubmitterFinalizeAsyncEnrollmentErrorNotAccepted
  | SubmitterFinalizeAsyncEnrollmentErrorOffline
  | SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadServerResponse
  | SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadURL
  | SubmitterFinalizeAsyncEnrollmentErrorOpenBaoNoServerResponse
  | SubmitterFinalizeAsyncEnrollmentErrorPKICannotOpenCertificateStore
  | SubmitterFinalizeAsyncEnrollmentErrorPKIUnusableX509CertificateReference
  | SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceInvalidPath
  | SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadFailed
  | SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadOffline

// SubmitterGetAsyncEnrollmentInfoError
export enum SubmitterGetAsyncEnrollmentInfoErrorTag {
    EnrollmentNotFound = 'SubmitterGetAsyncEnrollmentInfoErrorEnrollmentNotFound',
    Internal = 'SubmitterGetAsyncEnrollmentInfoErrorInternal',
    Offline = 'SubmitterGetAsyncEnrollmentInfoErrorOffline',
}

export interface SubmitterGetAsyncEnrollmentInfoErrorEnrollmentNotFound {
    tag: SubmitterGetAsyncEnrollmentInfoErrorTag.EnrollmentNotFound
    error: string
}
export interface SubmitterGetAsyncEnrollmentInfoErrorInternal {
    tag: SubmitterGetAsyncEnrollmentInfoErrorTag.Internal
    error: string
}
export interface SubmitterGetAsyncEnrollmentInfoErrorOffline {
    tag: SubmitterGetAsyncEnrollmentInfoErrorTag.Offline
    error: string
}
export type SubmitterGetAsyncEnrollmentInfoError =
  | SubmitterGetAsyncEnrollmentInfoErrorEnrollmentNotFound
  | SubmitterGetAsyncEnrollmentInfoErrorInternal
  | SubmitterGetAsyncEnrollmentInfoErrorOffline

// SubmitterListLocalAsyncEnrollmentsError
export enum SubmitterListLocalAsyncEnrollmentsErrorTag {
    Internal = 'SubmitterListLocalAsyncEnrollmentsErrorInternal',
    StorageNotAvailable = 'SubmitterListLocalAsyncEnrollmentsErrorStorageNotAvailable',
}

export interface SubmitterListLocalAsyncEnrollmentsErrorInternal {
    tag: SubmitterListLocalAsyncEnrollmentsErrorTag.Internal
    error: string
}
export interface SubmitterListLocalAsyncEnrollmentsErrorStorageNotAvailable {
    tag: SubmitterListLocalAsyncEnrollmentsErrorTag.StorageNotAvailable
    error: string
}
export type SubmitterListLocalAsyncEnrollmentsError =
  | SubmitterListLocalAsyncEnrollmentsErrorInternal
  | SubmitterListLocalAsyncEnrollmentsErrorStorageNotAvailable

// TOTPSetupStatus
export enum TOTPSetupStatusTag {
    AlreadySetup = 'TOTPSetupStatusAlreadySetup',
    Stalled = 'TOTPSetupStatusStalled',
}

export interface TOTPSetupStatusAlreadySetup {
    tag: TOTPSetupStatusTag.AlreadySetup
}
export interface TOTPSetupStatusStalled {
    tag: TOTPSetupStatusTag.Stalled
    base32TotpSecret: string
}
export type TOTPSetupStatus =
  | TOTPSetupStatusAlreadySetup
  | TOTPSetupStatusStalled

// TestbedError
export enum TestbedErrorTag {
    Disabled = 'TestbedErrorDisabled',
    Internal = 'TestbedErrorInternal',
}

export interface TestbedErrorDisabled {
    tag: TestbedErrorTag.Disabled
    error: string
}
export interface TestbedErrorInternal {
    tag: TestbedErrorTag.Internal
    error: string
}
export type TestbedError =
  | TestbedErrorDisabled
  | TestbedErrorInternal

// TotpFetchOpaqueKeyError
export enum TotpFetchOpaqueKeyErrorTag {
    Internal = 'TotpFetchOpaqueKeyErrorInternal',
    InvalidOneTimePassword = 'TotpFetchOpaqueKeyErrorInvalidOneTimePassword',
    Offline = 'TotpFetchOpaqueKeyErrorOffline',
    Throttled = 'TotpFetchOpaqueKeyErrorThrottled',
}

export interface TotpFetchOpaqueKeyErrorInternal {
    tag: TotpFetchOpaqueKeyErrorTag.Internal
    error: string
}
export interface TotpFetchOpaqueKeyErrorInvalidOneTimePassword {
    tag: TotpFetchOpaqueKeyErrorTag.InvalidOneTimePassword
    error: string
}
export interface TotpFetchOpaqueKeyErrorOffline {
    tag: TotpFetchOpaqueKeyErrorTag.Offline
    error: string
}
export interface TotpFetchOpaqueKeyErrorThrottled {
    tag: TotpFetchOpaqueKeyErrorTag.Throttled
    error: string
}
export type TotpFetchOpaqueKeyError =
  | TotpFetchOpaqueKeyErrorInternal
  | TotpFetchOpaqueKeyErrorInvalidOneTimePassword
  | TotpFetchOpaqueKeyErrorOffline
  | TotpFetchOpaqueKeyErrorThrottled

// TotpSetupConfirmAnonymousError
export enum TotpSetupConfirmAnonymousErrorTag {
    BadToken = 'TotpSetupConfirmAnonymousErrorBadToken',
    Internal = 'TotpSetupConfirmAnonymousErrorInternal',
    InvalidOneTimePassword = 'TotpSetupConfirmAnonymousErrorInvalidOneTimePassword',
    Offline = 'TotpSetupConfirmAnonymousErrorOffline',
}

export interface TotpSetupConfirmAnonymousErrorBadToken {
    tag: TotpSetupConfirmAnonymousErrorTag.BadToken
    error: string
}
export interface TotpSetupConfirmAnonymousErrorInternal {
    tag: TotpSetupConfirmAnonymousErrorTag.Internal
    error: string
}
export interface TotpSetupConfirmAnonymousErrorInvalidOneTimePassword {
    tag: TotpSetupConfirmAnonymousErrorTag.InvalidOneTimePassword
    error: string
}
export interface TotpSetupConfirmAnonymousErrorOffline {
    tag: TotpSetupConfirmAnonymousErrorTag.Offline
    error: string
}
export type TotpSetupConfirmAnonymousError =
  | TotpSetupConfirmAnonymousErrorBadToken
  | TotpSetupConfirmAnonymousErrorInternal
  | TotpSetupConfirmAnonymousErrorInvalidOneTimePassword
  | TotpSetupConfirmAnonymousErrorOffline

// TotpSetupStatusAnonymousError
export enum TotpSetupStatusAnonymousErrorTag {
    BadToken = 'TotpSetupStatusAnonymousErrorBadToken',
    Internal = 'TotpSetupStatusAnonymousErrorInternal',
    Offline = 'TotpSetupStatusAnonymousErrorOffline',
}

export interface TotpSetupStatusAnonymousErrorBadToken {
    tag: TotpSetupStatusAnonymousErrorTag.BadToken
    error: string
}
export interface TotpSetupStatusAnonymousErrorInternal {
    tag: TotpSetupStatusAnonymousErrorTag.Internal
    error: string
}
export interface TotpSetupStatusAnonymousErrorOffline {
    tag: TotpSetupStatusAnonymousErrorTag.Offline
    error: string
}
export type TotpSetupStatusAnonymousError =
  | TotpSetupStatusAnonymousErrorBadToken
  | TotpSetupStatusAnonymousErrorInternal
  | TotpSetupStatusAnonymousErrorOffline

// UpdateDeviceError
export enum UpdateDeviceErrorTag {
    DecryptionFailed = 'UpdateDeviceErrorDecryptionFailed',
    Internal = 'UpdateDeviceErrorInternal',
    InvalidData = 'UpdateDeviceErrorInvalidData',
    InvalidPath = 'UpdateDeviceErrorInvalidPath',
    NoSpaceAvailable = 'UpdateDeviceErrorNoSpaceAvailable',
    RemoteOpaqueKeyOperationFailed = 'UpdateDeviceErrorRemoteOpaqueKeyOperationFailed',
    RemoteOpaqueKeyOperationOffline = 'UpdateDeviceErrorRemoteOpaqueKeyOperationOffline',
    TOTPDecryptionFailed = 'UpdateDeviceErrorTOTPDecryptionFailed',
}

export interface UpdateDeviceErrorDecryptionFailed {
    tag: UpdateDeviceErrorTag.DecryptionFailed
    error: string
}
export interface UpdateDeviceErrorInternal {
    tag: UpdateDeviceErrorTag.Internal
    error: string
}
export interface UpdateDeviceErrorInvalidData {
    tag: UpdateDeviceErrorTag.InvalidData
    error: string
}
export interface UpdateDeviceErrorInvalidPath {
    tag: UpdateDeviceErrorTag.InvalidPath
    error: string
}
export interface UpdateDeviceErrorNoSpaceAvailable {
    tag: UpdateDeviceErrorTag.NoSpaceAvailable
    error: string
}
export interface UpdateDeviceErrorRemoteOpaqueKeyOperationFailed {
    tag: UpdateDeviceErrorTag.RemoteOpaqueKeyOperationFailed
    error: string
}
export interface UpdateDeviceErrorRemoteOpaqueKeyOperationOffline {
    tag: UpdateDeviceErrorTag.RemoteOpaqueKeyOperationOffline
    error: string
}
export interface UpdateDeviceErrorTOTPDecryptionFailed {
    tag: UpdateDeviceErrorTag.TOTPDecryptionFailed
    error: string
}
export type UpdateDeviceError =
  | UpdateDeviceErrorDecryptionFailed
  | UpdateDeviceErrorInternal
  | UpdateDeviceErrorInvalidData
  | UpdateDeviceErrorInvalidPath
  | UpdateDeviceErrorNoSpaceAvailable
  | UpdateDeviceErrorRemoteOpaqueKeyOperationFailed
  | UpdateDeviceErrorRemoteOpaqueKeyOperationOffline
  | UpdateDeviceErrorTOTPDecryptionFailed

// UserClaimListInitialInfosError
export enum UserClaimListInitialInfosErrorTag {
    Internal = 'UserClaimListInitialInfosErrorInternal',
}

export interface UserClaimListInitialInfosErrorInternal {
    tag: UserClaimListInitialInfosErrorTag.Internal
    error: string
}
export type UserClaimListInitialInfosError =
  | UserClaimListInitialInfosErrorInternal

// WaitForDeviceAvailableError
export enum WaitForDeviceAvailableErrorTag {
    Internal = 'WaitForDeviceAvailableErrorInternal',
}

export interface WaitForDeviceAvailableErrorInternal {
    tag: WaitForDeviceAvailableErrorTag.Internal
    error: string
}
export type WaitForDeviceAvailableError =
  | WaitForDeviceAvailableErrorInternal

// WorkspaceCreateFileError
export enum WorkspaceCreateFileErrorTag {
    EntryExists = 'WorkspaceCreateFileErrorEntryExists',
    Internal = 'WorkspaceCreateFileErrorInternal',
    InvalidCertificate = 'WorkspaceCreateFileErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceCreateFileErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceCreateFileErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceCreateFileErrorNoRealmAccess',
    Offline = 'WorkspaceCreateFileErrorOffline',
    ParentNotAFolder = 'WorkspaceCreateFileErrorParentNotAFolder',
    ParentNotFound = 'WorkspaceCreateFileErrorParentNotFound',
    ReadOnlyRealm = 'WorkspaceCreateFileErrorReadOnlyRealm',
    Stopped = 'WorkspaceCreateFileErrorStopped',
}

export interface WorkspaceCreateFileErrorEntryExists {
    tag: WorkspaceCreateFileErrorTag.EntryExists
    error: string
}
export interface WorkspaceCreateFileErrorInternal {
    tag: WorkspaceCreateFileErrorTag.Internal
    error: string
}
export interface WorkspaceCreateFileErrorInvalidCertificate {
    tag: WorkspaceCreateFileErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceCreateFileErrorInvalidKeysBundle {
    tag: WorkspaceCreateFileErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceCreateFileErrorInvalidManifest {
    tag: WorkspaceCreateFileErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceCreateFileErrorNoRealmAccess {
    tag: WorkspaceCreateFileErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceCreateFileErrorOffline {
    tag: WorkspaceCreateFileErrorTag.Offline
    error: string
}
export interface WorkspaceCreateFileErrorParentNotAFolder {
    tag: WorkspaceCreateFileErrorTag.ParentNotAFolder
    error: string
}
export interface WorkspaceCreateFileErrorParentNotFound {
    tag: WorkspaceCreateFileErrorTag.ParentNotFound
    error: string
}
export interface WorkspaceCreateFileErrorReadOnlyRealm {
    tag: WorkspaceCreateFileErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceCreateFileErrorStopped {
    tag: WorkspaceCreateFileErrorTag.Stopped
    error: string
}
export type WorkspaceCreateFileError =
  | WorkspaceCreateFileErrorEntryExists
  | WorkspaceCreateFileErrorInternal
  | WorkspaceCreateFileErrorInvalidCertificate
  | WorkspaceCreateFileErrorInvalidKeysBundle
  | WorkspaceCreateFileErrorInvalidManifest
  | WorkspaceCreateFileErrorNoRealmAccess
  | WorkspaceCreateFileErrorOffline
  | WorkspaceCreateFileErrorParentNotAFolder
  | WorkspaceCreateFileErrorParentNotFound
  | WorkspaceCreateFileErrorReadOnlyRealm
  | WorkspaceCreateFileErrorStopped

// WorkspaceCreateFolderError
export enum WorkspaceCreateFolderErrorTag {
    EntryExists = 'WorkspaceCreateFolderErrorEntryExists',
    Internal = 'WorkspaceCreateFolderErrorInternal',
    InvalidCertificate = 'WorkspaceCreateFolderErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceCreateFolderErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceCreateFolderErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceCreateFolderErrorNoRealmAccess',
    Offline = 'WorkspaceCreateFolderErrorOffline',
    ParentNotAFolder = 'WorkspaceCreateFolderErrorParentNotAFolder',
    ParentNotFound = 'WorkspaceCreateFolderErrorParentNotFound',
    ReadOnlyRealm = 'WorkspaceCreateFolderErrorReadOnlyRealm',
    Stopped = 'WorkspaceCreateFolderErrorStopped',
}

export interface WorkspaceCreateFolderErrorEntryExists {
    tag: WorkspaceCreateFolderErrorTag.EntryExists
    error: string
}
export interface WorkspaceCreateFolderErrorInternal {
    tag: WorkspaceCreateFolderErrorTag.Internal
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidCertificate {
    tag: WorkspaceCreateFolderErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidKeysBundle {
    tag: WorkspaceCreateFolderErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidManifest {
    tag: WorkspaceCreateFolderErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceCreateFolderErrorNoRealmAccess {
    tag: WorkspaceCreateFolderErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceCreateFolderErrorOffline {
    tag: WorkspaceCreateFolderErrorTag.Offline
    error: string
}
export interface WorkspaceCreateFolderErrorParentNotAFolder {
    tag: WorkspaceCreateFolderErrorTag.ParentNotAFolder
    error: string
}
export interface WorkspaceCreateFolderErrorParentNotFound {
    tag: WorkspaceCreateFolderErrorTag.ParentNotFound
    error: string
}
export interface WorkspaceCreateFolderErrorReadOnlyRealm {
    tag: WorkspaceCreateFolderErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceCreateFolderErrorStopped {
    tag: WorkspaceCreateFolderErrorTag.Stopped
    error: string
}
export type WorkspaceCreateFolderError =
  | WorkspaceCreateFolderErrorEntryExists
  | WorkspaceCreateFolderErrorInternal
  | WorkspaceCreateFolderErrorInvalidCertificate
  | WorkspaceCreateFolderErrorInvalidKeysBundle
  | WorkspaceCreateFolderErrorInvalidManifest
  | WorkspaceCreateFolderErrorNoRealmAccess
  | WorkspaceCreateFolderErrorOffline
  | WorkspaceCreateFolderErrorParentNotAFolder
  | WorkspaceCreateFolderErrorParentNotFound
  | WorkspaceCreateFolderErrorReadOnlyRealm
  | WorkspaceCreateFolderErrorStopped

// WorkspaceDecryptPathAddrError
export enum WorkspaceDecryptPathAddrErrorTag {
    CorruptedData = 'WorkspaceDecryptPathAddrErrorCorruptedData',
    CorruptedKey = 'WorkspaceDecryptPathAddrErrorCorruptedKey',
    Internal = 'WorkspaceDecryptPathAddrErrorInternal',
    InvalidCertificate = 'WorkspaceDecryptPathAddrErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceDecryptPathAddrErrorInvalidKeysBundle',
    KeyNotFound = 'WorkspaceDecryptPathAddrErrorKeyNotFound',
    NotAllowed = 'WorkspaceDecryptPathAddrErrorNotAllowed',
    Offline = 'WorkspaceDecryptPathAddrErrorOffline',
    Stopped = 'WorkspaceDecryptPathAddrErrorStopped',
}

export interface WorkspaceDecryptPathAddrErrorCorruptedData {
    tag: WorkspaceDecryptPathAddrErrorTag.CorruptedData
    error: string
}
export interface WorkspaceDecryptPathAddrErrorCorruptedKey {
    tag: WorkspaceDecryptPathAddrErrorTag.CorruptedKey
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInternal {
    tag: WorkspaceDecryptPathAddrErrorTag.Internal
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInvalidCertificate {
    tag: WorkspaceDecryptPathAddrErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInvalidKeysBundle {
    tag: WorkspaceDecryptPathAddrErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceDecryptPathAddrErrorKeyNotFound {
    tag: WorkspaceDecryptPathAddrErrorTag.KeyNotFound
    error: string
}
export interface WorkspaceDecryptPathAddrErrorNotAllowed {
    tag: WorkspaceDecryptPathAddrErrorTag.NotAllowed
    error: string
}
export interface WorkspaceDecryptPathAddrErrorOffline {
    tag: WorkspaceDecryptPathAddrErrorTag.Offline
    error: string
}
export interface WorkspaceDecryptPathAddrErrorStopped {
    tag: WorkspaceDecryptPathAddrErrorTag.Stopped
    error: string
}
export type WorkspaceDecryptPathAddrError =
  | WorkspaceDecryptPathAddrErrorCorruptedData
  | WorkspaceDecryptPathAddrErrorCorruptedKey
  | WorkspaceDecryptPathAddrErrorInternal
  | WorkspaceDecryptPathAddrErrorInvalidCertificate
  | WorkspaceDecryptPathAddrErrorInvalidKeysBundle
  | WorkspaceDecryptPathAddrErrorKeyNotFound
  | WorkspaceDecryptPathAddrErrorNotAllowed
  | WorkspaceDecryptPathAddrErrorOffline
  | WorkspaceDecryptPathAddrErrorStopped

// WorkspaceFdCloseError
export enum WorkspaceFdCloseErrorTag {
    BadFileDescriptor = 'WorkspaceFdCloseErrorBadFileDescriptor',
    Internal = 'WorkspaceFdCloseErrorInternal',
    Stopped = 'WorkspaceFdCloseErrorStopped',
}

export interface WorkspaceFdCloseErrorBadFileDescriptor {
    tag: WorkspaceFdCloseErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdCloseErrorInternal {
    tag: WorkspaceFdCloseErrorTag.Internal
    error: string
}
export interface WorkspaceFdCloseErrorStopped {
    tag: WorkspaceFdCloseErrorTag.Stopped
    error: string
}
export type WorkspaceFdCloseError =
  | WorkspaceFdCloseErrorBadFileDescriptor
  | WorkspaceFdCloseErrorInternal
  | WorkspaceFdCloseErrorStopped

// WorkspaceFdFlushError
export enum WorkspaceFdFlushErrorTag {
    BadFileDescriptor = 'WorkspaceFdFlushErrorBadFileDescriptor',
    Internal = 'WorkspaceFdFlushErrorInternal',
    NotInWriteMode = 'WorkspaceFdFlushErrorNotInWriteMode',
    Stopped = 'WorkspaceFdFlushErrorStopped',
}

export interface WorkspaceFdFlushErrorBadFileDescriptor {
    tag: WorkspaceFdFlushErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdFlushErrorInternal {
    tag: WorkspaceFdFlushErrorTag.Internal
    error: string
}
export interface WorkspaceFdFlushErrorNotInWriteMode {
    tag: WorkspaceFdFlushErrorTag.NotInWriteMode
    error: string
}
export interface WorkspaceFdFlushErrorStopped {
    tag: WorkspaceFdFlushErrorTag.Stopped
    error: string
}
export type WorkspaceFdFlushError =
  | WorkspaceFdFlushErrorBadFileDescriptor
  | WorkspaceFdFlushErrorInternal
  | WorkspaceFdFlushErrorNotInWriteMode
  | WorkspaceFdFlushErrorStopped

// WorkspaceFdReadError
export enum WorkspaceFdReadErrorTag {
    BadFileDescriptor = 'WorkspaceFdReadErrorBadFileDescriptor',
    Internal = 'WorkspaceFdReadErrorInternal',
    InvalidBlockAccess = 'WorkspaceFdReadErrorInvalidBlockAccess',
    InvalidCertificate = 'WorkspaceFdReadErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceFdReadErrorInvalidKeysBundle',
    NoRealmAccess = 'WorkspaceFdReadErrorNoRealmAccess',
    NotInReadMode = 'WorkspaceFdReadErrorNotInReadMode',
    Offline = 'WorkspaceFdReadErrorOffline',
    ServerBlockstoreUnavailable = 'WorkspaceFdReadErrorServerBlockstoreUnavailable',
    Stopped = 'WorkspaceFdReadErrorStopped',
}

export interface WorkspaceFdReadErrorBadFileDescriptor {
    tag: WorkspaceFdReadErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdReadErrorInternal {
    tag: WorkspaceFdReadErrorTag.Internal
    error: string
}
export interface WorkspaceFdReadErrorInvalidBlockAccess {
    tag: WorkspaceFdReadErrorTag.InvalidBlockAccess
    error: string
}
export interface WorkspaceFdReadErrorInvalidCertificate {
    tag: WorkspaceFdReadErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceFdReadErrorInvalidKeysBundle {
    tag: WorkspaceFdReadErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceFdReadErrorNoRealmAccess {
    tag: WorkspaceFdReadErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceFdReadErrorNotInReadMode {
    tag: WorkspaceFdReadErrorTag.NotInReadMode
    error: string
}
export interface WorkspaceFdReadErrorOffline {
    tag: WorkspaceFdReadErrorTag.Offline
    error: string
}
export interface WorkspaceFdReadErrorServerBlockstoreUnavailable {
    tag: WorkspaceFdReadErrorTag.ServerBlockstoreUnavailable
    error: string
}
export interface WorkspaceFdReadErrorStopped {
    tag: WorkspaceFdReadErrorTag.Stopped
    error: string
}
export type WorkspaceFdReadError =
  | WorkspaceFdReadErrorBadFileDescriptor
  | WorkspaceFdReadErrorInternal
  | WorkspaceFdReadErrorInvalidBlockAccess
  | WorkspaceFdReadErrorInvalidCertificate
  | WorkspaceFdReadErrorInvalidKeysBundle
  | WorkspaceFdReadErrorNoRealmAccess
  | WorkspaceFdReadErrorNotInReadMode
  | WorkspaceFdReadErrorOffline
  | WorkspaceFdReadErrorServerBlockstoreUnavailable
  | WorkspaceFdReadErrorStopped

// WorkspaceFdResizeError
export enum WorkspaceFdResizeErrorTag {
    BadFileDescriptor = 'WorkspaceFdResizeErrorBadFileDescriptor',
    Internal = 'WorkspaceFdResizeErrorInternal',
    NotInWriteMode = 'WorkspaceFdResizeErrorNotInWriteMode',
}

export interface WorkspaceFdResizeErrorBadFileDescriptor {
    tag: WorkspaceFdResizeErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdResizeErrorInternal {
    tag: WorkspaceFdResizeErrorTag.Internal
    error: string
}
export interface WorkspaceFdResizeErrorNotInWriteMode {
    tag: WorkspaceFdResizeErrorTag.NotInWriteMode
    error: string
}
export type WorkspaceFdResizeError =
  | WorkspaceFdResizeErrorBadFileDescriptor
  | WorkspaceFdResizeErrorInternal
  | WorkspaceFdResizeErrorNotInWriteMode

// WorkspaceFdStatError
export enum WorkspaceFdStatErrorTag {
    BadFileDescriptor = 'WorkspaceFdStatErrorBadFileDescriptor',
    Internal = 'WorkspaceFdStatErrorInternal',
}

export interface WorkspaceFdStatErrorBadFileDescriptor {
    tag: WorkspaceFdStatErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdStatErrorInternal {
    tag: WorkspaceFdStatErrorTag.Internal
    error: string
}
export type WorkspaceFdStatError =
  | WorkspaceFdStatErrorBadFileDescriptor
  | WorkspaceFdStatErrorInternal

// WorkspaceFdWriteError
export enum WorkspaceFdWriteErrorTag {
    BadFileDescriptor = 'WorkspaceFdWriteErrorBadFileDescriptor',
    Internal = 'WorkspaceFdWriteErrorInternal',
    NotInWriteMode = 'WorkspaceFdWriteErrorNotInWriteMode',
}

export interface WorkspaceFdWriteErrorBadFileDescriptor {
    tag: WorkspaceFdWriteErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceFdWriteErrorInternal {
    tag: WorkspaceFdWriteErrorTag.Internal
    error: string
}
export interface WorkspaceFdWriteErrorNotInWriteMode {
    tag: WorkspaceFdWriteErrorTag.NotInWriteMode
    error: string
}
export type WorkspaceFdWriteError =
  | WorkspaceFdWriteErrorBadFileDescriptor
  | WorkspaceFdWriteErrorInternal
  | WorkspaceFdWriteErrorNotInWriteMode

// WorkspaceGeneratePathAddrError
export enum WorkspaceGeneratePathAddrErrorTag {
    Internal = 'WorkspaceGeneratePathAddrErrorInternal',
    InvalidKeysBundle = 'WorkspaceGeneratePathAddrErrorInvalidKeysBundle',
    NoKey = 'WorkspaceGeneratePathAddrErrorNoKey',
    NotAllowed = 'WorkspaceGeneratePathAddrErrorNotAllowed',
    Offline = 'WorkspaceGeneratePathAddrErrorOffline',
    Stopped = 'WorkspaceGeneratePathAddrErrorStopped',
}

export interface WorkspaceGeneratePathAddrErrorInternal {
    tag: WorkspaceGeneratePathAddrErrorTag.Internal
    error: string
}
export interface WorkspaceGeneratePathAddrErrorInvalidKeysBundle {
    tag: WorkspaceGeneratePathAddrErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceGeneratePathAddrErrorNoKey {
    tag: WorkspaceGeneratePathAddrErrorTag.NoKey
    error: string
}
export interface WorkspaceGeneratePathAddrErrorNotAllowed {
    tag: WorkspaceGeneratePathAddrErrorTag.NotAllowed
    error: string
}
export interface WorkspaceGeneratePathAddrErrorOffline {
    tag: WorkspaceGeneratePathAddrErrorTag.Offline
    error: string
}
export interface WorkspaceGeneratePathAddrErrorStopped {
    tag: WorkspaceGeneratePathAddrErrorTag.Stopped
    error: string
}
export type WorkspaceGeneratePathAddrError =
  | WorkspaceGeneratePathAddrErrorInternal
  | WorkspaceGeneratePathAddrErrorInvalidKeysBundle
  | WorkspaceGeneratePathAddrErrorNoKey
  | WorkspaceGeneratePathAddrErrorNotAllowed
  | WorkspaceGeneratePathAddrErrorOffline
  | WorkspaceGeneratePathAddrErrorStopped

// WorkspaceHistoryEntryStat
export enum WorkspaceHistoryEntryStatTag {
    File = 'WorkspaceHistoryEntryStatFile',
    Folder = 'WorkspaceHistoryEntryStatFolder',
}

export interface WorkspaceHistoryEntryStatFile {
    tag: WorkspaceHistoryEntryStatTag.File
    id: VlobID
    parent: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
    size: SizeInt
    lastUpdater: DeviceID
}
export interface WorkspaceHistoryEntryStatFolder {
    tag: WorkspaceHistoryEntryStatTag.Folder
    id: VlobID
    parent: VlobID
    created: DateTime
    updated: DateTime
    version: VersionInt
    lastUpdater: DeviceID
}
export type WorkspaceHistoryEntryStat =
  | WorkspaceHistoryEntryStatFile
  | WorkspaceHistoryEntryStatFolder

// WorkspaceHistoryFdCloseError
export enum WorkspaceHistoryFdCloseErrorTag {
    BadFileDescriptor = 'WorkspaceHistoryFdCloseErrorBadFileDescriptor',
    Internal = 'WorkspaceHistoryFdCloseErrorInternal',
}

export interface WorkspaceHistoryFdCloseErrorBadFileDescriptor {
    tag: WorkspaceHistoryFdCloseErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceHistoryFdCloseErrorInternal {
    tag: WorkspaceHistoryFdCloseErrorTag.Internal
    error: string
}
export type WorkspaceHistoryFdCloseError =
  | WorkspaceHistoryFdCloseErrorBadFileDescriptor
  | WorkspaceHistoryFdCloseErrorInternal

// WorkspaceHistoryFdReadError
export enum WorkspaceHistoryFdReadErrorTag {
    BadFileDescriptor = 'WorkspaceHistoryFdReadErrorBadFileDescriptor',
    Internal = 'WorkspaceHistoryFdReadErrorInternal',
    InvalidBlockAccess = 'WorkspaceHistoryFdReadErrorInvalidBlockAccess',
    InvalidCertificate = 'WorkspaceHistoryFdReadErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceHistoryFdReadErrorInvalidKeysBundle',
    NoRealmAccess = 'WorkspaceHistoryFdReadErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryFdReadErrorOffline',
    ServerBlockstoreUnavailable = 'WorkspaceHistoryFdReadErrorServerBlockstoreUnavailable',
    Stopped = 'WorkspaceHistoryFdReadErrorStopped',
}

export interface WorkspaceHistoryFdReadErrorBadFileDescriptor {
    tag: WorkspaceHistoryFdReadErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceHistoryFdReadErrorInternal {
    tag: WorkspaceHistoryFdReadErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidBlockAccess {
    tag: WorkspaceHistoryFdReadErrorTag.InvalidBlockAccess
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidCertificate {
    tag: WorkspaceHistoryFdReadErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidKeysBundle {
    tag: WorkspaceHistoryFdReadErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryFdReadErrorNoRealmAccess {
    tag: WorkspaceHistoryFdReadErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryFdReadErrorOffline {
    tag: WorkspaceHistoryFdReadErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryFdReadErrorServerBlockstoreUnavailable {
    tag: WorkspaceHistoryFdReadErrorTag.ServerBlockstoreUnavailable
    error: string
}
export interface WorkspaceHistoryFdReadErrorStopped {
    tag: WorkspaceHistoryFdReadErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryFdReadError =
  | WorkspaceHistoryFdReadErrorBadFileDescriptor
  | WorkspaceHistoryFdReadErrorInternal
  | WorkspaceHistoryFdReadErrorInvalidBlockAccess
  | WorkspaceHistoryFdReadErrorInvalidCertificate
  | WorkspaceHistoryFdReadErrorInvalidKeysBundle
  | WorkspaceHistoryFdReadErrorNoRealmAccess
  | WorkspaceHistoryFdReadErrorOffline
  | WorkspaceHistoryFdReadErrorServerBlockstoreUnavailable
  | WorkspaceHistoryFdReadErrorStopped

// WorkspaceHistoryFdStatError
export enum WorkspaceHistoryFdStatErrorTag {
    BadFileDescriptor = 'WorkspaceHistoryFdStatErrorBadFileDescriptor',
    Internal = 'WorkspaceHistoryFdStatErrorInternal',
}

export interface WorkspaceHistoryFdStatErrorBadFileDescriptor {
    tag: WorkspaceHistoryFdStatErrorTag.BadFileDescriptor
    error: string
}
export interface WorkspaceHistoryFdStatErrorInternal {
    tag: WorkspaceHistoryFdStatErrorTag.Internal
    error: string
}
export type WorkspaceHistoryFdStatError =
  | WorkspaceHistoryFdStatErrorBadFileDescriptor
  | WorkspaceHistoryFdStatErrorInternal

// WorkspaceHistoryInternalOnlyError
export enum WorkspaceHistoryInternalOnlyErrorTag {
    Internal = 'WorkspaceHistoryInternalOnlyErrorInternal',
}

export interface WorkspaceHistoryInternalOnlyErrorInternal {
    tag: WorkspaceHistoryInternalOnlyErrorTag.Internal
    error: string
}
export type WorkspaceHistoryInternalOnlyError =
  | WorkspaceHistoryInternalOnlyErrorInternal

// WorkspaceHistoryOpenFileError
export enum WorkspaceHistoryOpenFileErrorTag {
    EntryNotAFile = 'WorkspaceHistoryOpenFileErrorEntryNotAFile',
    EntryNotFound = 'WorkspaceHistoryOpenFileErrorEntryNotFound',
    Internal = 'WorkspaceHistoryOpenFileErrorInternal',
    InvalidCertificate = 'WorkspaceHistoryOpenFileErrorInvalidCertificate',
    InvalidHistory = 'WorkspaceHistoryOpenFileErrorInvalidHistory',
    InvalidKeysBundle = 'WorkspaceHistoryOpenFileErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistoryOpenFileErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceHistoryOpenFileErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryOpenFileErrorOffline',
    Stopped = 'WorkspaceHistoryOpenFileErrorStopped',
}

export interface WorkspaceHistoryOpenFileErrorEntryNotAFile {
    tag: WorkspaceHistoryOpenFileErrorTag.EntryNotAFile
    error: string
}
export interface WorkspaceHistoryOpenFileErrorEntryNotFound {
    tag: WorkspaceHistoryOpenFileErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInternal {
    tag: WorkspaceHistoryOpenFileErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidCertificate {
    tag: WorkspaceHistoryOpenFileErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidHistory {
    tag: WorkspaceHistoryOpenFileErrorTag.InvalidHistory
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidKeysBundle {
    tag: WorkspaceHistoryOpenFileErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidManifest {
    tag: WorkspaceHistoryOpenFileErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistoryOpenFileErrorNoRealmAccess {
    tag: WorkspaceHistoryOpenFileErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryOpenFileErrorOffline {
    tag: WorkspaceHistoryOpenFileErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryOpenFileErrorStopped {
    tag: WorkspaceHistoryOpenFileErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryOpenFileError =
  | WorkspaceHistoryOpenFileErrorEntryNotAFile
  | WorkspaceHistoryOpenFileErrorEntryNotFound
  | WorkspaceHistoryOpenFileErrorInternal
  | WorkspaceHistoryOpenFileErrorInvalidCertificate
  | WorkspaceHistoryOpenFileErrorInvalidHistory
  | WorkspaceHistoryOpenFileErrorInvalidKeysBundle
  | WorkspaceHistoryOpenFileErrorInvalidManifest
  | WorkspaceHistoryOpenFileErrorNoRealmAccess
  | WorkspaceHistoryOpenFileErrorOffline
  | WorkspaceHistoryOpenFileErrorStopped

// WorkspaceHistoryRealmExportDecryptor
export enum WorkspaceHistoryRealmExportDecryptorTag {
    SequesterService = 'WorkspaceHistoryRealmExportDecryptorSequesterService',
    User = 'WorkspaceHistoryRealmExportDecryptorUser',
}

export interface WorkspaceHistoryRealmExportDecryptorSequesterService {
    tag: WorkspaceHistoryRealmExportDecryptorTag.SequesterService
    sequesterServiceId: SequesterServiceID
    privateKeyPemPath: Path
}
export interface WorkspaceHistoryRealmExportDecryptorUser {
    tag: WorkspaceHistoryRealmExportDecryptorTag.User
    access: DeviceAccessStrategy
}
export type WorkspaceHistoryRealmExportDecryptor =
  | WorkspaceHistoryRealmExportDecryptorSequesterService
  | WorkspaceHistoryRealmExportDecryptorUser

// WorkspaceHistorySetTimestampOfInterestError
export enum WorkspaceHistorySetTimestampOfInterestErrorTag {
    EntryNotFound = 'WorkspaceHistorySetTimestampOfInterestErrorEntryNotFound',
    Internal = 'WorkspaceHistorySetTimestampOfInterestErrorInternal',
    InvalidCertificate = 'WorkspaceHistorySetTimestampOfInterestErrorInvalidCertificate',
    InvalidHistory = 'WorkspaceHistorySetTimestampOfInterestErrorInvalidHistory',
    InvalidKeysBundle = 'WorkspaceHistorySetTimestampOfInterestErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistorySetTimestampOfInterestErrorInvalidManifest',
    NewerThanHigherBound = 'WorkspaceHistorySetTimestampOfInterestErrorNewerThanHigherBound',
    NoRealmAccess = 'WorkspaceHistorySetTimestampOfInterestErrorNoRealmAccess',
    Offline = 'WorkspaceHistorySetTimestampOfInterestErrorOffline',
    OlderThanLowerBound = 'WorkspaceHistorySetTimestampOfInterestErrorOlderThanLowerBound',
    Stopped = 'WorkspaceHistorySetTimestampOfInterestErrorStopped',
}

export interface WorkspaceHistorySetTimestampOfInterestErrorEntryNotFound {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInternal {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.Internal
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInvalidCertificate {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInvalidHistory {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.InvalidHistory
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInvalidKeysBundle {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInvalidManifest {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorNewerThanHigherBound {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.NewerThanHigherBound
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorNoRealmAccess {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorOffline {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.Offline
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorOlderThanLowerBound {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.OlderThanLowerBound
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorStopped {
    tag: WorkspaceHistorySetTimestampOfInterestErrorTag.Stopped
    error: string
}
export type WorkspaceHistorySetTimestampOfInterestError =
  | WorkspaceHistorySetTimestampOfInterestErrorEntryNotFound
  | WorkspaceHistorySetTimestampOfInterestErrorInternal
  | WorkspaceHistorySetTimestampOfInterestErrorInvalidCertificate
  | WorkspaceHistorySetTimestampOfInterestErrorInvalidHistory
  | WorkspaceHistorySetTimestampOfInterestErrorInvalidKeysBundle
  | WorkspaceHistorySetTimestampOfInterestErrorInvalidManifest
  | WorkspaceHistorySetTimestampOfInterestErrorNewerThanHigherBound
  | WorkspaceHistorySetTimestampOfInterestErrorNoRealmAccess
  | WorkspaceHistorySetTimestampOfInterestErrorOffline
  | WorkspaceHistorySetTimestampOfInterestErrorOlderThanLowerBound
  | WorkspaceHistorySetTimestampOfInterestErrorStopped

// WorkspaceHistoryStartError
export enum WorkspaceHistoryStartErrorTag {
    CannotOpenRealmExportDatabase = 'WorkspaceHistoryStartErrorCannotOpenRealmExportDatabase',
    IncompleteRealmExportDatabase = 'WorkspaceHistoryStartErrorIncompleteRealmExportDatabase',
    Internal = 'WorkspaceHistoryStartErrorInternal',
    InvalidCertificate = 'WorkspaceHistoryStartErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceHistoryStartErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistoryStartErrorInvalidManifest',
    InvalidRealmExportDatabase = 'WorkspaceHistoryStartErrorInvalidRealmExportDatabase',
    NoHistory = 'WorkspaceHistoryStartErrorNoHistory',
    NoRealmAccess = 'WorkspaceHistoryStartErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryStartErrorOffline',
    Stopped = 'WorkspaceHistoryStartErrorStopped',
    UnsupportedRealmExportDatabaseVersion = 'WorkspaceHistoryStartErrorUnsupportedRealmExportDatabaseVersion',
}

export interface WorkspaceHistoryStartErrorCannotOpenRealmExportDatabase {
    tag: WorkspaceHistoryStartErrorTag.CannotOpenRealmExportDatabase
    error: string
}
export interface WorkspaceHistoryStartErrorIncompleteRealmExportDatabase {
    tag: WorkspaceHistoryStartErrorTag.IncompleteRealmExportDatabase
    error: string
}
export interface WorkspaceHistoryStartErrorInternal {
    tag: WorkspaceHistoryStartErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryStartErrorInvalidCertificate {
    tag: WorkspaceHistoryStartErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryStartErrorInvalidKeysBundle {
    tag: WorkspaceHistoryStartErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryStartErrorInvalidManifest {
    tag: WorkspaceHistoryStartErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistoryStartErrorInvalidRealmExportDatabase {
    tag: WorkspaceHistoryStartErrorTag.InvalidRealmExportDatabase
    error: string
}
export interface WorkspaceHistoryStartErrorNoHistory {
    tag: WorkspaceHistoryStartErrorTag.NoHistory
    error: string
}
export interface WorkspaceHistoryStartErrorNoRealmAccess {
    tag: WorkspaceHistoryStartErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryStartErrorOffline {
    tag: WorkspaceHistoryStartErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryStartErrorStopped {
    tag: WorkspaceHistoryStartErrorTag.Stopped
    error: string
}
export interface WorkspaceHistoryStartErrorUnsupportedRealmExportDatabaseVersion {
    tag: WorkspaceHistoryStartErrorTag.UnsupportedRealmExportDatabaseVersion
    error: string
}
export type WorkspaceHistoryStartError =
  | WorkspaceHistoryStartErrorCannotOpenRealmExportDatabase
  | WorkspaceHistoryStartErrorIncompleteRealmExportDatabase
  | WorkspaceHistoryStartErrorInternal
  | WorkspaceHistoryStartErrorInvalidCertificate
  | WorkspaceHistoryStartErrorInvalidKeysBundle
  | WorkspaceHistoryStartErrorInvalidManifest
  | WorkspaceHistoryStartErrorInvalidRealmExportDatabase
  | WorkspaceHistoryStartErrorNoHistory
  | WorkspaceHistoryStartErrorNoRealmAccess
  | WorkspaceHistoryStartErrorOffline
  | WorkspaceHistoryStartErrorStopped
  | WorkspaceHistoryStartErrorUnsupportedRealmExportDatabaseVersion

// WorkspaceHistoryStatEntryError
export enum WorkspaceHistoryStatEntryErrorTag {
    EntryNotFound = 'WorkspaceHistoryStatEntryErrorEntryNotFound',
    Internal = 'WorkspaceHistoryStatEntryErrorInternal',
    InvalidCertificate = 'WorkspaceHistoryStatEntryErrorInvalidCertificate',
    InvalidHistory = 'WorkspaceHistoryStatEntryErrorInvalidHistory',
    InvalidKeysBundle = 'WorkspaceHistoryStatEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistoryStatEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceHistoryStatEntryErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryStatEntryErrorOffline',
    Stopped = 'WorkspaceHistoryStatEntryErrorStopped',
}

export interface WorkspaceHistoryStatEntryErrorEntryNotFound {
    tag: WorkspaceHistoryStatEntryErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInternal {
    tag: WorkspaceHistoryStatEntryErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidCertificate {
    tag: WorkspaceHistoryStatEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidHistory {
    tag: WorkspaceHistoryStatEntryErrorTag.InvalidHistory
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidKeysBundle {
    tag: WorkspaceHistoryStatEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidManifest {
    tag: WorkspaceHistoryStatEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistoryStatEntryErrorNoRealmAccess {
    tag: WorkspaceHistoryStatEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryStatEntryErrorOffline {
    tag: WorkspaceHistoryStatEntryErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryStatEntryErrorStopped {
    tag: WorkspaceHistoryStatEntryErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryStatEntryError =
  | WorkspaceHistoryStatEntryErrorEntryNotFound
  | WorkspaceHistoryStatEntryErrorInternal
  | WorkspaceHistoryStatEntryErrorInvalidCertificate
  | WorkspaceHistoryStatEntryErrorInvalidHistory
  | WorkspaceHistoryStatEntryErrorInvalidKeysBundle
  | WorkspaceHistoryStatEntryErrorInvalidManifest
  | WorkspaceHistoryStatEntryErrorNoRealmAccess
  | WorkspaceHistoryStatEntryErrorOffline
  | WorkspaceHistoryStatEntryErrorStopped

// WorkspaceHistoryStatFolderChildrenError
export enum WorkspaceHistoryStatFolderChildrenErrorTag {
    EntryIsFile = 'WorkspaceHistoryStatFolderChildrenErrorEntryIsFile',
    EntryNotFound = 'WorkspaceHistoryStatFolderChildrenErrorEntryNotFound',
    Internal = 'WorkspaceHistoryStatFolderChildrenErrorInternal',
    InvalidCertificate = 'WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate',
    InvalidHistory = 'WorkspaceHistoryStatFolderChildrenErrorInvalidHistory',
    InvalidKeysBundle = 'WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceHistoryStatFolderChildrenErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess',
    Offline = 'WorkspaceHistoryStatFolderChildrenErrorOffline',
    Stopped = 'WorkspaceHistoryStatFolderChildrenErrorStopped',
}

export interface WorkspaceHistoryStatFolderChildrenErrorEntryIsFile {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.EntryIsFile
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorEntryNotFound {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInternal {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.Internal
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidHistory {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.InvalidHistory
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidManifest {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorOffline {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.Offline
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorStopped {
    tag: WorkspaceHistoryStatFolderChildrenErrorTag.Stopped
    error: string
}
export type WorkspaceHistoryStatFolderChildrenError =
  | WorkspaceHistoryStatFolderChildrenErrorEntryIsFile
  | WorkspaceHistoryStatFolderChildrenErrorEntryNotFound
  | WorkspaceHistoryStatFolderChildrenErrorInternal
  | WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate
  | WorkspaceHistoryStatFolderChildrenErrorInvalidHistory
  | WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle
  | WorkspaceHistoryStatFolderChildrenErrorInvalidManifest
  | WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess
  | WorkspaceHistoryStatFolderChildrenErrorOffline
  | WorkspaceHistoryStatFolderChildrenErrorStopped

// WorkspaceInfoError
export enum WorkspaceInfoErrorTag {
    Internal = 'WorkspaceInfoErrorInternal',
}

export interface WorkspaceInfoErrorInternal {
    tag: WorkspaceInfoErrorTag.Internal
    error: string
}
export type WorkspaceInfoError =
  | WorkspaceInfoErrorInternal

// WorkspaceIsFileContentLocalError
export enum WorkspaceIsFileContentLocalErrorTag {
    EntryNotFound = 'WorkspaceIsFileContentLocalErrorEntryNotFound',
    Internal = 'WorkspaceIsFileContentLocalErrorInternal',
    InvalidCertificate = 'WorkspaceIsFileContentLocalErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceIsFileContentLocalErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceIsFileContentLocalErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceIsFileContentLocalErrorNoRealmAccess',
    NotAFile = 'WorkspaceIsFileContentLocalErrorNotAFile',
    Offline = 'WorkspaceIsFileContentLocalErrorOffline',
    Stopped = 'WorkspaceIsFileContentLocalErrorStopped',
}

export interface WorkspaceIsFileContentLocalErrorEntryNotFound {
    tag: WorkspaceIsFileContentLocalErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceIsFileContentLocalErrorInternal {
    tag: WorkspaceIsFileContentLocalErrorTag.Internal
    error: string
}
export interface WorkspaceIsFileContentLocalErrorInvalidCertificate {
    tag: WorkspaceIsFileContentLocalErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceIsFileContentLocalErrorInvalidKeysBundle {
    tag: WorkspaceIsFileContentLocalErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceIsFileContentLocalErrorInvalidManifest {
    tag: WorkspaceIsFileContentLocalErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceIsFileContentLocalErrorNoRealmAccess {
    tag: WorkspaceIsFileContentLocalErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceIsFileContentLocalErrorNotAFile {
    tag: WorkspaceIsFileContentLocalErrorTag.NotAFile
    error: string
}
export interface WorkspaceIsFileContentLocalErrorOffline {
    tag: WorkspaceIsFileContentLocalErrorTag.Offline
    error: string
}
export interface WorkspaceIsFileContentLocalErrorStopped {
    tag: WorkspaceIsFileContentLocalErrorTag.Stopped
    error: string
}
export type WorkspaceIsFileContentLocalError =
  | WorkspaceIsFileContentLocalErrorEntryNotFound
  | WorkspaceIsFileContentLocalErrorInternal
  | WorkspaceIsFileContentLocalErrorInvalidCertificate
  | WorkspaceIsFileContentLocalErrorInvalidKeysBundle
  | WorkspaceIsFileContentLocalErrorInvalidManifest
  | WorkspaceIsFileContentLocalErrorNoRealmAccess
  | WorkspaceIsFileContentLocalErrorNotAFile
  | WorkspaceIsFileContentLocalErrorOffline
  | WorkspaceIsFileContentLocalErrorStopped

// WorkspaceMountError
export enum WorkspaceMountErrorTag {
    Disabled = 'WorkspaceMountErrorDisabled',
    Internal = 'WorkspaceMountErrorInternal',
}

export interface WorkspaceMountErrorDisabled {
    tag: WorkspaceMountErrorTag.Disabled
    error: string
}
export interface WorkspaceMountErrorInternal {
    tag: WorkspaceMountErrorTag.Internal
    error: string
}
export type WorkspaceMountError =
  | WorkspaceMountErrorDisabled
  | WorkspaceMountErrorInternal

// WorkspaceMoveEntryError
export enum WorkspaceMoveEntryErrorTag {
    CannotMoveRoot = 'WorkspaceMoveEntryErrorCannotMoveRoot',
    DestinationExists = 'WorkspaceMoveEntryErrorDestinationExists',
    DestinationNotFound = 'WorkspaceMoveEntryErrorDestinationNotFound',
    Internal = 'WorkspaceMoveEntryErrorInternal',
    InvalidCertificate = 'WorkspaceMoveEntryErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceMoveEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceMoveEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceMoveEntryErrorNoRealmAccess',
    Offline = 'WorkspaceMoveEntryErrorOffline',
    ReadOnlyRealm = 'WorkspaceMoveEntryErrorReadOnlyRealm',
    SourceNotFound = 'WorkspaceMoveEntryErrorSourceNotFound',
    Stopped = 'WorkspaceMoveEntryErrorStopped',
}

export interface WorkspaceMoveEntryErrorCannotMoveRoot {
    tag: WorkspaceMoveEntryErrorTag.CannotMoveRoot
    error: string
}
export interface WorkspaceMoveEntryErrorDestinationExists {
    tag: WorkspaceMoveEntryErrorTag.DestinationExists
    error: string
}
export interface WorkspaceMoveEntryErrorDestinationNotFound {
    tag: WorkspaceMoveEntryErrorTag.DestinationNotFound
    error: string
}
export interface WorkspaceMoveEntryErrorInternal {
    tag: WorkspaceMoveEntryErrorTag.Internal
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidCertificate {
    tag: WorkspaceMoveEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidKeysBundle {
    tag: WorkspaceMoveEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidManifest {
    tag: WorkspaceMoveEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceMoveEntryErrorNoRealmAccess {
    tag: WorkspaceMoveEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceMoveEntryErrorOffline {
    tag: WorkspaceMoveEntryErrorTag.Offline
    error: string
}
export interface WorkspaceMoveEntryErrorReadOnlyRealm {
    tag: WorkspaceMoveEntryErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceMoveEntryErrorSourceNotFound {
    tag: WorkspaceMoveEntryErrorTag.SourceNotFound
    error: string
}
export interface WorkspaceMoveEntryErrorStopped {
    tag: WorkspaceMoveEntryErrorTag.Stopped
    error: string
}
export type WorkspaceMoveEntryError =
  | WorkspaceMoveEntryErrorCannotMoveRoot
  | WorkspaceMoveEntryErrorDestinationExists
  | WorkspaceMoveEntryErrorDestinationNotFound
  | WorkspaceMoveEntryErrorInternal
  | WorkspaceMoveEntryErrorInvalidCertificate
  | WorkspaceMoveEntryErrorInvalidKeysBundle
  | WorkspaceMoveEntryErrorInvalidManifest
  | WorkspaceMoveEntryErrorNoRealmAccess
  | WorkspaceMoveEntryErrorOffline
  | WorkspaceMoveEntryErrorReadOnlyRealm
  | WorkspaceMoveEntryErrorSourceNotFound
  | WorkspaceMoveEntryErrorStopped

// WorkspaceOpenFileError
export enum WorkspaceOpenFileErrorTag {
    EntryExistsInCreateNewMode = 'WorkspaceOpenFileErrorEntryExistsInCreateNewMode',
    EntryNotAFile = 'WorkspaceOpenFileErrorEntryNotAFile',
    EntryNotFound = 'WorkspaceOpenFileErrorEntryNotFound',
    Internal = 'WorkspaceOpenFileErrorInternal',
    InvalidCertificate = 'WorkspaceOpenFileErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceOpenFileErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceOpenFileErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceOpenFileErrorNoRealmAccess',
    Offline = 'WorkspaceOpenFileErrorOffline',
    ReadOnlyRealm = 'WorkspaceOpenFileErrorReadOnlyRealm',
    Stopped = 'WorkspaceOpenFileErrorStopped',
}

export interface WorkspaceOpenFileErrorEntryExistsInCreateNewMode {
    tag: WorkspaceOpenFileErrorTag.EntryExistsInCreateNewMode
    error: string
}
export interface WorkspaceOpenFileErrorEntryNotAFile {
    tag: WorkspaceOpenFileErrorTag.EntryNotAFile
    error: string
}
export interface WorkspaceOpenFileErrorEntryNotFound {
    tag: WorkspaceOpenFileErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceOpenFileErrorInternal {
    tag: WorkspaceOpenFileErrorTag.Internal
    error: string
}
export interface WorkspaceOpenFileErrorInvalidCertificate {
    tag: WorkspaceOpenFileErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceOpenFileErrorInvalidKeysBundle {
    tag: WorkspaceOpenFileErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceOpenFileErrorInvalidManifest {
    tag: WorkspaceOpenFileErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceOpenFileErrorNoRealmAccess {
    tag: WorkspaceOpenFileErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceOpenFileErrorOffline {
    tag: WorkspaceOpenFileErrorTag.Offline
    error: string
}
export interface WorkspaceOpenFileErrorReadOnlyRealm {
    tag: WorkspaceOpenFileErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceOpenFileErrorStopped {
    tag: WorkspaceOpenFileErrorTag.Stopped
    error: string
}
export type WorkspaceOpenFileError =
  | WorkspaceOpenFileErrorEntryExistsInCreateNewMode
  | WorkspaceOpenFileErrorEntryNotAFile
  | WorkspaceOpenFileErrorEntryNotFound
  | WorkspaceOpenFileErrorInternal
  | WorkspaceOpenFileErrorInvalidCertificate
  | WorkspaceOpenFileErrorInvalidKeysBundle
  | WorkspaceOpenFileErrorInvalidManifest
  | WorkspaceOpenFileErrorNoRealmAccess
  | WorkspaceOpenFileErrorOffline
  | WorkspaceOpenFileErrorReadOnlyRealm
  | WorkspaceOpenFileErrorStopped

// WorkspaceRemoveEntryError
export enum WorkspaceRemoveEntryErrorTag {
    CannotRemoveRoot = 'WorkspaceRemoveEntryErrorCannotRemoveRoot',
    EntryIsFile = 'WorkspaceRemoveEntryErrorEntryIsFile',
    EntryIsFolder = 'WorkspaceRemoveEntryErrorEntryIsFolder',
    EntryIsNonEmptyFolder = 'WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder',
    EntryNotFound = 'WorkspaceRemoveEntryErrorEntryNotFound',
    Internal = 'WorkspaceRemoveEntryErrorInternal',
    InvalidCertificate = 'WorkspaceRemoveEntryErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceRemoveEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceRemoveEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceRemoveEntryErrorNoRealmAccess',
    Offline = 'WorkspaceRemoveEntryErrorOffline',
    ReadOnlyRealm = 'WorkspaceRemoveEntryErrorReadOnlyRealm',
    Stopped = 'WorkspaceRemoveEntryErrorStopped',
}

export interface WorkspaceRemoveEntryErrorCannotRemoveRoot {
    tag: WorkspaceRemoveEntryErrorTag.CannotRemoveRoot
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsFile {
    tag: WorkspaceRemoveEntryErrorTag.EntryIsFile
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsFolder {
    tag: WorkspaceRemoveEntryErrorTag.EntryIsFolder
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder {
    tag: WorkspaceRemoveEntryErrorTag.EntryIsNonEmptyFolder
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryNotFound {
    tag: WorkspaceRemoveEntryErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceRemoveEntryErrorInternal {
    tag: WorkspaceRemoveEntryErrorTag.Internal
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidCertificate {
    tag: WorkspaceRemoveEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidKeysBundle {
    tag: WorkspaceRemoveEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidManifest {
    tag: WorkspaceRemoveEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceRemoveEntryErrorNoRealmAccess {
    tag: WorkspaceRemoveEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceRemoveEntryErrorOffline {
    tag: WorkspaceRemoveEntryErrorTag.Offline
    error: string
}
export interface WorkspaceRemoveEntryErrorReadOnlyRealm {
    tag: WorkspaceRemoveEntryErrorTag.ReadOnlyRealm
    error: string
}
export interface WorkspaceRemoveEntryErrorStopped {
    tag: WorkspaceRemoveEntryErrorTag.Stopped
    error: string
}
export type WorkspaceRemoveEntryError =
  | WorkspaceRemoveEntryErrorCannotRemoveRoot
  | WorkspaceRemoveEntryErrorEntryIsFile
  | WorkspaceRemoveEntryErrorEntryIsFolder
  | WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder
  | WorkspaceRemoveEntryErrorEntryNotFound
  | WorkspaceRemoveEntryErrorInternal
  | WorkspaceRemoveEntryErrorInvalidCertificate
  | WorkspaceRemoveEntryErrorInvalidKeysBundle
  | WorkspaceRemoveEntryErrorInvalidManifest
  | WorkspaceRemoveEntryErrorNoRealmAccess
  | WorkspaceRemoveEntryErrorOffline
  | WorkspaceRemoveEntryErrorReadOnlyRealm
  | WorkspaceRemoveEntryErrorStopped

// WorkspaceStatEntryError
export enum WorkspaceStatEntryErrorTag {
    EntryNotFound = 'WorkspaceStatEntryErrorEntryNotFound',
    Internal = 'WorkspaceStatEntryErrorInternal',
    InvalidCertificate = 'WorkspaceStatEntryErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceStatEntryErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceStatEntryErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceStatEntryErrorNoRealmAccess',
    Offline = 'WorkspaceStatEntryErrorOffline',
    Stopped = 'WorkspaceStatEntryErrorStopped',
}

export interface WorkspaceStatEntryErrorEntryNotFound {
    tag: WorkspaceStatEntryErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceStatEntryErrorInternal {
    tag: WorkspaceStatEntryErrorTag.Internal
    error: string
}
export interface WorkspaceStatEntryErrorInvalidCertificate {
    tag: WorkspaceStatEntryErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceStatEntryErrorInvalidKeysBundle {
    tag: WorkspaceStatEntryErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceStatEntryErrorInvalidManifest {
    tag: WorkspaceStatEntryErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceStatEntryErrorNoRealmAccess {
    tag: WorkspaceStatEntryErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceStatEntryErrorOffline {
    tag: WorkspaceStatEntryErrorTag.Offline
    error: string
}
export interface WorkspaceStatEntryErrorStopped {
    tag: WorkspaceStatEntryErrorTag.Stopped
    error: string
}
export type WorkspaceStatEntryError =
  | WorkspaceStatEntryErrorEntryNotFound
  | WorkspaceStatEntryErrorInternal
  | WorkspaceStatEntryErrorInvalidCertificate
  | WorkspaceStatEntryErrorInvalidKeysBundle
  | WorkspaceStatEntryErrorInvalidManifest
  | WorkspaceStatEntryErrorNoRealmAccess
  | WorkspaceStatEntryErrorOffline
  | WorkspaceStatEntryErrorStopped

// WorkspaceStatFolderChildrenError
export enum WorkspaceStatFolderChildrenErrorTag {
    EntryIsFile = 'WorkspaceStatFolderChildrenErrorEntryIsFile',
    EntryNotFound = 'WorkspaceStatFolderChildrenErrorEntryNotFound',
    Internal = 'WorkspaceStatFolderChildrenErrorInternal',
    InvalidCertificate = 'WorkspaceStatFolderChildrenErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceStatFolderChildrenErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceStatFolderChildrenErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceStatFolderChildrenErrorNoRealmAccess',
    Offline = 'WorkspaceStatFolderChildrenErrorOffline',
    Stopped = 'WorkspaceStatFolderChildrenErrorStopped',
}

export interface WorkspaceStatFolderChildrenErrorEntryIsFile {
    tag: WorkspaceStatFolderChildrenErrorTag.EntryIsFile
    error: string
}
export interface WorkspaceStatFolderChildrenErrorEntryNotFound {
    tag: WorkspaceStatFolderChildrenErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInternal {
    tag: WorkspaceStatFolderChildrenErrorTag.Internal
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidCertificate {
    tag: WorkspaceStatFolderChildrenErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidKeysBundle {
    tag: WorkspaceStatFolderChildrenErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidManifest {
    tag: WorkspaceStatFolderChildrenErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceStatFolderChildrenErrorNoRealmAccess {
    tag: WorkspaceStatFolderChildrenErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceStatFolderChildrenErrorOffline {
    tag: WorkspaceStatFolderChildrenErrorTag.Offline
    error: string
}
export interface WorkspaceStatFolderChildrenErrorStopped {
    tag: WorkspaceStatFolderChildrenErrorTag.Stopped
    error: string
}
export type WorkspaceStatFolderChildrenError =
  | WorkspaceStatFolderChildrenErrorEntryIsFile
  | WorkspaceStatFolderChildrenErrorEntryNotFound
  | WorkspaceStatFolderChildrenErrorInternal
  | WorkspaceStatFolderChildrenErrorInvalidCertificate
  | WorkspaceStatFolderChildrenErrorInvalidKeysBundle
  | WorkspaceStatFolderChildrenErrorInvalidManifest
  | WorkspaceStatFolderChildrenErrorNoRealmAccess
  | WorkspaceStatFolderChildrenErrorOffline
  | WorkspaceStatFolderChildrenErrorStopped

// WorkspaceStopError
export enum WorkspaceStopErrorTag {
    Internal = 'WorkspaceStopErrorInternal',
}

export interface WorkspaceStopErrorInternal {
    tag: WorkspaceStopErrorTag.Internal
    error: string
}
export type WorkspaceStopError =
  | WorkspaceStopErrorInternal

// WorkspaceStorageCacheSize
export enum WorkspaceStorageCacheSizeTag {
    Custom = 'WorkspaceStorageCacheSizeCustom',
    Default = 'WorkspaceStorageCacheSizeDefault',
}

export interface WorkspaceStorageCacheSizeCustom {
    tag: WorkspaceStorageCacheSizeTag.Custom
    size: CacheSize
}
export interface WorkspaceStorageCacheSizeDefault {
    tag: WorkspaceStorageCacheSizeTag.Default
}
export type WorkspaceStorageCacheSize =
  | WorkspaceStorageCacheSizeCustom
  | WorkspaceStorageCacheSizeDefault

// WorkspaceWatchEntryOneShotError
export enum WorkspaceWatchEntryOneShotErrorTag {
    EntryNotFound = 'WorkspaceWatchEntryOneShotErrorEntryNotFound',
    Internal = 'WorkspaceWatchEntryOneShotErrorInternal',
    InvalidCertificate = 'WorkspaceWatchEntryOneShotErrorInvalidCertificate',
    InvalidKeysBundle = 'WorkspaceWatchEntryOneShotErrorInvalidKeysBundle',
    InvalidManifest = 'WorkspaceWatchEntryOneShotErrorInvalidManifest',
    NoRealmAccess = 'WorkspaceWatchEntryOneShotErrorNoRealmAccess',
    Offline = 'WorkspaceWatchEntryOneShotErrorOffline',
    Stopped = 'WorkspaceWatchEntryOneShotErrorStopped',
}

export interface WorkspaceWatchEntryOneShotErrorEntryNotFound {
    tag: WorkspaceWatchEntryOneShotErrorTag.EntryNotFound
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInternal {
    tag: WorkspaceWatchEntryOneShotErrorTag.Internal
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidCertificate {
    tag: WorkspaceWatchEntryOneShotErrorTag.InvalidCertificate
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidKeysBundle {
    tag: WorkspaceWatchEntryOneShotErrorTag.InvalidKeysBundle
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidManifest {
    tag: WorkspaceWatchEntryOneShotErrorTag.InvalidManifest
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorNoRealmAccess {
    tag: WorkspaceWatchEntryOneShotErrorTag.NoRealmAccess
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorOffline {
    tag: WorkspaceWatchEntryOneShotErrorTag.Offline
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorStopped {
    tag: WorkspaceWatchEntryOneShotErrorTag.Stopped
    error: string
}
export type WorkspaceWatchEntryOneShotError =
  | WorkspaceWatchEntryOneShotErrorEntryNotFound
  | WorkspaceWatchEntryOneShotErrorInternal
  | WorkspaceWatchEntryOneShotErrorInvalidCertificate
  | WorkspaceWatchEntryOneShotErrorInvalidKeysBundle
  | WorkspaceWatchEntryOneShotErrorInvalidManifest
  | WorkspaceWatchEntryOneShotErrorNoRealmAccess
  | WorkspaceWatchEntryOneShotErrorOffline
  | WorkspaceWatchEntryOneShotErrorStopped

// X509URIFlavorValue
export enum X509URIFlavorValueTag {
    PKCS11 = 'X509URIFlavorValuePKCS11',
    WindowsCNG = 'X509URIFlavorValueWindowsCNG',
}

export interface X509URIFlavorValuePKCS11 {
    tag: X509URIFlavorValueTag.PKCS11
    x1: X509Pkcs11URI
}
export interface X509URIFlavorValueWindowsCNG {
    tag: X509URIFlavorValueTag.WindowsCNG
    x1: X509WindowsCngURI
}
export type X509URIFlavorValue =
  | X509URIFlavorValuePKCS11
  | X509URIFlavorValueWindowsCNG

export interface LibParsecPlugin {
    accountCreate1SendValidationEmail(
        config_dir: Path,
        addr: ParsecAddr,
        email: EmailAddress
    ): Promise<Result<null, AccountCreateSendValidationEmailError>>
    accountCreate2CheckValidationCode(
        config_dir: Path,
        addr: ParsecAddr,
        validation_code: string,
        email: EmailAddress
    ): Promise<Result<null, AccountCreateError>>
    accountCreate3Proceed(
        config_dir: Path,
        addr: ParsecAddr,
        validation_code: string,
        human_handle: HumanHandle,
        auth_method_strategy: AccountAuthMethodStrategy
    ): Promise<Result<null, AccountCreateError>>
    accountCreateAuthMethod(
        account: Handle,
        auth_method_strategy: AccountAuthMethodStrategy
    ): Promise<Result<null, AccountCreateAuthMethodError>>
    accountCreateRegistrationDevice(
        account: Handle,
        existing_local_device_access: DeviceAccessStrategy
    ): Promise<Result<null, AccountCreateRegistrationDeviceError>>
    accountDelete1SendValidationEmail(
        account: Handle
    ): Promise<Result<null, AccountDeleteSendValidationEmailError>>
    accountDelete2Proceed(
        account: Handle,
        validation_code: string
    ): Promise<Result<null, AccountDeleteProceedError>>
    accountDisableAuthMethod(
        account: Handle,
        auth_method_id: AccountAuthMethodID
    ): Promise<Result<null, AccountDisableAuthMethodError>>
    accountInfo(
        account: Handle
    ): Promise<Result<AccountInfo, AccountInfoError>>
    accountListAuthMethods(
        account: Handle
    ): Promise<Result<Array<AuthMethodInfo>, AccountListAuthMethodsError>>
    accountListInvitations(
        account: Handle
    ): Promise<Result<Array<[ParsecInvitationAddr, OrganizationID, AccessToken, InvitationType]>, AccountListInvitationsError>>
    accountListOrganizations(
        account: Handle
    ): Promise<Result<AccountOrganizations, AccountListOrganizationsError>>
    accountListRegistrationDevices(
        account: Handle
    ): Promise<Result<Array<[OrganizationID, UserID]>, AccountListRegistrationDevicesError>>
    accountLogin(
        config_dir: Path,
        addr: ParsecAddr,
        login_strategy: AccountLoginStrategy
    ): Promise<Result<Handle, AccountLoginError>>
    accountLogout(
        account: Handle
    ): Promise<Result<null, AccountLogoutError>>
    accountRecover1SendValidationEmail(
        config_dir: Path,
        addr: ParsecAddr,
        email: EmailAddress
    ): Promise<Result<null, AccountRecoverSendValidationEmailError>>
    accountRecover2Proceed(
        config_dir: Path,
        addr: ParsecAddr,
        validation_code: string,
        email: EmailAddress,
        auth_method_strategy: AccountAuthMethodStrategy
    ): Promise<Result<null, AccountRecoverProceedError>>
    accountRegisterNewDevice(
        account: Handle,
        organization_id: OrganizationID,
        user_id: UserID,
        new_device_label: DeviceLabel,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, AccountRegisterNewDeviceError>>
    archiveDevice(
        config_dir: Path,
        device_path: Path
    ): Promise<Result<null, ArchiveDeviceError>>
    bootstrapOrganization(
        config: ClientConfig,
        bootstrap_organization_addr: ParsecOrganizationBootstrapAddr,
        save_strategy: DeviceSaveStrategy,
        human_handle: HumanHandle,
        device_label: DeviceLabel,
        sequester_authority_verify_key_pem: string | null
    ): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
    buildParsecAddr(
        hostname: string,
        port: U16 | null,
        use_ssl: boolean
    ): Promise<ParsecAddr>
    buildParsecOrganizationBootstrapAddr(
        addr: ParsecAddr,
        organization_id: OrganizationID
    ): Promise<ParsecOrganizationBootstrapAddr>
    cancel(
        canceller: Handle
    ): Promise<Result<null, CancelError>>
    claimerDeviceFinalizeSaveLocalDevice(
        handle: Handle,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimFinalizeError>>
    claimerDeviceInProgress1DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, ClaimInProgressError>>
    claimerDeviceInProgress1DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>>
    claimerDeviceInProgress2DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>>
    claimerDeviceInProgress3DoClaim(
        canceller: Handle,
        handle: Handle,
        requested_device_label: DeviceLabel
    ): Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>>
    claimerDeviceInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>>
    claimerGreeterAbortOperation(
        handle: Handle
    ): Promise<Result<null, ClaimerGreeterAbortOperationError>>
    claimerRetrieveInfo(
        config: ClientConfig,
        addr: ParsecInvitationAddr
    ): Promise<Result<AnyClaimRetrievedInfo, ClaimerRetrieveInfoError>>
    claimerShamirRecoveryAddShare(
        recipient_pick_handle: Handle,
        share_handle: Handle
    ): Promise<Result<ShamirRecoveryClaimMaybeRecoverDeviceInfo, ShamirRecoveryClaimAddShareError>>
    claimerShamirRecoveryFinalizeSaveLocalDevice(
        handle: Handle,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimFinalizeError>>
    claimerShamirRecoveryInProgress1DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, ClaimInProgressError>>
    claimerShamirRecoveryInProgress1DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryClaimInProgress2Info, ClaimInProgressError>>
    claimerShamirRecoveryInProgress2DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryClaimInProgress3Info, ClaimInProgressError>>
    claimerShamirRecoveryInProgress3DoClaim(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryClaimShareInfo, ClaimInProgressError>>
    claimerShamirRecoveryInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryClaimInProgress1Info, ClaimInProgressError>>
    claimerShamirRecoveryPickRecipient(
        handle: Handle,
        recipient_user_id: UserID
    ): Promise<Result<ShamirRecoveryClaimInitialInfo, ShamirRecoveryClaimPickRecipientError>>
    claimerShamirRecoveryRecoverDevice(
        handle: Handle,
        requested_device_label: DeviceLabel
    ): Promise<Result<ShamirRecoveryClaimMaybeFinalizeInfo, ShamirRecoveryClaimRecoverDeviceError>>
    claimerUserFinalizeSaveLocalDevice(
        handle: Handle,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ClaimFinalizeError>>
    claimerUserInProgress1DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, ClaimInProgressError>>
    claimerUserInProgress1DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>>
    claimerUserInProgress2DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>>
    claimerUserInProgress3DoClaim(
        canceller: Handle,
        handle: Handle,
        requested_device_label: DeviceLabel,
        requested_human_handle: HumanHandle
    ): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>>
    claimerUserInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
    claimerUserListInitialInfo(
        handle: Handle
    ): Promise<Result<Array<UserClaimInitialInfo>, UserClaimListInitialInfosError>>
    claimerUserWaitAllPeers(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
    clientAcceptAsyncEnrollment(
        client: Handle,
        profile: UserProfile,
        enrollment_id: AsyncEnrollmentID,
        identity_strategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy
    ): Promise<Result<null, ClientAcceptAsyncEnrollmentError>>
    clientAcceptTos(
        client: Handle,
        tos_updated_on: DateTime
    ): Promise<Result<null, ClientAcceptTosError>>
    clientCancelInvitation(
        client: Handle,
        token: AccessToken
    ): Promise<Result<null, ClientCancelInvitationError>>
    clientCreateWorkspace(
        client: Handle,
        name: EntryName
    ): Promise<Result<VlobID, ClientCreateWorkspaceError>>
    clientDeleteShamirRecovery(
        client_handle: Handle
    ): Promise<Result<null, ClientDeleteShamirRecoveryError>>
    clientExportRecoveryDevice(
        client_handle: Handle,
        device_label: DeviceLabel
    ): Promise<Result<[string, Uint8Array], ClientExportRecoveryDeviceError>>
    clientForgetAllCertificates(
        client: Handle
    ): Promise<Result<null, ClientForgetAllCertificatesError>>
    clientGetAsyncEnrollmentAddr(
        client: Handle
    ): Promise<Result<ParsecAsyncEnrollmentAddr, ClientGetAsyncEnrollmentAddrError>>
    clientGetOrganizationBootstrapDate(
        client_handle: Handle
    ): Promise<Result<DateTime, ClientGetOrganizationBootstrapDateError>>
    clientGetSelfShamirRecovery(
        client_handle: Handle
    ): Promise<Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError>>
    clientGetTos(
        client: Handle
    ): Promise<Result<Tos, ClientGetTosError>>
    clientGetUserDevice(
        client: Handle,
        device: DeviceID
    ): Promise<Result<[UserInfo, DeviceInfo], ClientGetUserDeviceError>>
    clientGetUserInfo(
        client: Handle,
        user_id: UserID
    ): Promise<Result<UserInfo, ClientGetUserInfoError>>
    clientInfo(
        client: Handle
    ): Promise<Result<ClientInfo, ClientInfoError>>
    clientListAsyncEnrollments(
        client: Handle
    ): Promise<Result<Array<AsyncEnrollmentUntrusted>, ClientListAsyncEnrollmentsError>>
    clientListFrozenUsers(
        client_handle: Handle
    ): Promise<Result<Array<UserID>, ClientListFrozenUsersError>>
    clientListInvitations(
        client: Handle
    ): Promise<Result<Array<InviteListItem>, ListInvitationsError>>
    clientListShamirRecoveriesForOthers(
        client_handle: Handle
    ): Promise<Result<Array<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError>>
    clientListUserDevices(
        client: Handle,
        user: UserID
    ): Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>>
    clientListUsers(
        client: Handle,
        skip_revoked: boolean
    ): Promise<Result<Array<UserInfo>, ClientListUsersError>>
    clientListWorkspaceUsers(
        client: Handle,
        realm_id: VlobID
    ): Promise<Result<Array<WorkspaceUserAccessInfo>, ClientListWorkspaceUsersError>>
    clientListWorkspaces(
        client: Handle
    ): Promise<Result<Array<WorkspaceInfo>, ClientListWorkspacesError>>
    clientNewDeviceInvitation(
        client: Handle,
        send_email: boolean
    ): Promise<Result<NewInvitationInfo, ClientNewDeviceInvitationError>>
    clientNewShamirRecoveryInvitation(
        client: Handle,
        claimer_user_id: UserID,
        send_email: boolean
    ): Promise<Result<NewInvitationInfo, ClientNewShamirRecoveryInvitationError>>
    clientNewUserInvitation(
        client: Handle,
        claimer_email: EmailAddress,
        send_email: boolean
    ): Promise<Result<NewInvitationInfo, ClientNewUserInvitationError>>
    clientOrganizationInfo(
        client_handle: Handle
    ): Promise<Result<OrganizationInfo, ClientOrganizationInfoError>>
    clientRejectAsyncEnrollment(
        client: Handle,
        enrollment_id: AsyncEnrollmentID
    ): Promise<Result<null, ClientRejectAsyncEnrollmentError>>
    clientRenameWorkspace(
        client: Handle,
        realm_id: VlobID,
        new_name: EntryName
    ): Promise<Result<null, ClientRenameWorkspaceError>>
    clientRevokeUser(
        client: Handle,
        user: UserID
    ): Promise<Result<null, ClientRevokeUserError>>
    clientSetupShamirRecovery(
        client_handle: Handle,
        per_recipient_shares: Map<UserID, NonZeroU8>,
        threshold: NonZeroU8
    ): Promise<Result<null, ClientSetupShamirRecoveryError>>
    clientShareWorkspace(
        client: Handle,
        realm_id: VlobID,
        recipient: UserID,
        role: RealmRole | null
    ): Promise<Result<null, ClientShareWorkspaceError>>
    clientStart(
        config: ClientConfig,
        access: DeviceAccessStrategy
    ): Promise<Result<Handle, ClientStartError>>
    clientStartDeviceInvitationGreet(
        client: Handle,
        token: AccessToken
    ): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>>
    clientStartShamirRecoveryInvitationGreet(
        client: Handle,
        token: AccessToken
    ): Promise<Result<ShamirRecoveryGreetInitialInfo, ClientStartShamirRecoveryInvitationGreetError>>
    clientStartUserInvitationGreet(
        client: Handle,
        token: AccessToken
    ): Promise<Result<UserGreetInitialInfo, ClientStartInvitationGreetError>>
    clientStartWorkspace(
        client: Handle,
        realm_id: VlobID
    ): Promise<Result<Handle, ClientStartWorkspaceError>>
    clientStartWorkspaceHistory(
        client: Handle,
        realm_id: VlobID
    ): Promise<Result<Handle, WorkspaceHistoryStartError>>
    clientStop(
        client: Handle
    ): Promise<Result<null, ClientStopError>>
    clientTotpCreateOpaqueKey(
        client: Handle
    ): Promise<Result<[TOTPOpaqueKeyID, SecretKey], ClientTotpCreateOpaqueKeyError>>
    clientTotpSetupConfirm(
        client: Handle,
        one_time_password: string
    ): Promise<Result<null, ClientTOTPSetupConfirmError>>
    clientTotpSetupStatus(
        client: Handle
    ): Promise<Result<TOTPSetupStatus, ClientTotpSetupStatusError>>
    clientUpdateUserProfile(
        client_handle: Handle,
        user: UserID,
        new_profile: UserProfile
    ): Promise<Result<null, ClientUserUpdateProfileError>>
    getDefaultConfigDir(
    ): Promise<Path>
    getDefaultDataBaseDir(
    ): Promise<Path>
    getDefaultMountpointBaseDir(
    ): Promise<Path>
    getPlatform(
    ): Promise<Platform>
    getServerConfig(
        config_dir: Path,
        addr: ParsecAddr
    ): Promise<Result<ServerConfig, GetServerConfigError>>
    greeterDeviceInProgress1DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceGreetInProgress2Info, GreetInProgressError>>
    greeterDeviceInProgress2DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, GreetInProgressError>>
    greeterDeviceInProgress2DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceGreetInProgress3Info, GreetInProgressError>>
    greeterDeviceInProgress3DoGetClaimRequests(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceGreetInProgress4Info, GreetInProgressError>>
    greeterDeviceInProgress4DoCreate(
        canceller: Handle,
        handle: Handle,
        device_label: DeviceLabel
    ): Promise<Result<null, GreetInProgressError>>
    greeterDeviceInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<DeviceGreetInProgress1Info, GreetInProgressError>>
    greeterShamirRecoveryInProgress1DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryGreetInProgress2Info, GreetInProgressError>>
    greeterShamirRecoveryInProgress2DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, GreetInProgressError>>
    greeterShamirRecoveryInProgress2DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryGreetInProgress3Info, GreetInProgressError>>
    greeterShamirRecoveryInProgress3DoGetClaimRequests(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, GreetInProgressError>>
    greeterShamirRecoveryInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<ShamirRecoveryGreetInProgress1Info, GreetInProgressError>>
    greeterUserInProgress1DoWaitPeerTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>
    greeterUserInProgress2DoDenyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<null, GreetInProgressError>>
    greeterUserInProgress2DoSignifyTrust(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>>
    greeterUserInProgress3DoGetClaimRequests(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>>
    greeterUserInProgress4DoCreate(
        canceller: Handle,
        handle: Handle,
        human_handle: HumanHandle,
        device_label: DeviceLabel,
        profile: UserProfile
    ): Promise<Result<null, GreetInProgressError>>
    greeterUserInitialDoWaitPeer(
        canceller: Handle,
        handle: Handle
    ): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>>
    importRecoveryDevice(
        config: ClientConfig,
        recovery_device: Uint8Array,
        passphrase: string,
        device_label: DeviceLabel,
        save_strategy: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, ImportRecoveryDeviceError>>
    isKeyringAvailable(
    ): Promise<boolean>
    isPkiAvailable(
    ): Promise<boolean>
    libparsecInitNativeOnlyInit(
        config: ClientConfig
    ): Promise<null>
    libparsecInitSetOnEventCallback(
        on_event_callback: (handle: number, event: ClientEvent) => void
    ): Promise<null>
    listAvailableDevices(
        path: Path
    ): Promise<Result<Array<AvailableDevice>, ListAvailableDeviceError>>
    listStartedAccounts(
    ): Promise<Array<Handle>>
    listStartedClients(
    ): Promise<Array<[Handle, DeviceID]>>
    mountpointToOsPath(
        mountpoint: Handle,
        parsec_path: FsPath
    ): Promise<Result<Path, MountpointToOsPathError>>
    mountpointUnmount(
        mountpoint: Handle
    ): Promise<Result<null, MountpointUnmountError>>
    newCanceller(
    ): Promise<Handle>
    openbaoListSelfEmails(
        openbao_server_url: string,
        openbao_secret_mount_path: string,
        openbao_transit_mount_path: string,
        openbao_entity_id: string,
        openbao_auth_token: string
    ): Promise<Result<Array<string>, OpenBaoListSelfEmailsError>>
    parseParsecAddr(
        url: string
    ): Promise<Result<ParsedParsecAddr, ParseParsecAddrError>>
    pathFilename(
        path: FsPath
    ): Promise<EntryName | null>
    pathJoin(
        parent: FsPath,
        child: EntryName
    ): Promise<FsPath>
    pathNormalize(
        path: FsPath
    ): Promise<FsPath>
    pathParent(
        path: FsPath
    ): Promise<FsPath>
    pathSplit(
        path: FsPath
    ): Promise<Array<EntryName>>
    removeDeviceData(
        config: ClientConfig,
        device_id: DeviceID
    ): Promise<Result<null, RemoveDeviceDataError>>
    showCertificateSelectionDialogWindowsOnly(
    ): Promise<Result<X509CertificateReference | null, ShowCertificateSelectionDialogError>>
    submitAsyncEnrollment(
        config: ClientConfig,
        addr: ParsecAsyncEnrollmentAddr,
        force: boolean,
        requested_device_label: DeviceLabel,
        identity_strategy: SubmitAsyncEnrollmentIdentityStrategy
    ): Promise<Result<AvailablePendingAsyncEnrollment, SubmitAsyncEnrollmentError>>
    submitterCancelAsyncEnrollment(
        config: ClientConfig,
        addr: ParsecAsyncEnrollmentAddr,
        enrollment_id: AsyncEnrollmentID
    ): Promise<Result<null, SubmitterCancelAsyncEnrollmentError>>
    submitterFinalizeAsyncEnrollment(
        config: ClientConfig,
        enrollment_file: Path,
        new_device_save_strategy: DeviceSaveStrategy,
        identity_strategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy
    ): Promise<Result<AvailableDevice, SubmitterFinalizeAsyncEnrollmentError>>
    submitterGetAsyncEnrollmentInfo(
        config: ClientConfig,
        addr: ParsecAsyncEnrollmentAddr,
        enrollment_id: AsyncEnrollmentID
    ): Promise<Result<PendingAsyncEnrollmentInfo, SubmitterGetAsyncEnrollmentInfoError>>
    submitterListAsyncEnrollments(
        config_dir: Path
    ): Promise<Result<Array<AvailablePendingAsyncEnrollment>, SubmitterListLocalAsyncEnrollmentsError>>
    testCheckMailbox(
        server_addr: ParsecAddr,
        email: EmailAddress
    ): Promise<Result<Array<[EmailAddress, DateTime, string]>, TestbedError>>
    testDropTestbed(
        path: Path
    ): Promise<Result<null, TestbedError>>
    testGetTestbedBootstrapOrganizationAddr(
        discriminant_dir: Path
    ): Promise<Result<ParsecOrganizationBootstrapAddr | null, TestbedError>>
    testGetTestbedOrganizationId(
        discriminant_dir: Path
    ): Promise<Result<OrganizationID | null, TestbedError>>
    testNewAccount(
        server_addr: ParsecAddr
    ): Promise<Result<[HumanHandle, KeyDerivation], TestbedError>>
    testNewTestbed(
        template: string,
        test_server: ParsecAddr | null
    ): Promise<Result<Path, TestbedError>>
    totpFetchOpaqueKey(
        config: ClientConfig,
        server_addr: ParsecAddr,
        organization_id: OrganizationID,
        user_id: UserID,
        opaque_key_id: TOTPOpaqueKeyID,
        one_time_password: string
    ): Promise<Result<SecretKey, TotpFetchOpaqueKeyError>>
    totpSetupConfirmAnonymous(
        config: ClientConfig,
        addr: ParsecTOTPResetAddr,
        one_time_password: string
    ): Promise<Result<null, TotpSetupConfirmAnonymousError>>
    totpSetupStatusAnonymous(
        config: ClientConfig,
        addr: ParsecTOTPResetAddr
    ): Promise<Result<TOTPSetupStatus, TotpSetupStatusAnonymousError>>
    updateDeviceChangeAuthentication(
        config_dir: Path,
        current_auth: DeviceAccessStrategy,
        new_auth: DeviceSaveStrategy
    ): Promise<Result<AvailableDevice, UpdateDeviceError>>
    updateDeviceOverwriteServerAddr(
        config_dir: Path,
        access: DeviceAccessStrategy,
        new_server_addr: ParsecAddr
    ): Promise<Result<ParsecAddr, UpdateDeviceError>>
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
    validateOrganizationId(
        raw: string
    ): Promise<boolean>
    validatePath(
        raw: string
    ): Promise<boolean>
    waitForDeviceAvailable(
        config_dir: Path,
        device_id: DeviceID
    ): Promise<Result<null, WaitForDeviceAvailableError>>
    workspaceCreateFile(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceCreateFileError>>
    workspaceCreateFolder(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceCreateFolderError>>
    workspaceCreateFolderAll(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceCreateFolderError>>
    workspaceDecryptPathAddr(
        workspace: Handle,
        link: ParsecWorkspacePathAddr
    ): Promise<Result<FsPath, WorkspaceDecryptPathAddrError>>
    workspaceFdClose(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<null, WorkspaceFdCloseError>>
    workspaceFdFlush(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<null, WorkspaceFdFlushError>>
    workspaceFdRead(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        size: U64
    ): Promise<Result<Uint8Array, WorkspaceFdReadError>>
    workspaceFdResize(
        workspace: Handle,
        fd: FileDescriptor,
        length: U64,
        truncate_only: boolean
    ): Promise<Result<null, WorkspaceFdResizeError>>
    workspaceFdStat(
        workspace: Handle,
        fd: FileDescriptor
    ): Promise<Result<FileStat, WorkspaceFdStatError>>
    workspaceFdWrite(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    workspaceFdWriteConstrainedIo(
        workspace: Handle,
        fd: FileDescriptor,
        offset: U64,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    workspaceFdWriteStartEof(
        workspace: Handle,
        fd: FileDescriptor,
        data: Uint8Array
    ): Promise<Result<U64, WorkspaceFdWriteError>>
    workspaceGeneratePathAddr(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<ParsecWorkspacePathAddr, WorkspaceGeneratePathAddrError>>
    workspaceHistoryFdClose(
        workspace_history: Handle,
        fd: FileDescriptor
    ): Promise<Result<null, WorkspaceHistoryFdCloseError>>
    workspaceHistoryFdRead(
        workspace_history: Handle,
        fd: FileDescriptor,
        offset: U64,
        size: U64
    ): Promise<Result<Uint8Array, WorkspaceHistoryFdReadError>>
    workspaceHistoryFdStat(
        workspace_history: Handle,
        fd: FileDescriptor
    ): Promise<Result<WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError>>
    workspaceHistoryGetTimestampHigherBound(
        workspace_history: Handle
    ): Promise<Result<DateTime, WorkspaceHistoryInternalOnlyError>>
    workspaceHistoryGetTimestampLowerBound(
        workspace_history: Handle
    ): Promise<Result<DateTime, WorkspaceHistoryInternalOnlyError>>
    workspaceHistoryGetTimestampOfInterest(
        workspace_history: Handle
    ): Promise<Result<DateTime, WorkspaceHistoryInternalOnlyError>>
    workspaceHistoryOpenFile(
        workspace_history: Handle,
        path: FsPath
    ): Promise<Result<FileDescriptor, WorkspaceHistoryOpenFileError>>
    workspaceHistoryOpenFileAndGetId(
        workspace_history: Handle,
        path: FsPath
    ): Promise<Result<[FileDescriptor, VlobID], WorkspaceHistoryOpenFileError>>
    workspaceHistoryOpenFileById(
        workspace_history: Handle,
        entry_id: VlobID
    ): Promise<Result<FileDescriptor, WorkspaceHistoryOpenFileError>>
    workspaceHistorySetTimestampOfInterest(
        workspace_history: Handle,
        toi: DateTime
    ): Promise<Result<null, WorkspaceHistorySetTimestampOfInterestError>>
    workspaceHistoryStartWithRealmExport(
        config: ClientConfig,
        export_db_path: Path,
        decryptors: Array<WorkspaceHistoryRealmExportDecryptor>
    ): Promise<Result<Handle, WorkspaceHistoryStartError>>
    workspaceHistoryStatEntry(
        workspace_history: Handle,
        path: FsPath
    ): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
    workspaceHistoryStatEntryById(
        workspace_history: Handle,
        entry_id: VlobID
    ): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
    workspaceHistoryStatFolderChildren(
        workspace_history: Handle,
        path: FsPath
    ): Promise<Result<Array<[EntryName, WorkspaceHistoryEntryStat]>, WorkspaceHistoryStatFolderChildrenError>>
    workspaceHistoryStatFolderChildrenById(
        workspace_history: Handle,
        entry_id: VlobID
    ): Promise<Result<Array<[EntryName, WorkspaceHistoryEntryStat]>, WorkspaceHistoryStatFolderChildrenError>>
    workspaceHistoryStop(
        workspace_history: Handle
    ): Promise<Result<null, WorkspaceHistoryInternalOnlyError>>
    workspaceInfo(
        workspace: Handle
    ): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError>>
    workspaceIsFileContentLocal(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<boolean, WorkspaceIsFileContentLocalError>>
    workspaceMount(
        workspace: Handle
    ): Promise<Result<[Handle, Path], WorkspaceMountError>>
    workspaceMoveEntry(
        workspace: Handle,
        src: FsPath,
        dst: FsPath,
        mode: MoveEntryMode
    ): Promise<Result<null, WorkspaceMoveEntryError>>
    workspaceOpenFile(
        workspace: Handle,
        path: FsPath,
        mode: OpenOptions
    ): Promise<Result<FileDescriptor, WorkspaceOpenFileError>>
    workspaceOpenFileAndGetId(
        workspace: Handle,
        path: FsPath,
        mode: OpenOptions
    ): Promise<Result<[FileDescriptor, VlobID], WorkspaceOpenFileError>>
    workspaceOpenFileById(
        workspace: Handle,
        entry_id: VlobID,
        mode: OpenOptions
    ): Promise<Result<FileDescriptor, WorkspaceOpenFileError>>
    workspaceRemoveEntry(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceRemoveEntryError>>
    workspaceRemoveFile(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceRemoveEntryError>>
    workspaceRemoveFolder(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceRemoveEntryError>>
    workspaceRemoveFolderAll(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<null, WorkspaceRemoveEntryError>>
    workspaceRenameEntryById(
        workspace: Handle,
        src_parent_id: VlobID,
        src_name: EntryName,
        dst_name: EntryName,
        mode: MoveEntryMode
    ): Promise<Result<null, WorkspaceMoveEntryError>>
    workspaceStatEntry(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<EntryStat, WorkspaceStatEntryError>>
    workspaceStatEntryById(
        workspace: Handle,
        entry_id: VlobID
    ): Promise<Result<EntryStat, WorkspaceStatEntryError>>
    workspaceStatEntryByIdIgnoreConfinementPoint(
        workspace: Handle,
        entry_id: VlobID
    ): Promise<Result<EntryStat, WorkspaceStatEntryError>>
    workspaceStatFolderChildren(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<Array<[EntryName, EntryStat]>, WorkspaceStatFolderChildrenError>>
    workspaceStatFolderChildrenById(
        workspace: Handle,
        entry_id: VlobID
    ): Promise<Result<Array<[EntryName, EntryStat]>, WorkspaceStatFolderChildrenError>>
    workspaceStop(
        workspace: Handle
    ): Promise<Result<null, WorkspaceStopError>>
    workspaceWatchEntryOneshot(
        workspace: Handle,
        path: FsPath
    ): Promise<Result<VlobID, WorkspaceWatchEntryOneShotError>>
}
