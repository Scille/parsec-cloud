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


export interface AccountInfo {
    serverAddr: string
    inUseAuthMethod: string
    humanHandle: HumanHandle
}


export interface AccountOrganizations {
    active: Array<AccountOrganizationsActiveUser>
    revoked: Array<AccountOrganizationsRevokedUser>
}


export interface AccountOrganizationsActiveUser {
    organizationId: string
    userId: string
    createdOn: number
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
    organizationId: string
    userId: string
    createdOn: number
    revokedOn: number
    currentProfile: UserProfile
}


export interface AsyncEnrollmentUntrusted {
    enrollmentId: string
    submittedOn: number
    untrustedRequestedDeviceLabel: string
    untrustedRequestedHumanHandle: HumanHandle
    identitySystem: AsyncEnrollmentIdentitySystem
}


export interface AuthMethodInfo {
    authMethodId: string
    createdOn: number
    createdByIp: string
    createdByUserAgent: string
    usePassword: boolean
}


export interface AvailableDevice {
    keyFilePath: string
    createdOn: number
    protectedOn: number
    serverAddr: string
    organizationId: string
    userId: string
    deviceId: string
    humanHandle: HumanHandle
    deviceLabel: string
    ty: AvailableDeviceType
}


export interface AvailablePendingAsyncEnrollment {
    filePath: string
    submittedOn: number
    addr: string
    enrollmentId: string
    requestedDeviceLabel: string
    requestedHumanHandle: HumanHandle
    identitySystem: AvailablePendingAsyncEnrollmentIdentitySystem
}


export interface ClientConfig {
    configDir: string
    dataBaseDir: string
    mountpointMountStrategy: MountpointMountStrategy
    workspaceStorageCacheSize: WorkspaceStorageCacheSize
    withMonitors: boolean
    preventSyncPattern: string | null
    logLevel: LogLevel | null
}


export interface ClientInfo {
    organizationAddr: string
    organizationId: string
    deviceId: string
    userId: string
    deviceLabel: string
    humanHandle: HumanHandle
    currentProfile: UserProfile
    serverOrganizationConfig: ServerOrganizationConfig
    isServerOnline: boolean
    isOrganizationExpired: boolean
    mustAcceptTos: boolean
}


export interface DeviceClaimFinalizeInfo {
    handle: number
}


export interface DeviceClaimInProgress1Info {
    handle: number
    greeterUserId: string
    greeterHumanHandle: HumanHandle
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface DeviceClaimInProgress2Info {
    handle: number
    claimerSas: string
}


export interface DeviceClaimInProgress3Info {
    handle: number
}


export interface DeviceGreetInProgress1Info {
    handle: number
    greeterSas: string
}


export interface DeviceGreetInProgress2Info {
    handle: number
    claimerSas: string
    claimerSasChoices: Array<string>
}


export interface DeviceGreetInProgress3Info {
    handle: number
}


export interface DeviceGreetInProgress4Info {
    handle: number
    requestedDeviceLabel: string
}


export interface DeviceGreetInitialInfo {
    handle: number
}


export interface DeviceInfo {
    id: string
    purpose: DevicePurpose
    deviceLabel: string
    createdOn: number
    createdBy: string | null
}


export interface FileStat {
    id: string
    created: number
    updated: number
    baseVersion: number
    isPlaceholder: boolean
    needSync: boolean
    size: number
}


export interface HumanHandle {
    email: string
    label: string
}


export interface NewInvitationInfo {
    addr: string
    token: string
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
    totalBlockBytes: number
    totalMetadataBytes: number
}


export interface PKILocalPendingEnrollment {
    certRef: X509CertificateReference
    addr: string
    submittedOn: number
    enrollmentId: string
    payload: PkiEnrollmentSubmitPayload
    encryptedKey: Uint8Array
    encryptedKeyAlgo: string
    ciphertext: Uint8Array
}


export interface PkiEnrollmentAnswerPayload {
    userId: string
    deviceId: string
    deviceLabel: string
    profile: UserProfile
    rootVerifyKey: Uint8Array
}


export interface PkiEnrollmentSubmitPayload {
    verifyKey: Uint8Array
    publicKey: Uint8Array
    deviceLabel: string
}


export interface RawPkiEnrollmentListItem {
    enrollmentId: string
    submittedOn: number
    derX509Certificate: Uint8Array
    intermediateDerX509Certificates: Array<Uint8Array>
    payloadSignature: Uint8Array
    payloadSignatureAlgorithm: string
    payload: Uint8Array
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
    handle: number
    greeterUserId: string
    greeterHumanHandle: HumanHandle
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface ShamirRecoveryClaimInProgress2Info {
    handle: number
    claimerSas: string
}


export interface ShamirRecoveryClaimInProgress3Info {
    handle: number
}


export interface ShamirRecoveryClaimInitialInfo {
    handle: number
    greeterUserId: string
    greeterHumanHandle: HumanHandle
}


export interface ShamirRecoveryClaimShareInfo {
    handle: number
}


export interface ShamirRecoveryGreetInProgress1Info {
    handle: number
    greeterSas: string
}


export interface ShamirRecoveryGreetInProgress2Info {
    handle: number
    claimerSas: string
    claimerSasChoices: Array<string>
}


export interface ShamirRecoveryGreetInProgress3Info {
    handle: number
}


export interface ShamirRecoveryGreetInitialInfo {
    handle: number
}


export interface ShamirRecoveryRecipient {
    userId: string
    humanHandle: HumanHandle
    revokedOn: number | null
    shares: number
    onlineStatus: UserOnlineStatus
}


export interface StartedWorkspaceInfo {
    client: number
    id: string
    currentName: string
    currentSelfRole: RealmRole
    mountpoints: Array<[number, string]>
}


export interface Tos {
    perLocaleUrls: Map<string, string>
    updatedOn: number
}


export interface UserClaimFinalizeInfo {
    handle: number
}


export interface UserClaimInProgress1Info {
    handle: number
    greeterUserId: string
    greeterHumanHandle: HumanHandle
    greeterSas: string
    greeterSasChoices: Array<string>
}


export interface UserClaimInProgress2Info {
    handle: number
    claimerSas: string
}


export interface UserClaimInProgress3Info {
    handle: number
}


export interface UserClaimInitialInfo {
    handle: number
    greeterUserId: string
    greeterHumanHandle: HumanHandle
    onlineStatus: UserOnlineStatus
    lastGreetingAttemptJoinedOn: number | null
}


export interface UserGreetInProgress1Info {
    handle: number
    greeterSas: string
}


export interface UserGreetInProgress2Info {
    handle: number
    claimerSas: string
    claimerSasChoices: Array<string>
}


export interface UserGreetInProgress3Info {
    handle: number
}


export interface UserGreetInProgress4Info {
    handle: number
    requestedHumanHandle: HumanHandle
    requestedDeviceLabel: string
}


export interface UserGreetInitialInfo {
    handle: number
}


export interface UserGreetingAdministrator {
    userId: string
    humanHandle: HumanHandle
    onlineStatus: UserOnlineStatus
    lastGreetingAttemptJoinedOn: number | null
}


export interface UserInfo {
    id: string
    humanHandle: HumanHandle
    currentProfile: UserProfile
    createdOn: number
    createdBy: string | null
    revokedOn: number | null
    revokedBy: string | null
}


export interface WorkspaceHistoryFileStat {
    id: string
    created: number
    updated: number
    version: number
    size: number
}


export interface WorkspaceInfo {
    id: string
    currentName: string
    currentSelfRole: RealmRole
    isStarted: boolean
    isBootstrapped: boolean
}


export interface WorkspaceUserAccessInfo {
    userId: string
    humanHandle: HumanHandle
    currentProfile: UserProfile
    currentRole: RealmRole
}


export interface X509CertificateReference {
    uris: Array<X509URIFlavorValue>
    hash: string
}


export interface X509Pkcs11URI {
}


export interface X509WindowsCngURI {
    issuer: Uint8Array
    serialNumber: Uint8Array
}


// AcceptFinalizeAsyncEnrollmentIdentityStrategy
export interface AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao {
    tag: "AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao"
    openbao_server_url: string
    openbao_transit_mount_path: string
    openbao_secret_mount_path: string
    openbao_entity_id: string
    openbao_auth_token: string
}
export interface AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI {
    tag: "AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI"
    certificate_reference: X509CertificateReference
}
export type AcceptFinalizeAsyncEnrollmentIdentityStrategy =
  | AcceptFinalizeAsyncEnrollmentIdentityStrategyOpenBao
  | AcceptFinalizeAsyncEnrollmentIdentityStrategyPKI


// AccountAuthMethodStrategy
export interface AccountAuthMethodStrategyMasterSecret {
    tag: "AccountAuthMethodStrategyMasterSecret"
    master_secret: Uint8Array
}
export interface AccountAuthMethodStrategyPassword {
    tag: "AccountAuthMethodStrategyPassword"
    password: string
}
export type AccountAuthMethodStrategy =
  | AccountAuthMethodStrategyMasterSecret
  | AccountAuthMethodStrategyPassword


// AccountConfig
export interface AccountConfigDisabled {
    tag: "AccountConfigDisabled"
}
export interface AccountConfigEnabledWithVault {
    tag: "AccountConfigEnabledWithVault"
}
export interface AccountConfigEnabledWithoutVault {
    tag: "AccountConfigEnabledWithoutVault"
}
export type AccountConfig =
  | AccountConfigDisabled
  | AccountConfigEnabledWithVault
  | AccountConfigEnabledWithoutVault


// AccountCreateAuthMethodError
export interface AccountCreateAuthMethodErrorBadVaultKeyAccess {
    tag: "AccountCreateAuthMethodErrorBadVaultKeyAccess"
    error: string
}
export interface AccountCreateAuthMethodErrorInternal {
    tag: "AccountCreateAuthMethodErrorInternal"
    error: string
}
export interface AccountCreateAuthMethodErrorOffline {
    tag: "AccountCreateAuthMethodErrorOffline"
    error: string
}
export type AccountCreateAuthMethodError =
  | AccountCreateAuthMethodErrorBadVaultKeyAccess
  | AccountCreateAuthMethodErrorInternal
  | AccountCreateAuthMethodErrorOffline


// AccountCreateError
export interface AccountCreateErrorInternal {
    tag: "AccountCreateErrorInternal"
    error: string
}
export interface AccountCreateErrorInvalidValidationCode {
    tag: "AccountCreateErrorInvalidValidationCode"
    error: string
}
export interface AccountCreateErrorOffline {
    tag: "AccountCreateErrorOffline"
    error: string
}
export interface AccountCreateErrorSendValidationEmailRequired {
    tag: "AccountCreateErrorSendValidationEmailRequired"
    error: string
}
export type AccountCreateError =
  | AccountCreateErrorInternal
  | AccountCreateErrorInvalidValidationCode
  | AccountCreateErrorOffline
  | AccountCreateErrorSendValidationEmailRequired


// AccountCreateRegistrationDeviceError
export interface AccountCreateRegistrationDeviceErrorBadVaultKeyAccess {
    tag: "AccountCreateRegistrationDeviceErrorBadVaultKeyAccess"
    error: string
}
export interface AccountCreateRegistrationDeviceErrorInternal {
    tag: "AccountCreateRegistrationDeviceErrorInternal"
    error: string
}
export interface AccountCreateRegistrationDeviceErrorLoadDeviceDecryptionFailed {
    tag: "AccountCreateRegistrationDeviceErrorLoadDeviceDecryptionFailed"
    error: string
}
export interface AccountCreateRegistrationDeviceErrorLoadDeviceInvalidData {
    tag: "AccountCreateRegistrationDeviceErrorLoadDeviceInvalidData"
    error: string
}
export interface AccountCreateRegistrationDeviceErrorLoadDeviceInvalidPath {
    tag: "AccountCreateRegistrationDeviceErrorLoadDeviceInvalidPath"
    error: string
}
export interface AccountCreateRegistrationDeviceErrorOffline {
    tag: "AccountCreateRegistrationDeviceErrorOffline"
    error: string
}
export interface AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchFailed {
    tag: "AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchFailed"
    error: string
}
export interface AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchOffline {
    tag: "AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchOffline"
    error: string
}
export interface AccountCreateRegistrationDeviceErrorTimestampOutOfBallpark {
    tag: "AccountCreateRegistrationDeviceErrorTimestampOutOfBallpark"
    error: string
}
export type AccountCreateRegistrationDeviceError =
  | AccountCreateRegistrationDeviceErrorBadVaultKeyAccess
  | AccountCreateRegistrationDeviceErrorInternal
  | AccountCreateRegistrationDeviceErrorLoadDeviceDecryptionFailed
  | AccountCreateRegistrationDeviceErrorLoadDeviceInvalidData
  | AccountCreateRegistrationDeviceErrorLoadDeviceInvalidPath
  | AccountCreateRegistrationDeviceErrorOffline
  | AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchFailed
  | AccountCreateRegistrationDeviceErrorRemoteOpaqueKeyFetchOffline
  | AccountCreateRegistrationDeviceErrorTimestampOutOfBallpark


// AccountCreateSendValidationEmailError
export interface AccountCreateSendValidationEmailErrorEmailRecipientRefused {
    tag: "AccountCreateSendValidationEmailErrorEmailRecipientRefused"
    error: string
}
export interface AccountCreateSendValidationEmailErrorEmailSendingRateLimited {
    tag: "AccountCreateSendValidationEmailErrorEmailSendingRateLimited"
    error: string
    wait_until: number
}
export interface AccountCreateSendValidationEmailErrorEmailServerUnavailable {
    tag: "AccountCreateSendValidationEmailErrorEmailServerUnavailable"
    error: string
}
export interface AccountCreateSendValidationEmailErrorInternal {
    tag: "AccountCreateSendValidationEmailErrorInternal"
    error: string
}
export interface AccountCreateSendValidationEmailErrorOffline {
    tag: "AccountCreateSendValidationEmailErrorOffline"
    error: string
}
export type AccountCreateSendValidationEmailError =
  | AccountCreateSendValidationEmailErrorEmailRecipientRefused
  | AccountCreateSendValidationEmailErrorEmailSendingRateLimited
  | AccountCreateSendValidationEmailErrorEmailServerUnavailable
  | AccountCreateSendValidationEmailErrorInternal
  | AccountCreateSendValidationEmailErrorOffline


// AccountDeleteProceedError
export interface AccountDeleteProceedErrorInternal {
    tag: "AccountDeleteProceedErrorInternal"
    error: string
}
export interface AccountDeleteProceedErrorInvalidValidationCode {
    tag: "AccountDeleteProceedErrorInvalidValidationCode"
    error: string
}
export interface AccountDeleteProceedErrorOffline {
    tag: "AccountDeleteProceedErrorOffline"
    error: string
}
export interface AccountDeleteProceedErrorSendValidationEmailRequired {
    tag: "AccountDeleteProceedErrorSendValidationEmailRequired"
    error: string
}
export type AccountDeleteProceedError =
  | AccountDeleteProceedErrorInternal
  | AccountDeleteProceedErrorInvalidValidationCode
  | AccountDeleteProceedErrorOffline
  | AccountDeleteProceedErrorSendValidationEmailRequired


// AccountDeleteSendValidationEmailError
export interface AccountDeleteSendValidationEmailErrorEmailRecipientRefused {
    tag: "AccountDeleteSendValidationEmailErrorEmailRecipientRefused"
    error: string
}
export interface AccountDeleteSendValidationEmailErrorEmailSendingRateLimited {
    tag: "AccountDeleteSendValidationEmailErrorEmailSendingRateLimited"
    error: string
    wait_until: number
}
export interface AccountDeleteSendValidationEmailErrorEmailServerUnavailable {
    tag: "AccountDeleteSendValidationEmailErrorEmailServerUnavailable"
    error: string
}
export interface AccountDeleteSendValidationEmailErrorInternal {
    tag: "AccountDeleteSendValidationEmailErrorInternal"
    error: string
}
export interface AccountDeleteSendValidationEmailErrorOffline {
    tag: "AccountDeleteSendValidationEmailErrorOffline"
    error: string
}
export type AccountDeleteSendValidationEmailError =
  | AccountDeleteSendValidationEmailErrorEmailRecipientRefused
  | AccountDeleteSendValidationEmailErrorEmailSendingRateLimited
  | AccountDeleteSendValidationEmailErrorEmailServerUnavailable
  | AccountDeleteSendValidationEmailErrorInternal
  | AccountDeleteSendValidationEmailErrorOffline


// AccountDisableAuthMethodError
export interface AccountDisableAuthMethodErrorAuthMethodAlreadyDisabled {
    tag: "AccountDisableAuthMethodErrorAuthMethodAlreadyDisabled"
    error: string
}
export interface AccountDisableAuthMethodErrorAuthMethodNotFound {
    tag: "AccountDisableAuthMethodErrorAuthMethodNotFound"
    error: string
}
export interface AccountDisableAuthMethodErrorInternal {
    tag: "AccountDisableAuthMethodErrorInternal"
    error: string
}
export interface AccountDisableAuthMethodErrorOffline {
    tag: "AccountDisableAuthMethodErrorOffline"
    error: string
}
export interface AccountDisableAuthMethodErrorSelfDisableNotAllowed {
    tag: "AccountDisableAuthMethodErrorSelfDisableNotAllowed"
    error: string
}
export type AccountDisableAuthMethodError =
  | AccountDisableAuthMethodErrorAuthMethodAlreadyDisabled
  | AccountDisableAuthMethodErrorAuthMethodNotFound
  | AccountDisableAuthMethodErrorInternal
  | AccountDisableAuthMethodErrorOffline
  | AccountDisableAuthMethodErrorSelfDisableNotAllowed


// AccountInfoError
export interface AccountInfoErrorInternal {
    tag: "AccountInfoErrorInternal"
    error: string
}
export type AccountInfoError =
  | AccountInfoErrorInternal


// AccountListAuthMethodsError
export interface AccountListAuthMethodsErrorInternal {
    tag: "AccountListAuthMethodsErrorInternal"
    error: string
}
export interface AccountListAuthMethodsErrorOffline {
    tag: "AccountListAuthMethodsErrorOffline"
    error: string
}
export type AccountListAuthMethodsError =
  | AccountListAuthMethodsErrorInternal
  | AccountListAuthMethodsErrorOffline


// AccountListInvitationsError
export interface AccountListInvitationsErrorInternal {
    tag: "AccountListInvitationsErrorInternal"
    error: string
}
export interface AccountListInvitationsErrorOffline {
    tag: "AccountListInvitationsErrorOffline"
    error: string
}
export type AccountListInvitationsError =
  | AccountListInvitationsErrorInternal
  | AccountListInvitationsErrorOffline


// AccountListOrganizationsError
export interface AccountListOrganizationsErrorInternal {
    tag: "AccountListOrganizationsErrorInternal"
    error: string
}
export interface AccountListOrganizationsErrorOffline {
    tag: "AccountListOrganizationsErrorOffline"
    error: string
}
export type AccountListOrganizationsError =
  | AccountListOrganizationsErrorInternal
  | AccountListOrganizationsErrorOffline


// AccountListRegistrationDevicesError
export interface AccountListRegistrationDevicesErrorBadVaultKeyAccess {
    tag: "AccountListRegistrationDevicesErrorBadVaultKeyAccess"
    error: string
}
export interface AccountListRegistrationDevicesErrorInternal {
    tag: "AccountListRegistrationDevicesErrorInternal"
    error: string
}
export interface AccountListRegistrationDevicesErrorOffline {
    tag: "AccountListRegistrationDevicesErrorOffline"
    error: string
}
export type AccountListRegistrationDevicesError =
  | AccountListRegistrationDevicesErrorBadVaultKeyAccess
  | AccountListRegistrationDevicesErrorInternal
  | AccountListRegistrationDevicesErrorOffline


// AccountLoginError
export interface AccountLoginErrorBadPasswordAlgorithm {
    tag: "AccountLoginErrorBadPasswordAlgorithm"
    error: string
}
export interface AccountLoginErrorInternal {
    tag: "AccountLoginErrorInternal"
    error: string
}
export interface AccountLoginErrorOffline {
    tag: "AccountLoginErrorOffline"
    error: string
}
export type AccountLoginError =
  | AccountLoginErrorBadPasswordAlgorithm
  | AccountLoginErrorInternal
  | AccountLoginErrorOffline


// AccountLoginStrategy
export interface AccountLoginStrategyMasterSecret {
    tag: "AccountLoginStrategyMasterSecret"
    master_secret: Uint8Array
}
export interface AccountLoginStrategyPassword {
    tag: "AccountLoginStrategyPassword"
    email: string
    password: string
}
export type AccountLoginStrategy =
  | AccountLoginStrategyMasterSecret
  | AccountLoginStrategyPassword


// AccountLogoutError
export interface AccountLogoutErrorInternal {
    tag: "AccountLogoutErrorInternal"
    error: string
}
export type AccountLogoutError =
  | AccountLogoutErrorInternal


// AccountRecoverProceedError
export interface AccountRecoverProceedErrorInternal {
    tag: "AccountRecoverProceedErrorInternal"
    error: string
}
export interface AccountRecoverProceedErrorInvalidValidationCode {
    tag: "AccountRecoverProceedErrorInvalidValidationCode"
    error: string
}
export interface AccountRecoverProceedErrorOffline {
    tag: "AccountRecoverProceedErrorOffline"
    error: string
}
export interface AccountRecoverProceedErrorSendValidationEmailRequired {
    tag: "AccountRecoverProceedErrorSendValidationEmailRequired"
    error: string
}
export type AccountRecoverProceedError =
  | AccountRecoverProceedErrorInternal
  | AccountRecoverProceedErrorInvalidValidationCode
  | AccountRecoverProceedErrorOffline
  | AccountRecoverProceedErrorSendValidationEmailRequired


// AccountRecoverSendValidationEmailError
export interface AccountRecoverSendValidationEmailErrorEmailRecipientRefused {
    tag: "AccountRecoverSendValidationEmailErrorEmailRecipientRefused"
    error: string
}
export interface AccountRecoverSendValidationEmailErrorEmailSendingRateLimited {
    tag: "AccountRecoverSendValidationEmailErrorEmailSendingRateLimited"
    error: string
    wait_until: number
}
export interface AccountRecoverSendValidationEmailErrorEmailServerUnavailable {
    tag: "AccountRecoverSendValidationEmailErrorEmailServerUnavailable"
    error: string
}
export interface AccountRecoverSendValidationEmailErrorInternal {
    tag: "AccountRecoverSendValidationEmailErrorInternal"
    error: string
}
export interface AccountRecoverSendValidationEmailErrorOffline {
    tag: "AccountRecoverSendValidationEmailErrorOffline"
    error: string
}
export type AccountRecoverSendValidationEmailError =
  | AccountRecoverSendValidationEmailErrorEmailRecipientRefused
  | AccountRecoverSendValidationEmailErrorEmailSendingRateLimited
  | AccountRecoverSendValidationEmailErrorEmailServerUnavailable
  | AccountRecoverSendValidationEmailErrorInternal
  | AccountRecoverSendValidationEmailErrorOffline


// AccountRegisterNewDeviceError
export interface AccountRegisterNewDeviceErrorBadVaultKeyAccess {
    tag: "AccountRegisterNewDeviceErrorBadVaultKeyAccess"
    error: string
}
export interface AccountRegisterNewDeviceErrorCorruptedRegistrationDevice {
    tag: "AccountRegisterNewDeviceErrorCorruptedRegistrationDevice"
    error: string
}
export interface AccountRegisterNewDeviceErrorInternal {
    tag: "AccountRegisterNewDeviceErrorInternal"
    error: string
}
export interface AccountRegisterNewDeviceErrorInvalidPath {
    tag: "AccountRegisterNewDeviceErrorInvalidPath"
    error: string
}
export interface AccountRegisterNewDeviceErrorOffline {
    tag: "AccountRegisterNewDeviceErrorOffline"
    error: string
}
export interface AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadFailed {
    tag: "AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadFailed"
    error: string
}
export interface AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadOffline {
    tag: "AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadOffline"
    error: string
}
export interface AccountRegisterNewDeviceErrorStorageNotAvailable {
    tag: "AccountRegisterNewDeviceErrorStorageNotAvailable"
    error: string
}
export interface AccountRegisterNewDeviceErrorTimestampOutOfBallpark {
    tag: "AccountRegisterNewDeviceErrorTimestampOutOfBallpark"
    error: string
}
export interface AccountRegisterNewDeviceErrorUnknownRegistrationDevice {
    tag: "AccountRegisterNewDeviceErrorUnknownRegistrationDevice"
    error: string
}
export type AccountRegisterNewDeviceError =
  | AccountRegisterNewDeviceErrorBadVaultKeyAccess
  | AccountRegisterNewDeviceErrorCorruptedRegistrationDevice
  | AccountRegisterNewDeviceErrorInternal
  | AccountRegisterNewDeviceErrorInvalidPath
  | AccountRegisterNewDeviceErrorOffline
  | AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadFailed
  | AccountRegisterNewDeviceErrorRemoteOpaqueKeyUploadOffline
  | AccountRegisterNewDeviceErrorStorageNotAvailable
  | AccountRegisterNewDeviceErrorTimestampOutOfBallpark
  | AccountRegisterNewDeviceErrorUnknownRegistrationDevice


// ActiveUsersLimit
export interface ActiveUsersLimitLimitedTo {
    tag: "ActiveUsersLimitLimitedTo"
    x1: number
}
export interface ActiveUsersLimitNoLimit {
    tag: "ActiveUsersLimitNoLimit"
}
export type ActiveUsersLimit =
  | ActiveUsersLimitLimitedTo
  | ActiveUsersLimitNoLimit


// AnyClaimRetrievedInfo
export interface AnyClaimRetrievedInfoDevice {
    tag: "AnyClaimRetrievedInfoDevice"
    handle: number
    greeter_user_id: string
    greeter_human_handle: HumanHandle
}
export interface AnyClaimRetrievedInfoShamirRecovery {
    tag: "AnyClaimRetrievedInfoShamirRecovery"
    handle: number
    claimer_user_id: string
    claimer_human_handle: HumanHandle
    invitation_created_by: InviteInfoInvitationCreatedBy
    shamir_recovery_created_on: number
    recipients: Array<ShamirRecoveryRecipient>
    threshold: number
    is_recoverable: boolean
}
export interface AnyClaimRetrievedInfoUser {
    tag: "AnyClaimRetrievedInfoUser"
    handle: number
    claimer_email: string
    created_by: InviteInfoInvitationCreatedBy
    administrators: Array<UserGreetingAdministrator>
    preferred_greeter: UserGreetingAdministrator | null
}
export type AnyClaimRetrievedInfo =
  | AnyClaimRetrievedInfoDevice
  | AnyClaimRetrievedInfoShamirRecovery
  | AnyClaimRetrievedInfoUser


// ArchiveDeviceError
export interface ArchiveDeviceErrorInternal {
    tag: "ArchiveDeviceErrorInternal"
    error: string
}
export interface ArchiveDeviceErrorNoSpaceAvailable {
    tag: "ArchiveDeviceErrorNoSpaceAvailable"
    error: string
}
export type ArchiveDeviceError =
  | ArchiveDeviceErrorInternal
  | ArchiveDeviceErrorNoSpaceAvailable


// AsyncEnrollmentIdentitySystem
export interface AsyncEnrollmentIdentitySystemOpenBao {
    tag: "AsyncEnrollmentIdentitySystemOpenBao"
}
export interface AsyncEnrollmentIdentitySystemPKI {
    tag: "AsyncEnrollmentIdentitySystemPKI"
    x509_root_certificate_common_name: string
    x509_root_certificate_subject: Uint8Array
}
export interface AsyncEnrollmentIdentitySystemPKICorrupted {
    tag: "AsyncEnrollmentIdentitySystemPKICorrupted"
    reason: string
}
export type AsyncEnrollmentIdentitySystem =
  | AsyncEnrollmentIdentitySystemOpenBao
  | AsyncEnrollmentIdentitySystemPKI
  | AsyncEnrollmentIdentitySystemPKICorrupted


// AvailableDeviceType
export interface AvailableDeviceTypeAccountVault {
    tag: "AvailableDeviceTypeAccountVault"
}
export interface AvailableDeviceTypeKeyring {
    tag: "AvailableDeviceTypeKeyring"
}
export interface AvailableDeviceTypeOpenBao {
    tag: "AvailableDeviceTypeOpenBao"
    openbao_preferred_auth_id: string
    openbao_entity_id: string
}
export interface AvailableDeviceTypePKI {
    tag: "AvailableDeviceTypePKI"
    certificate_ref: X509CertificateReference
}
export interface AvailableDeviceTypePassword {
    tag: "AvailableDeviceTypePassword"
}
export interface AvailableDeviceTypeRecovery {
    tag: "AvailableDeviceTypeRecovery"
}
export type AvailableDeviceType =
  | AvailableDeviceTypeAccountVault
  | AvailableDeviceTypeKeyring
  | AvailableDeviceTypeOpenBao
  | AvailableDeviceTypePKI
  | AvailableDeviceTypePassword
  | AvailableDeviceTypeRecovery


// AvailablePendingAsyncEnrollmentIdentitySystem
export interface AvailablePendingAsyncEnrollmentIdentitySystemOpenBao {
    tag: "AvailablePendingAsyncEnrollmentIdentitySystemOpenBao"
    openbao_entity_id: string
    openbao_preferred_auth_id: string
}
export interface AvailablePendingAsyncEnrollmentIdentitySystemPKI {
    tag: "AvailablePendingAsyncEnrollmentIdentitySystemPKI"
    certificate_ref: X509CertificateReference
}
export type AvailablePendingAsyncEnrollmentIdentitySystem =
  | AvailablePendingAsyncEnrollmentIdentitySystemOpenBao
  | AvailablePendingAsyncEnrollmentIdentitySystemPKI


// BootstrapOrganizationError
export interface BootstrapOrganizationErrorAlreadyUsedToken {
    tag: "BootstrapOrganizationErrorAlreadyUsedToken"
    error: string
}
export interface BootstrapOrganizationErrorInternal {
    tag: "BootstrapOrganizationErrorInternal"
    error: string
}
export interface BootstrapOrganizationErrorInvalidSequesterAuthorityVerifyKey {
    tag: "BootstrapOrganizationErrorInvalidSequesterAuthorityVerifyKey"
    error: string
}
export interface BootstrapOrganizationErrorInvalidToken {
    tag: "BootstrapOrganizationErrorInvalidToken"
    error: string
}
export interface BootstrapOrganizationErrorOffline {
    tag: "BootstrapOrganizationErrorOffline"
    error: string
}
export interface BootstrapOrganizationErrorOrganizationExpired {
    tag: "BootstrapOrganizationErrorOrganizationExpired"
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceInvalidPath {
    tag: "BootstrapOrganizationErrorSaveDeviceInvalidPath"
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadFailed {
    tag: "BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadFailed"
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadOffline {
    tag: "BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadOffline"
    error: string
}
export interface BootstrapOrganizationErrorSaveDeviceStorageNotAvailable {
    tag: "BootstrapOrganizationErrorSaveDeviceStorageNotAvailable"
    error: string
}
export interface BootstrapOrganizationErrorTimestampOutOfBallpark {
    tag: "BootstrapOrganizationErrorTimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export type BootstrapOrganizationError =
  | BootstrapOrganizationErrorAlreadyUsedToken
  | BootstrapOrganizationErrorInternal
  | BootstrapOrganizationErrorInvalidSequesterAuthorityVerifyKey
  | BootstrapOrganizationErrorInvalidToken
  | BootstrapOrganizationErrorOffline
  | BootstrapOrganizationErrorOrganizationExpired
  | BootstrapOrganizationErrorSaveDeviceInvalidPath
  | BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadFailed
  | BootstrapOrganizationErrorSaveDeviceRemoteOpaqueKeyUploadOffline
  | BootstrapOrganizationErrorSaveDeviceStorageNotAvailable
  | BootstrapOrganizationErrorTimestampOutOfBallpark


// CancelError
export interface CancelErrorInternal {
    tag: "CancelErrorInternal"
    error: string
}
export interface CancelErrorNotBound {
    tag: "CancelErrorNotBound"
    error: string
}
export type CancelError =
  | CancelErrorInternal
  | CancelErrorNotBound


// ClaimFinalizeError
export interface ClaimFinalizeErrorInternal {
    tag: "ClaimFinalizeErrorInternal"
    error: string
}
export interface ClaimFinalizeErrorInvalidPath {
    tag: "ClaimFinalizeErrorInvalidPath"
    error: string
}
export interface ClaimFinalizeErrorRemoteOpaqueKeyUploadFailed {
    tag: "ClaimFinalizeErrorRemoteOpaqueKeyUploadFailed"
    error: string
}
export interface ClaimFinalizeErrorRemoteOpaqueKeyUploadOffline {
    tag: "ClaimFinalizeErrorRemoteOpaqueKeyUploadOffline"
    error: string
}
export interface ClaimFinalizeErrorStorageNotAvailable {
    tag: "ClaimFinalizeErrorStorageNotAvailable"
    error: string
}
export type ClaimFinalizeError =
  | ClaimFinalizeErrorInternal
  | ClaimFinalizeErrorInvalidPath
  | ClaimFinalizeErrorRemoteOpaqueKeyUploadFailed
  | ClaimFinalizeErrorRemoteOpaqueKeyUploadOffline
  | ClaimFinalizeErrorStorageNotAvailable


// ClaimInProgressError
export interface ClaimInProgressErrorActiveUsersLimitReached {
    tag: "ClaimInProgressErrorActiveUsersLimitReached"
    error: string
}
export interface ClaimInProgressErrorAlreadyUsedOrDeleted {
    tag: "ClaimInProgressErrorAlreadyUsedOrDeleted"
    error: string
}
export interface ClaimInProgressErrorCancelled {
    tag: "ClaimInProgressErrorCancelled"
    error: string
}
export interface ClaimInProgressErrorCorruptedConfirmation {
    tag: "ClaimInProgressErrorCorruptedConfirmation"
    error: string
}
export interface ClaimInProgressErrorCorruptedSharedSecretKey {
    tag: "ClaimInProgressErrorCorruptedSharedSecretKey"
    error: string
}
export interface ClaimInProgressErrorGreeterNotAllowed {
    tag: "ClaimInProgressErrorGreeterNotAllowed"
    error: string
}
export interface ClaimInProgressErrorGreetingAttemptCancelled {
    tag: "ClaimInProgressErrorGreetingAttemptCancelled"
    error: string
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: number
}
export interface ClaimInProgressErrorInternal {
    tag: "ClaimInProgressErrorInternal"
    error: string
}
export interface ClaimInProgressErrorNotFound {
    tag: "ClaimInProgressErrorNotFound"
    error: string
}
export interface ClaimInProgressErrorOffline {
    tag: "ClaimInProgressErrorOffline"
    error: string
}
export interface ClaimInProgressErrorOrganizationExpired {
    tag: "ClaimInProgressErrorOrganizationExpired"
    error: string
}
export interface ClaimInProgressErrorPeerReset {
    tag: "ClaimInProgressErrorPeerReset"
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
export interface ClaimerGreeterAbortOperationErrorInternal {
    tag: "ClaimerGreeterAbortOperationErrorInternal"
    error: string
}
export type ClaimerGreeterAbortOperationError =
  | ClaimerGreeterAbortOperationErrorInternal


// ClaimerRetrieveInfoError
export interface ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted {
    tag: "ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted"
    error: string
}
export interface ClaimerRetrieveInfoErrorInternal {
    tag: "ClaimerRetrieveInfoErrorInternal"
    error: string
}
export interface ClaimerRetrieveInfoErrorNotFound {
    tag: "ClaimerRetrieveInfoErrorNotFound"
    error: string
}
export interface ClaimerRetrieveInfoErrorOffline {
    tag: "ClaimerRetrieveInfoErrorOffline"
    error: string
}
export interface ClaimerRetrieveInfoErrorOrganizationExpired {
    tag: "ClaimerRetrieveInfoErrorOrganizationExpired"
    error: string
}
export type ClaimerRetrieveInfoError =
  | ClaimerRetrieveInfoErrorAlreadyUsedOrDeleted
  | ClaimerRetrieveInfoErrorInternal
  | ClaimerRetrieveInfoErrorNotFound
  | ClaimerRetrieveInfoErrorOffline
  | ClaimerRetrieveInfoErrorOrganizationExpired


// ClientAcceptAsyncEnrollmentError
export interface ClientAcceptAsyncEnrollmentErrorActiveUsersLimitReached {
    tag: "ClientAcceptAsyncEnrollmentErrorActiveUsersLimitReached"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorAuthorNotAllowed {
    tag: "ClientAcceptAsyncEnrollmentErrorAuthorNotAllowed"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorBadSubmitPayload {
    tag: "ClientAcceptAsyncEnrollmentErrorBadSubmitPayload"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorEnrollmentNoLongerAvailable {
    tag: "ClientAcceptAsyncEnrollmentErrorEnrollmentNoLongerAvailable"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorEnrollmentNotFound {
    tag: "ClientAcceptAsyncEnrollmentErrorEnrollmentNotFound"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorHumanHandleAlreadyTaken {
    tag: "ClientAcceptAsyncEnrollmentErrorHumanHandleAlreadyTaken"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorIdentityStrategyMismatch {
    tag: "ClientAcceptAsyncEnrollmentErrorIdentityStrategyMismatch"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorInternal {
    tag: "ClientAcceptAsyncEnrollmentErrorInternal"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorOffline {
    tag: "ClientAcceptAsyncEnrollmentErrorOffline"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorOpenBaoBadServerResponse {
    tag: "ClientAcceptAsyncEnrollmentErrorOpenBaoBadServerResponse"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorOpenBaoBadURL {
    tag: "ClientAcceptAsyncEnrollmentErrorOpenBaoBadURL"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorOpenBaoNoServerResponse {
    tag: "ClientAcceptAsyncEnrollmentErrorOpenBaoNoServerResponse"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorPKICannotOpenCertificateStore {
    tag: "ClientAcceptAsyncEnrollmentErrorPKICannotOpenCertificateStore"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorPKIServerInvalidX509Trustchain {
    tag: "ClientAcceptAsyncEnrollmentErrorPKIServerInvalidX509Trustchain"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorPKIUnusableX509CertificateReference {
    tag: "ClientAcceptAsyncEnrollmentErrorPKIUnusableX509CertificateReference"
    error: string
}
export interface ClientAcceptAsyncEnrollmentErrorTimestampOutOfBallpark {
    tag: "ClientAcceptAsyncEnrollmentErrorTimestampOutOfBallpark"
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
export interface ClientAcceptTosErrorInternal {
    tag: "ClientAcceptTosErrorInternal"
    error: string
}
export interface ClientAcceptTosErrorNoTos {
    tag: "ClientAcceptTosErrorNoTos"
    error: string
}
export interface ClientAcceptTosErrorOffline {
    tag: "ClientAcceptTosErrorOffline"
    error: string
}
export interface ClientAcceptTosErrorTosMismatch {
    tag: "ClientAcceptTosErrorTosMismatch"
    error: string
}
export type ClientAcceptTosError =
  | ClientAcceptTosErrorInternal
  | ClientAcceptTosErrorNoTos
  | ClientAcceptTosErrorOffline
  | ClientAcceptTosErrorTosMismatch


// ClientCancelInvitationError
export interface ClientCancelInvitationErrorAlreadyCancelled {
    tag: "ClientCancelInvitationErrorAlreadyCancelled"
    error: string
}
export interface ClientCancelInvitationErrorCompleted {
    tag: "ClientCancelInvitationErrorCompleted"
    error: string
}
export interface ClientCancelInvitationErrorInternal {
    tag: "ClientCancelInvitationErrorInternal"
    error: string
}
export interface ClientCancelInvitationErrorNotAllowed {
    tag: "ClientCancelInvitationErrorNotAllowed"
    error: string
}
export interface ClientCancelInvitationErrorNotFound {
    tag: "ClientCancelInvitationErrorNotFound"
    error: string
}
export interface ClientCancelInvitationErrorOffline {
    tag: "ClientCancelInvitationErrorOffline"
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
export interface ClientCreateWorkspaceErrorInternal {
    tag: "ClientCreateWorkspaceErrorInternal"
    error: string
}
export interface ClientCreateWorkspaceErrorStopped {
    tag: "ClientCreateWorkspaceErrorStopped"
    error: string
}
export type ClientCreateWorkspaceError =
  | ClientCreateWorkspaceErrorInternal
  | ClientCreateWorkspaceErrorStopped


// ClientDeleteShamirRecoveryError
export interface ClientDeleteShamirRecoveryErrorInternal {
    tag: "ClientDeleteShamirRecoveryErrorInternal"
    error: string
}
export interface ClientDeleteShamirRecoveryErrorInvalidCertificate {
    tag: "ClientDeleteShamirRecoveryErrorInvalidCertificate"
    error: string
}
export interface ClientDeleteShamirRecoveryErrorOffline {
    tag: "ClientDeleteShamirRecoveryErrorOffline"
    error: string
}
export interface ClientDeleteShamirRecoveryErrorStopped {
    tag: "ClientDeleteShamirRecoveryErrorStopped"
    error: string
}
export interface ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark {
    tag: "ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export type ClientDeleteShamirRecoveryError =
  | ClientDeleteShamirRecoveryErrorInternal
  | ClientDeleteShamirRecoveryErrorInvalidCertificate
  | ClientDeleteShamirRecoveryErrorOffline
  | ClientDeleteShamirRecoveryErrorStopped
  | ClientDeleteShamirRecoveryErrorTimestampOutOfBallpark


// ClientEvent
export interface ClientEventClientErrorResponse {
    tag: "ClientEventClientErrorResponse"
    error_type: string
}
export interface ClientEventClientStarted {
    tag: "ClientEventClientStarted"
    device_id: string
}
export interface ClientEventClientStopped {
    tag: "ClientEventClientStopped"
    device_id: string
}
export interface ClientEventExpiredOrganization {
    tag: "ClientEventExpiredOrganization"
}
export interface ClientEventFrozenSelfUser {
    tag: "ClientEventFrozenSelfUser"
}
export interface ClientEventGreetingAttemptCancelled {
    tag: "ClientEventGreetingAttemptCancelled"
    token: string
    greeting_attempt: string
}
export interface ClientEventGreetingAttemptJoined {
    tag: "ClientEventGreetingAttemptJoined"
    token: string
    greeting_attempt: string
}
export interface ClientEventGreetingAttemptReady {
    tag: "ClientEventGreetingAttemptReady"
    token: string
    greeting_attempt: string
}
export interface ClientEventIncompatibleServer {
    tag: "ClientEventIncompatibleServer"
    api_version: string
    supported_api_version: Array<string>
}
export interface ClientEventInvalidCertificate {
    tag: "ClientEventInvalidCertificate"
    detail: string
}
export interface ClientEventInvitationAlreadyUsedOrDeleted {
    tag: "ClientEventInvitationAlreadyUsedOrDeleted"
}
export interface ClientEventInvitationChanged {
    tag: "ClientEventInvitationChanged"
    token: string
    status: InvitationStatus
}
export interface ClientEventMustAcceptTos {
    tag: "ClientEventMustAcceptTos"
}
export interface ClientEventOffline {
    tag: "ClientEventOffline"
}
export interface ClientEventOnline {
    tag: "ClientEventOnline"
}
export interface ClientEventOrganizationNotFound {
    tag: "ClientEventOrganizationNotFound"
}
export interface ClientEventPing {
    tag: "ClientEventPing"
    ping: string
}
export interface ClientEventRevokedSelfUser {
    tag: "ClientEventRevokedSelfUser"
}
export interface ClientEventServerConfigChanged {
    tag: "ClientEventServerConfigChanged"
}
export interface ClientEventServerInvalidResponseContent {
    tag: "ClientEventServerInvalidResponseContent"
    protocol_decode_error: string
}
export interface ClientEventServerInvalidResponseStatus {
    tag: "ClientEventServerInvalidResponseStatus"
    status_code: string
}
export interface ClientEventTooMuchDriftWithServerClock {
    tag: "ClientEventTooMuchDriftWithServerClock"
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientEventWebClientNotAllowedByOrganization {
    tag: "ClientEventWebClientNotAllowedByOrganization"
}
export interface ClientEventWorkspaceLocallyCreated {
    tag: "ClientEventWorkspaceLocallyCreated"
}
export interface ClientEventWorkspaceOpsInboundSyncDone {
    tag: "ClientEventWorkspaceOpsInboundSyncDone"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspaceOpsOutboundSyncAborted {
    tag: "ClientEventWorkspaceOpsOutboundSyncAborted"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspaceOpsOutboundSyncDone {
    tag: "ClientEventWorkspaceOpsOutboundSyncDone"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspaceOpsOutboundSyncProgress {
    tag: "ClientEventWorkspaceOpsOutboundSyncProgress"
    realm_id: string
    entry_id: string
    blocks: number
    block_index: number
    blocksize: number
}
export interface ClientEventWorkspaceOpsOutboundSyncStarted {
    tag: "ClientEventWorkspaceOpsOutboundSyncStarted"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspaceWatchedEntryChanged {
    tag: "ClientEventWorkspaceWatchedEntryChanged"
    realm_id: string
    entry_id: string
}
export interface ClientEventWorkspacesSelfListChanged {
    tag: "ClientEventWorkspacesSelfListChanged"
}
export type ClientEvent =
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
export interface ClientExportRecoveryDeviceErrorInternal {
    tag: "ClientExportRecoveryDeviceErrorInternal"
    error: string
}
export interface ClientExportRecoveryDeviceErrorInvalidCertificate {
    tag: "ClientExportRecoveryDeviceErrorInvalidCertificate"
    error: string
}
export interface ClientExportRecoveryDeviceErrorOffline {
    tag: "ClientExportRecoveryDeviceErrorOffline"
    error: string
}
export interface ClientExportRecoveryDeviceErrorStopped {
    tag: "ClientExportRecoveryDeviceErrorStopped"
    error: string
}
export interface ClientExportRecoveryDeviceErrorTimestampOutOfBallpark {
    tag: "ClientExportRecoveryDeviceErrorTimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export type ClientExportRecoveryDeviceError =
  | ClientExportRecoveryDeviceErrorInternal
  | ClientExportRecoveryDeviceErrorInvalidCertificate
  | ClientExportRecoveryDeviceErrorOffline
  | ClientExportRecoveryDeviceErrorStopped
  | ClientExportRecoveryDeviceErrorTimestampOutOfBallpark


// ClientForgetAllCertificatesError
export interface ClientForgetAllCertificatesErrorInternal {
    tag: "ClientForgetAllCertificatesErrorInternal"
    error: string
}
export interface ClientForgetAllCertificatesErrorStopped {
    tag: "ClientForgetAllCertificatesErrorStopped"
    error: string
}
export type ClientForgetAllCertificatesError =
  | ClientForgetAllCertificatesErrorInternal
  | ClientForgetAllCertificatesErrorStopped


// ClientGetAsyncEnrollmentAddrError
export interface ClientGetAsyncEnrollmentAddrErrorInternal {
    tag: "ClientGetAsyncEnrollmentAddrErrorInternal"
    error: string
}
export type ClientGetAsyncEnrollmentAddrError =
  | ClientGetAsyncEnrollmentAddrErrorInternal


// ClientGetOrganizationBootstrapDateError
export interface ClientGetOrganizationBootstrapDateErrorBootstrapDateNotFound {
    tag: "ClientGetOrganizationBootstrapDateErrorBootstrapDateNotFound"
    error: string
}
export interface ClientGetOrganizationBootstrapDateErrorInternal {
    tag: "ClientGetOrganizationBootstrapDateErrorInternal"
    error: string
}
export interface ClientGetOrganizationBootstrapDateErrorInvalidCertificate {
    tag: "ClientGetOrganizationBootstrapDateErrorInvalidCertificate"
    error: string
}
export interface ClientGetOrganizationBootstrapDateErrorOffline {
    tag: "ClientGetOrganizationBootstrapDateErrorOffline"
    error: string
}
export interface ClientGetOrganizationBootstrapDateErrorStopped {
    tag: "ClientGetOrganizationBootstrapDateErrorStopped"
    error: string
}
export type ClientGetOrganizationBootstrapDateError =
  | ClientGetOrganizationBootstrapDateErrorBootstrapDateNotFound
  | ClientGetOrganizationBootstrapDateErrorInternal
  | ClientGetOrganizationBootstrapDateErrorInvalidCertificate
  | ClientGetOrganizationBootstrapDateErrorOffline
  | ClientGetOrganizationBootstrapDateErrorStopped


// ClientGetSelfShamirRecoveryError
export interface ClientGetSelfShamirRecoveryErrorInternal {
    tag: "ClientGetSelfShamirRecoveryErrorInternal"
    error: string
}
export interface ClientGetSelfShamirRecoveryErrorStopped {
    tag: "ClientGetSelfShamirRecoveryErrorStopped"
    error: string
}
export type ClientGetSelfShamirRecoveryError =
  | ClientGetSelfShamirRecoveryErrorInternal
  | ClientGetSelfShamirRecoveryErrorStopped


// ClientGetTosError
export interface ClientGetTosErrorInternal {
    tag: "ClientGetTosErrorInternal"
    error: string
}
export interface ClientGetTosErrorNoTos {
    tag: "ClientGetTosErrorNoTos"
    error: string
}
export interface ClientGetTosErrorOffline {
    tag: "ClientGetTosErrorOffline"
    error: string
}
export type ClientGetTosError =
  | ClientGetTosErrorInternal
  | ClientGetTosErrorNoTos
  | ClientGetTosErrorOffline


// ClientGetUserDeviceError
export interface ClientGetUserDeviceErrorInternal {
    tag: "ClientGetUserDeviceErrorInternal"
    error: string
}
export interface ClientGetUserDeviceErrorNonExisting {
    tag: "ClientGetUserDeviceErrorNonExisting"
    error: string
}
export interface ClientGetUserDeviceErrorStopped {
    tag: "ClientGetUserDeviceErrorStopped"
    error: string
}
export type ClientGetUserDeviceError =
  | ClientGetUserDeviceErrorInternal
  | ClientGetUserDeviceErrorNonExisting
  | ClientGetUserDeviceErrorStopped


// ClientGetUserInfoError
export interface ClientGetUserInfoErrorInternal {
    tag: "ClientGetUserInfoErrorInternal"
    error: string
}
export interface ClientGetUserInfoErrorNonExisting {
    tag: "ClientGetUserInfoErrorNonExisting"
    error: string
}
export interface ClientGetUserInfoErrorStopped {
    tag: "ClientGetUserInfoErrorStopped"
    error: string
}
export type ClientGetUserInfoError =
  | ClientGetUserInfoErrorInternal
  | ClientGetUserInfoErrorNonExisting
  | ClientGetUserInfoErrorStopped


// ClientInfoError
export interface ClientInfoErrorInternal {
    tag: "ClientInfoErrorInternal"
    error: string
}
export interface ClientInfoErrorStopped {
    tag: "ClientInfoErrorStopped"
    error: string
}
export type ClientInfoError =
  | ClientInfoErrorInternal
  | ClientInfoErrorStopped


// ClientListAsyncEnrollmentsError
export interface ClientListAsyncEnrollmentsErrorAuthorNotAllowed {
    tag: "ClientListAsyncEnrollmentsErrorAuthorNotAllowed"
    error: string
}
export interface ClientListAsyncEnrollmentsErrorInternal {
    tag: "ClientListAsyncEnrollmentsErrorInternal"
    error: string
}
export interface ClientListAsyncEnrollmentsErrorOffline {
    tag: "ClientListAsyncEnrollmentsErrorOffline"
    error: string
}
export type ClientListAsyncEnrollmentsError =
  | ClientListAsyncEnrollmentsErrorAuthorNotAllowed
  | ClientListAsyncEnrollmentsErrorInternal
  | ClientListAsyncEnrollmentsErrorOffline


// ClientListFrozenUsersError
export interface ClientListFrozenUsersErrorAuthorNotAllowed {
    tag: "ClientListFrozenUsersErrorAuthorNotAllowed"
    error: string
}
export interface ClientListFrozenUsersErrorInternal {
    tag: "ClientListFrozenUsersErrorInternal"
    error: string
}
export interface ClientListFrozenUsersErrorOffline {
    tag: "ClientListFrozenUsersErrorOffline"
    error: string
}
export type ClientListFrozenUsersError =
  | ClientListFrozenUsersErrorAuthorNotAllowed
  | ClientListFrozenUsersErrorInternal
  | ClientListFrozenUsersErrorOffline


// ClientListShamirRecoveriesForOthersError
export interface ClientListShamirRecoveriesForOthersErrorInternal {
    tag: "ClientListShamirRecoveriesForOthersErrorInternal"
    error: string
}
export interface ClientListShamirRecoveriesForOthersErrorStopped {
    tag: "ClientListShamirRecoveriesForOthersErrorStopped"
    error: string
}
export type ClientListShamirRecoveriesForOthersError =
  | ClientListShamirRecoveriesForOthersErrorInternal
  | ClientListShamirRecoveriesForOthersErrorStopped


// ClientListUserDevicesError
export interface ClientListUserDevicesErrorInternal {
    tag: "ClientListUserDevicesErrorInternal"
    error: string
}
export interface ClientListUserDevicesErrorStopped {
    tag: "ClientListUserDevicesErrorStopped"
    error: string
}
export type ClientListUserDevicesError =
  | ClientListUserDevicesErrorInternal
  | ClientListUserDevicesErrorStopped


// ClientListUsersError
export interface ClientListUsersErrorInternal {
    tag: "ClientListUsersErrorInternal"
    error: string
}
export interface ClientListUsersErrorStopped {
    tag: "ClientListUsersErrorStopped"
    error: string
}
export type ClientListUsersError =
  | ClientListUsersErrorInternal
  | ClientListUsersErrorStopped


// ClientListWorkspaceUsersError
export interface ClientListWorkspaceUsersErrorInternal {
    tag: "ClientListWorkspaceUsersErrorInternal"
    error: string
}
export interface ClientListWorkspaceUsersErrorStopped {
    tag: "ClientListWorkspaceUsersErrorStopped"
    error: string
}
export type ClientListWorkspaceUsersError =
  | ClientListWorkspaceUsersErrorInternal
  | ClientListWorkspaceUsersErrorStopped


// ClientListWorkspacesError
export interface ClientListWorkspacesErrorInternal {
    tag: "ClientListWorkspacesErrorInternal"
    error: string
}
export type ClientListWorkspacesError =
  | ClientListWorkspacesErrorInternal


// ClientNewDeviceInvitationError
export interface ClientNewDeviceInvitationErrorInternal {
    tag: "ClientNewDeviceInvitationErrorInternal"
    error: string
}
export interface ClientNewDeviceInvitationErrorOffline {
    tag: "ClientNewDeviceInvitationErrorOffline"
    error: string
}
export type ClientNewDeviceInvitationError =
  | ClientNewDeviceInvitationErrorInternal
  | ClientNewDeviceInvitationErrorOffline


// ClientNewShamirRecoveryInvitationError
export interface ClientNewShamirRecoveryInvitationErrorInternal {
    tag: "ClientNewShamirRecoveryInvitationErrorInternal"
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorNotAllowed {
    tag: "ClientNewShamirRecoveryInvitationErrorNotAllowed"
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorOffline {
    tag: "ClientNewShamirRecoveryInvitationErrorOffline"
    error: string
}
export interface ClientNewShamirRecoveryInvitationErrorUserNotFound {
    tag: "ClientNewShamirRecoveryInvitationErrorUserNotFound"
    error: string
}
export type ClientNewShamirRecoveryInvitationError =
  | ClientNewShamirRecoveryInvitationErrorInternal
  | ClientNewShamirRecoveryInvitationErrorNotAllowed
  | ClientNewShamirRecoveryInvitationErrorOffline
  | ClientNewShamirRecoveryInvitationErrorUserNotFound


// ClientNewUserInvitationError
export interface ClientNewUserInvitationErrorAlreadyMember {
    tag: "ClientNewUserInvitationErrorAlreadyMember"
    error: string
}
export interface ClientNewUserInvitationErrorInternal {
    tag: "ClientNewUserInvitationErrorInternal"
    error: string
}
export interface ClientNewUserInvitationErrorNotAllowed {
    tag: "ClientNewUserInvitationErrorNotAllowed"
    error: string
}
export interface ClientNewUserInvitationErrorOffline {
    tag: "ClientNewUserInvitationErrorOffline"
    error: string
}
export type ClientNewUserInvitationError =
  | ClientNewUserInvitationErrorAlreadyMember
  | ClientNewUserInvitationErrorInternal
  | ClientNewUserInvitationErrorNotAllowed
  | ClientNewUserInvitationErrorOffline


// ClientOrganizationInfoError
export interface ClientOrganizationInfoErrorInternal {
    tag: "ClientOrganizationInfoErrorInternal"
    error: string
}
export interface ClientOrganizationInfoErrorOffline {
    tag: "ClientOrganizationInfoErrorOffline"
    error: string
}
export type ClientOrganizationInfoError =
  | ClientOrganizationInfoErrorInternal
  | ClientOrganizationInfoErrorOffline


// ClientRejectAsyncEnrollmentError
export interface ClientRejectAsyncEnrollmentErrorAuthorNotAllowed {
    tag: "ClientRejectAsyncEnrollmentErrorAuthorNotAllowed"
    error: string
}
export interface ClientRejectAsyncEnrollmentErrorEnrollmentNoLongerAvailable {
    tag: "ClientRejectAsyncEnrollmentErrorEnrollmentNoLongerAvailable"
    error: string
}
export interface ClientRejectAsyncEnrollmentErrorEnrollmentNotFound {
    tag: "ClientRejectAsyncEnrollmentErrorEnrollmentNotFound"
    error: string
}
export interface ClientRejectAsyncEnrollmentErrorInternal {
    tag: "ClientRejectAsyncEnrollmentErrorInternal"
    error: string
}
export interface ClientRejectAsyncEnrollmentErrorOffline {
    tag: "ClientRejectAsyncEnrollmentErrorOffline"
    error: string
}
export type ClientRejectAsyncEnrollmentError =
  | ClientRejectAsyncEnrollmentErrorAuthorNotAllowed
  | ClientRejectAsyncEnrollmentErrorEnrollmentNoLongerAvailable
  | ClientRejectAsyncEnrollmentErrorEnrollmentNotFound
  | ClientRejectAsyncEnrollmentErrorInternal
  | ClientRejectAsyncEnrollmentErrorOffline


// ClientRenameWorkspaceError
export interface ClientRenameWorkspaceErrorAuthorNotAllowed {
    tag: "ClientRenameWorkspaceErrorAuthorNotAllowed"
    error: string
}
export interface ClientRenameWorkspaceErrorInternal {
    tag: "ClientRenameWorkspaceErrorInternal"
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidCertificate {
    tag: "ClientRenameWorkspaceErrorInvalidCertificate"
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidEncryptedRealmName {
    tag: "ClientRenameWorkspaceErrorInvalidEncryptedRealmName"
    error: string
}
export interface ClientRenameWorkspaceErrorInvalidKeysBundle {
    tag: "ClientRenameWorkspaceErrorInvalidKeysBundle"
    error: string
}
export interface ClientRenameWorkspaceErrorNoKey {
    tag: "ClientRenameWorkspaceErrorNoKey"
    error: string
}
export interface ClientRenameWorkspaceErrorOffline {
    tag: "ClientRenameWorkspaceErrorOffline"
    error: string
}
export interface ClientRenameWorkspaceErrorStopped {
    tag: "ClientRenameWorkspaceErrorStopped"
    error: string
}
export interface ClientRenameWorkspaceErrorTimestampOutOfBallpark {
    tag: "ClientRenameWorkspaceErrorTimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientRenameWorkspaceErrorWorkspaceNotFound {
    tag: "ClientRenameWorkspaceErrorWorkspaceNotFound"
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
export interface ClientRevokeUserErrorAuthorNotAllowed {
    tag: "ClientRevokeUserErrorAuthorNotAllowed"
    error: string
}
export interface ClientRevokeUserErrorInternal {
    tag: "ClientRevokeUserErrorInternal"
    error: string
}
export interface ClientRevokeUserErrorInvalidCertificate {
    tag: "ClientRevokeUserErrorInvalidCertificate"
    error: string
}
export interface ClientRevokeUserErrorInvalidKeysBundle {
    tag: "ClientRevokeUserErrorInvalidKeysBundle"
    error: string
}
export interface ClientRevokeUserErrorNoKey {
    tag: "ClientRevokeUserErrorNoKey"
    error: string
}
export interface ClientRevokeUserErrorOffline {
    tag: "ClientRevokeUserErrorOffline"
    error: string
}
export interface ClientRevokeUserErrorStopped {
    tag: "ClientRevokeUserErrorStopped"
    error: string
}
export interface ClientRevokeUserErrorTimestampOutOfBallpark {
    tag: "ClientRevokeUserErrorTimestampOutOfBallpark"
    error: string
}
export interface ClientRevokeUserErrorUserIsSelf {
    tag: "ClientRevokeUserErrorUserIsSelf"
    error: string
}
export interface ClientRevokeUserErrorUserNotFound {
    tag: "ClientRevokeUserErrorUserNotFound"
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
export interface ClientSetupShamirRecoveryErrorAuthorAmongRecipients {
    tag: "ClientSetupShamirRecoveryErrorAuthorAmongRecipients"
    error: string
}
export interface ClientSetupShamirRecoveryErrorInternal {
    tag: "ClientSetupShamirRecoveryErrorInternal"
    error: string
}
export interface ClientSetupShamirRecoveryErrorInvalidCertificate {
    tag: "ClientSetupShamirRecoveryErrorInvalidCertificate"
    error: string
}
export interface ClientSetupShamirRecoveryErrorOffline {
    tag: "ClientSetupShamirRecoveryErrorOffline"
    error: string
}
export interface ClientSetupShamirRecoveryErrorRecipientNotFound {
    tag: "ClientSetupShamirRecoveryErrorRecipientNotFound"
    error: string
}
export interface ClientSetupShamirRecoveryErrorRecipientRevoked {
    tag: "ClientSetupShamirRecoveryErrorRecipientRevoked"
    error: string
}
export interface ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists {
    tag: "ClientSetupShamirRecoveryErrorShamirRecoveryAlreadyExists"
    error: string
}
export interface ClientSetupShamirRecoveryErrorStopped {
    tag: "ClientSetupShamirRecoveryErrorStopped"
    error: string
}
export interface ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares {
    tag: "ClientSetupShamirRecoveryErrorThresholdBiggerThanSumOfShares"
    error: string
}
export interface ClientSetupShamirRecoveryErrorTimestampOutOfBallpark {
    tag: "ClientSetupShamirRecoveryErrorTimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientSetupShamirRecoveryErrorTooManyShares {
    tag: "ClientSetupShamirRecoveryErrorTooManyShares"
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
export interface ClientShareWorkspaceErrorAuthorNotAllowed {
    tag: "ClientShareWorkspaceErrorAuthorNotAllowed"
    error: string
}
export interface ClientShareWorkspaceErrorInternal {
    tag: "ClientShareWorkspaceErrorInternal"
    error: string
}
export interface ClientShareWorkspaceErrorInvalidCertificate {
    tag: "ClientShareWorkspaceErrorInvalidCertificate"
    error: string
}
export interface ClientShareWorkspaceErrorInvalidKeysBundle {
    tag: "ClientShareWorkspaceErrorInvalidKeysBundle"
    error: string
}
export interface ClientShareWorkspaceErrorOffline {
    tag: "ClientShareWorkspaceErrorOffline"
    error: string
}
export interface ClientShareWorkspaceErrorRecipientIsSelf {
    tag: "ClientShareWorkspaceErrorRecipientIsSelf"
    error: string
}
export interface ClientShareWorkspaceErrorRecipientNotFound {
    tag: "ClientShareWorkspaceErrorRecipientNotFound"
    error: string
}
export interface ClientShareWorkspaceErrorRecipientRevoked {
    tag: "ClientShareWorkspaceErrorRecipientRevoked"
    error: string
}
export interface ClientShareWorkspaceErrorRoleIncompatibleWithOutsider {
    tag: "ClientShareWorkspaceErrorRoleIncompatibleWithOutsider"
    error: string
}
export interface ClientShareWorkspaceErrorStopped {
    tag: "ClientShareWorkspaceErrorStopped"
    error: string
}
export interface ClientShareWorkspaceErrorTimestampOutOfBallpark {
    tag: "ClientShareWorkspaceErrorTimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface ClientShareWorkspaceErrorWorkspaceNotFound {
    tag: "ClientShareWorkspaceErrorWorkspaceNotFound"
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
export interface ClientStartErrorDeviceUsedByAnotherProcess {
    tag: "ClientStartErrorDeviceUsedByAnotherProcess"
    error: string
}
export interface ClientStartErrorInternal {
    tag: "ClientStartErrorInternal"
    error: string
}
export interface ClientStartErrorLoadDeviceDecryptionFailed {
    tag: "ClientStartErrorLoadDeviceDecryptionFailed"
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidData {
    tag: "ClientStartErrorLoadDeviceInvalidData"
    error: string
}
export interface ClientStartErrorLoadDeviceInvalidPath {
    tag: "ClientStartErrorLoadDeviceInvalidPath"
    error: string
}
export interface ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchFailed {
    tag: "ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchFailed"
    error: string
}
export interface ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchOffline {
    tag: "ClientStartErrorLoadDeviceRemoteOpaqueKeyFetchOffline"
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


// ClientStartInvitationGreetError
export interface ClientStartInvitationGreetErrorInternal {
    tag: "ClientStartInvitationGreetErrorInternal"
    error: string
}
export type ClientStartInvitationGreetError =
  | ClientStartInvitationGreetErrorInternal


// ClientStartShamirRecoveryInvitationGreetError
export interface ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorCorruptedShareData"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInternal {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorInternal"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorInvalidCertificate"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorInvitationNotFound"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorOffline {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorOffline"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryDeleted"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryNotFound"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorShamirRecoveryUnusable"
    error: string
}
export interface ClientStartShamirRecoveryInvitationGreetErrorStopped {
    tag: "ClientStartShamirRecoveryInvitationGreetErrorStopped"
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
export interface ClientStartWorkspaceErrorInternal {
    tag: "ClientStartWorkspaceErrorInternal"
    error: string
}
export interface ClientStartWorkspaceErrorWorkspaceNotFound {
    tag: "ClientStartWorkspaceErrorWorkspaceNotFound"
    error: string
}
export type ClientStartWorkspaceError =
  | ClientStartWorkspaceErrorInternal
  | ClientStartWorkspaceErrorWorkspaceNotFound


// ClientStopError
export interface ClientStopErrorInternal {
    tag: "ClientStopErrorInternal"
    error: string
}
export type ClientStopError =
  | ClientStopErrorInternal


// ClientUserUpdateProfileError
export interface ClientUserUpdateProfileErrorAuthorNotAllowed {
    tag: "ClientUserUpdateProfileErrorAuthorNotAllowed"
    error: string
}
export interface ClientUserUpdateProfileErrorInternal {
    tag: "ClientUserUpdateProfileErrorInternal"
    error: string
}
export interface ClientUserUpdateProfileErrorInvalidCertificate {
    tag: "ClientUserUpdateProfileErrorInvalidCertificate"
    error: string
}
export interface ClientUserUpdateProfileErrorOffline {
    tag: "ClientUserUpdateProfileErrorOffline"
    error: string
}
export interface ClientUserUpdateProfileErrorStopped {
    tag: "ClientUserUpdateProfileErrorStopped"
    error: string
}
export interface ClientUserUpdateProfileErrorTimestampOutOfBallpark {
    tag: "ClientUserUpdateProfileErrorTimestampOutOfBallpark"
    error: string
}
export interface ClientUserUpdateProfileErrorUserIsSelf {
    tag: "ClientUserUpdateProfileErrorUserIsSelf"
    error: string
}
export interface ClientUserUpdateProfileErrorUserNotFound {
    tag: "ClientUserUpdateProfileErrorUserNotFound"
    error: string
}
export interface ClientUserUpdateProfileErrorUserRevoked {
    tag: "ClientUserUpdateProfileErrorUserRevoked"
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
export interface DeviceAccessStrategyAccountVault {
    tag: "DeviceAccessStrategyAccountVault"
    key_file: string
    account_handle: number
}
export interface DeviceAccessStrategyKeyring {
    tag: "DeviceAccessStrategyKeyring"
    key_file: string
}
export interface DeviceAccessStrategyOpenBao {
    tag: "DeviceAccessStrategyOpenBao"
    key_file: string
    openbao_server_url: string
    openbao_secret_mount_path: string
    openbao_transit_mount_path: string
    openbao_entity_id: string
    openbao_auth_token: string
}
export interface DeviceAccessStrategyPKI {
    tag: "DeviceAccessStrategyPKI"
    key_file: string
}
export interface DeviceAccessStrategyPassword {
    tag: "DeviceAccessStrategyPassword"
    password: string
    key_file: string
}
export type DeviceAccessStrategy =
  | DeviceAccessStrategyAccountVault
  | DeviceAccessStrategyKeyring
  | DeviceAccessStrategyOpenBao
  | DeviceAccessStrategyPKI
  | DeviceAccessStrategyPassword


// DeviceSaveStrategy
export interface DeviceSaveStrategyAccountVault {
    tag: "DeviceSaveStrategyAccountVault"
    account_handle: number
}
export interface DeviceSaveStrategyKeyring {
    tag: "DeviceSaveStrategyKeyring"
}
export interface DeviceSaveStrategyOpenBao {
    tag: "DeviceSaveStrategyOpenBao"
    openbao_server_url: string
    openbao_secret_mount_path: string
    openbao_transit_mount_path: string
    openbao_entity_id: string
    openbao_auth_token: string
    openbao_preferred_auth_id: string
}
export interface DeviceSaveStrategyPKI {
    tag: "DeviceSaveStrategyPKI"
    certificate_ref: X509CertificateReference
}
export interface DeviceSaveStrategyPassword {
    tag: "DeviceSaveStrategyPassword"
    password: string
}
export type DeviceSaveStrategy =
  | DeviceSaveStrategyAccountVault
  | DeviceSaveStrategyKeyring
  | DeviceSaveStrategyOpenBao
  | DeviceSaveStrategyPKI
  | DeviceSaveStrategyPassword


// EntryStat
export interface EntryStatFile {
    tag: "EntryStatFile"
    confinement_point: string | null
    id: string
    parent: string
    created: number
    updated: number
    base_version: number
    is_placeholder: boolean
    need_sync: boolean
    size: number
    last_updater: string
}
export interface EntryStatFolder {
    tag: "EntryStatFolder"
    confinement_point: string | null
    id: string
    parent: string
    created: number
    updated: number
    base_version: number
    is_placeholder: boolean
    need_sync: boolean
    last_updater: string
}
export type EntryStat =
  | EntryStatFile
  | EntryStatFolder


// GetServerConfigError
export interface GetServerConfigErrorInternal {
    tag: "GetServerConfigErrorInternal"
    error: string
}
export interface GetServerConfigErrorOffline {
    tag: "GetServerConfigErrorOffline"
    error: string
}
export type GetServerConfigError =
  | GetServerConfigErrorInternal
  | GetServerConfigErrorOffline


// GreetInProgressError
export interface GreetInProgressErrorActiveUsersLimitReached {
    tag: "GreetInProgressErrorActiveUsersLimitReached"
    error: string
}
export interface GreetInProgressErrorAlreadyDeleted {
    tag: "GreetInProgressErrorAlreadyDeleted"
    error: string
}
export interface GreetInProgressErrorCancelled {
    tag: "GreetInProgressErrorCancelled"
    error: string
}
export interface GreetInProgressErrorCorruptedInviteUserData {
    tag: "GreetInProgressErrorCorruptedInviteUserData"
    error: string
}
export interface GreetInProgressErrorCorruptedSharedSecretKey {
    tag: "GreetInProgressErrorCorruptedSharedSecretKey"
    error: string
}
export interface GreetInProgressErrorDeviceAlreadyExists {
    tag: "GreetInProgressErrorDeviceAlreadyExists"
    error: string
}
export interface GreetInProgressErrorGreeterNotAllowed {
    tag: "GreetInProgressErrorGreeterNotAllowed"
    error: string
}
export interface GreetInProgressErrorGreetingAttemptCancelled {
    tag: "GreetInProgressErrorGreetingAttemptCancelled"
    error: string
    origin: GreeterOrClaimer
    reason: CancelledGreetingAttemptReason
    timestamp: number
}
export interface GreetInProgressErrorHumanHandleAlreadyTaken {
    tag: "GreetInProgressErrorHumanHandleAlreadyTaken"
    error: string
}
export interface GreetInProgressErrorInternal {
    tag: "GreetInProgressErrorInternal"
    error: string
}
export interface GreetInProgressErrorNonceMismatch {
    tag: "GreetInProgressErrorNonceMismatch"
    error: string
}
export interface GreetInProgressErrorNotFound {
    tag: "GreetInProgressErrorNotFound"
    error: string
}
export interface GreetInProgressErrorOffline {
    tag: "GreetInProgressErrorOffline"
    error: string
}
export interface GreetInProgressErrorPeerReset {
    tag: "GreetInProgressErrorPeerReset"
    error: string
}
export interface GreetInProgressErrorTimestampOutOfBallpark {
    tag: "GreetInProgressErrorTimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export interface GreetInProgressErrorUserAlreadyExists {
    tag: "GreetInProgressErrorUserAlreadyExists"
    error: string
}
export interface GreetInProgressErrorUserCreateNotAllowed {
    tag: "GreetInProgressErrorUserCreateNotAllowed"
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
export interface ImportRecoveryDeviceErrorDecryptionFailed {
    tag: "ImportRecoveryDeviceErrorDecryptionFailed"
    error: string
}
export interface ImportRecoveryDeviceErrorInternal {
    tag: "ImportRecoveryDeviceErrorInternal"
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidCertificate {
    tag: "ImportRecoveryDeviceErrorInvalidCertificate"
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidData {
    tag: "ImportRecoveryDeviceErrorInvalidData"
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidPassphrase {
    tag: "ImportRecoveryDeviceErrorInvalidPassphrase"
    error: string
}
export interface ImportRecoveryDeviceErrorInvalidPath {
    tag: "ImportRecoveryDeviceErrorInvalidPath"
    error: string
}
export interface ImportRecoveryDeviceErrorOffline {
    tag: "ImportRecoveryDeviceErrorOffline"
    error: string
}
export interface ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadFailed {
    tag: "ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadFailed"
    error: string
}
export interface ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadOffline {
    tag: "ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadOffline"
    error: string
}
export interface ImportRecoveryDeviceErrorStopped {
    tag: "ImportRecoveryDeviceErrorStopped"
    error: string
}
export interface ImportRecoveryDeviceErrorStorageNotAvailable {
    tag: "ImportRecoveryDeviceErrorStorageNotAvailable"
    error: string
}
export interface ImportRecoveryDeviceErrorTimestampOutOfBallpark {
    tag: "ImportRecoveryDeviceErrorTimestampOutOfBallpark"
    error: string
    server_timestamp: number
    client_timestamp: number
    ballpark_client_early_offset: number
    ballpark_client_late_offset: number
}
export type ImportRecoveryDeviceError =
  | ImportRecoveryDeviceErrorDecryptionFailed
  | ImportRecoveryDeviceErrorInternal
  | ImportRecoveryDeviceErrorInvalidCertificate
  | ImportRecoveryDeviceErrorInvalidData
  | ImportRecoveryDeviceErrorInvalidPassphrase
  | ImportRecoveryDeviceErrorInvalidPath
  | ImportRecoveryDeviceErrorOffline
  | ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadFailed
  | ImportRecoveryDeviceErrorRemoteOpaqueKeyUploadOffline
  | ImportRecoveryDeviceErrorStopped
  | ImportRecoveryDeviceErrorStorageNotAvailable
  | ImportRecoveryDeviceErrorTimestampOutOfBallpark


// InvalidityReason
export interface InvalidityReasonCannotGetCertificateInfo {
    tag: "InvalidityReasonCannotGetCertificateInfo"
}
export interface InvalidityReasonCannotOpenStore {
    tag: "InvalidityReasonCannotOpenStore"
}
export interface InvalidityReasonDataError {
    tag: "InvalidityReasonDataError"
}
export interface InvalidityReasonInvalidCertificateDer {
    tag: "InvalidityReasonInvalidCertificateDer"
}
export interface InvalidityReasonInvalidRootCertificate {
    tag: "InvalidityReasonInvalidRootCertificate"
}
export interface InvalidityReasonInvalidSignature {
    tag: "InvalidityReasonInvalidSignature"
}
export interface InvalidityReasonInvalidUserInformation {
    tag: "InvalidityReasonInvalidUserInformation"
}
export interface InvalidityReasonNotFound {
    tag: "InvalidityReasonNotFound"
}
export interface InvalidityReasonUntrusted {
    tag: "InvalidityReasonUntrusted"
}
export type InvalidityReason =
  | InvalidityReasonCannotGetCertificateInfo
  | InvalidityReasonCannotOpenStore
  | InvalidityReasonDataError
  | InvalidityReasonInvalidCertificateDer
  | InvalidityReasonInvalidRootCertificate
  | InvalidityReasonInvalidSignature
  | InvalidityReasonInvalidUserInformation
  | InvalidityReasonNotFound
  | InvalidityReasonUntrusted


// InviteInfoInvitationCreatedBy
export interface InviteInfoInvitationCreatedByExternalService {
    tag: "InviteInfoInvitationCreatedByExternalService"
    service_label: string
}
export interface InviteInfoInvitationCreatedByUser {
    tag: "InviteInfoInvitationCreatedByUser"
    user_id: string
    human_handle: HumanHandle
}
export type InviteInfoInvitationCreatedBy =
  | InviteInfoInvitationCreatedByExternalService
  | InviteInfoInvitationCreatedByUser


// InviteListInvitationCreatedBy
export interface InviteListInvitationCreatedByExternalService {
    tag: "InviteListInvitationCreatedByExternalService"
    service_label: string
}
export interface InviteListInvitationCreatedByUser {
    tag: "InviteListInvitationCreatedByUser"
    user_id: string
    human_handle: HumanHandle
}
export type InviteListInvitationCreatedBy =
  | InviteListInvitationCreatedByExternalService
  | InviteListInvitationCreatedByUser


// InviteListItem
export interface InviteListItemDevice {
    tag: "InviteListItemDevice"
    addr: string
    token: string
    created_on: number
    created_by: InviteListInvitationCreatedBy
    status: InvitationStatus
}
export interface InviteListItemShamirRecovery {
    tag: "InviteListItemShamirRecovery"
    addr: string
    token: string
    created_on: number
    created_by: InviteListInvitationCreatedBy
    claimer_user_id: string
    shamir_recovery_created_on: number
    status: InvitationStatus
}
export interface InviteListItemUser {
    tag: "InviteListItemUser"
    addr: string
    token: string
    created_on: number
    created_by: InviteListInvitationCreatedBy
    claimer_email: string
    status: InvitationStatus
}
export type InviteListItem =
  | InviteListItemDevice
  | InviteListItemShamirRecovery
  | InviteListItemUser


// ListAvailableDeviceError
export interface ListAvailableDeviceErrorInternal {
    tag: "ListAvailableDeviceErrorInternal"
    error: string
}
export interface ListAvailableDeviceErrorStorageNotAvailable {
    tag: "ListAvailableDeviceErrorStorageNotAvailable"
    error: string
}
export type ListAvailableDeviceError =
  | ListAvailableDeviceErrorInternal
  | ListAvailableDeviceErrorStorageNotAvailable


// ListInvitationsError
export interface ListInvitationsErrorInternal {
    tag: "ListInvitationsErrorInternal"
    error: string
}
export interface ListInvitationsErrorOffline {
    tag: "ListInvitationsErrorOffline"
    error: string
}
export type ListInvitationsError =
  | ListInvitationsErrorInternal
  | ListInvitationsErrorOffline


// ListPkiLocalPendingError
export interface ListPkiLocalPendingErrorInternal {
    tag: "ListPkiLocalPendingErrorInternal"
    error: string
}
export interface ListPkiLocalPendingErrorStorageNotAvailable {
    tag: "ListPkiLocalPendingErrorStorageNotAvailable"
    error: string
}
export type ListPkiLocalPendingError =
  | ListPkiLocalPendingErrorInternal
  | ListPkiLocalPendingErrorStorageNotAvailable


// MountpointMountStrategy
export interface MountpointMountStrategyDirectory {
    tag: "MountpointMountStrategyDirectory"
    base_dir: string
}
export interface MountpointMountStrategyDisabled {
    tag: "MountpointMountStrategyDisabled"
}
export interface MountpointMountStrategyDriveLetter {
    tag: "MountpointMountStrategyDriveLetter"
}
export type MountpointMountStrategy =
  | MountpointMountStrategyDirectory
  | MountpointMountStrategyDisabled
  | MountpointMountStrategyDriveLetter


// MountpointToOsPathError
export interface MountpointToOsPathErrorInternal {
    tag: "MountpointToOsPathErrorInternal"
    error: string
}
export type MountpointToOsPathError =
  | MountpointToOsPathErrorInternal


// MountpointUnmountError
export interface MountpointUnmountErrorInternal {
    tag: "MountpointUnmountErrorInternal"
    error: string
}
export type MountpointUnmountError =
  | MountpointUnmountErrorInternal


// MoveEntryMode
export interface MoveEntryModeCanReplace {
    tag: "MoveEntryModeCanReplace"
}
export interface MoveEntryModeCanReplaceFileOnly {
    tag: "MoveEntryModeCanReplaceFileOnly"
}
export interface MoveEntryModeExchange {
    tag: "MoveEntryModeExchange"
}
export interface MoveEntryModeNoReplace {
    tag: "MoveEntryModeNoReplace"
}
export type MoveEntryMode =
  | MoveEntryModeCanReplace
  | MoveEntryModeCanReplaceFileOnly
  | MoveEntryModeExchange
  | MoveEntryModeNoReplace


// OpenBaoAuthConfig
export interface OpenBaoAuthConfigOIDCHexagone {
    tag: "OpenBaoAuthConfigOIDCHexagone"
    mount_path: string
}
export interface OpenBaoAuthConfigOIDCProConnect {
    tag: "OpenBaoAuthConfigOIDCProConnect"
    mount_path: string
}
export type OpenBaoAuthConfig =
  | OpenBaoAuthConfigOIDCHexagone
  | OpenBaoAuthConfigOIDCProConnect


// OpenBaoListSelfEmailsError
export interface OpenBaoListSelfEmailsErrorBadServerResponse {
    tag: "OpenBaoListSelfEmailsErrorBadServerResponse"
    error: string
}
export interface OpenBaoListSelfEmailsErrorBadURL {
    tag: "OpenBaoListSelfEmailsErrorBadURL"
    error: string
}
export interface OpenBaoListSelfEmailsErrorInternal {
    tag: "OpenBaoListSelfEmailsErrorInternal"
    error: string
}
export interface OpenBaoListSelfEmailsErrorNoServerResponse {
    tag: "OpenBaoListSelfEmailsErrorNoServerResponse"
    error: string
}
export type OpenBaoListSelfEmailsError =
  | OpenBaoListSelfEmailsErrorBadServerResponse
  | OpenBaoListSelfEmailsErrorBadURL
  | OpenBaoListSelfEmailsErrorInternal
  | OpenBaoListSelfEmailsErrorNoServerResponse


// OpenBaoSecretConfig
export interface OpenBaoSecretConfigKV2 {
    tag: "OpenBaoSecretConfigKV2"
    mount_path: string
}
export type OpenBaoSecretConfig =
  | OpenBaoSecretConfigKV2


// OrganizationBootstrapConfig
export interface OrganizationBootstrapConfigSpontaneous {
    tag: "OrganizationBootstrapConfigSpontaneous"
}
export interface OrganizationBootstrapConfigWithBootstrapToken {
    tag: "OrganizationBootstrapConfigWithBootstrapToken"
}
export type OrganizationBootstrapConfig =
  | OrganizationBootstrapConfigSpontaneous
  | OrganizationBootstrapConfigWithBootstrapToken


// OtherShamirRecoveryInfo
export interface OtherShamirRecoveryInfoDeleted {
    tag: "OtherShamirRecoveryInfoDeleted"
    user_id: string
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    deleted_on: number
    deleted_by: string
}
export interface OtherShamirRecoveryInfoSetupAllValid {
    tag: "OtherShamirRecoveryInfoSetupAllValid"
    user_id: string
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
}
export interface OtherShamirRecoveryInfoSetupButUnusable {
    tag: "OtherShamirRecoveryInfoSetupButUnusable"
    user_id: string
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    revoked_recipients: Array<string>
}
export interface OtherShamirRecoveryInfoSetupWithRevokedRecipients {
    tag: "OtherShamirRecoveryInfoSetupWithRevokedRecipients"
    user_id: string
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    revoked_recipients: Array<string>
}
export type OtherShamirRecoveryInfo =
  | OtherShamirRecoveryInfoDeleted
  | OtherShamirRecoveryInfoSetupAllValid
  | OtherShamirRecoveryInfoSetupButUnusable
  | OtherShamirRecoveryInfoSetupWithRevokedRecipients


// PKIInfoItem
export interface PKIInfoItemAccepted {
    tag: "PKIInfoItemAccepted"
    answer: PkiEnrollmentAnswerPayload
    submitted_on: number
    accepted_on: number
}
export interface PKIInfoItemCancelled {
    tag: "PKIInfoItemCancelled"
    submitted_on: number
    cancelled_on: number
}
export interface PKIInfoItemRejected {
    tag: "PKIInfoItemRejected"
    submitted_on: number
    rejected_on: number
}
export interface PKIInfoItemSubmitted {
    tag: "PKIInfoItemSubmitted"
    submitted_on: number
}
export type PKIInfoItem =
  | PKIInfoItemAccepted
  | PKIInfoItemCancelled
  | PKIInfoItemRejected
  | PKIInfoItemSubmitted


// ParseParsecAddrError
export interface ParseParsecAddrErrorInvalidUrl {
    tag: "ParseParsecAddrErrorInvalidUrl"
    error: string
}
export type ParseParsecAddrError =
  | ParseParsecAddrErrorInvalidUrl


// ParsedParsecAddr
export interface ParsedParsecAddrAsyncEnrollment {
    tag: "ParsedParsecAddrAsyncEnrollment"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
    organization_id: string
}
export interface ParsedParsecAddrInvitationDevice {
    tag: "ParsedParsecAddrInvitationDevice"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
    organization_id: string
    token: string
}
export interface ParsedParsecAddrInvitationShamirRecovery {
    tag: "ParsedParsecAddrInvitationShamirRecovery"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
    organization_id: string
    token: string
}
export interface ParsedParsecAddrInvitationUser {
    tag: "ParsedParsecAddrInvitationUser"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
    organization_id: string
    token: string
}
export interface ParsedParsecAddrOrganization {
    tag: "ParsedParsecAddrOrganization"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
    organization_id: string
}
export interface ParsedParsecAddrOrganizationBootstrap {
    tag: "ParsedParsecAddrOrganizationBootstrap"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
    organization_id: string
    token: string | null
}
export interface ParsedParsecAddrPkiEnrollment {
    tag: "ParsedParsecAddrPkiEnrollment"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
    organization_id: string
}
export interface ParsedParsecAddrServer {
    tag: "ParsedParsecAddrServer"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
}
export interface ParsedParsecAddrWorkspacePath {
    tag: "ParsedParsecAddrWorkspacePath"
    hostname: string
    port: number
    is_default_port: boolean
    use_ssl: boolean
    organization_id: string
    workspace_id: string
    key_index: number
    encrypted_path: Uint8Array
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
  | ParsedParsecAddrWorkspacePath


// PendingAsyncEnrollmentInfo
export interface PendingAsyncEnrollmentInfoAccepted {
    tag: "PendingAsyncEnrollmentInfoAccepted"
    submitted_on: number
    accepted_on: number
}
export interface PendingAsyncEnrollmentInfoCancelled {
    tag: "PendingAsyncEnrollmentInfoCancelled"
    submitted_on: number
    cancelled_on: number
}
export interface PendingAsyncEnrollmentInfoRejected {
    tag: "PendingAsyncEnrollmentInfoRejected"
    submitted_on: number
    rejected_on: number
}
export interface PendingAsyncEnrollmentInfoSubmitted {
    tag: "PendingAsyncEnrollmentInfoSubmitted"
    submitted_on: number
}
export type PendingAsyncEnrollmentInfo =
  | PendingAsyncEnrollmentInfoAccepted
  | PendingAsyncEnrollmentInfoCancelled
  | PendingAsyncEnrollmentInfoRejected
  | PendingAsyncEnrollmentInfoSubmitted


// PkiEnrollmentAcceptError
export interface PkiEnrollmentAcceptErrorActiveUsersLimitReached {
    tag: "PkiEnrollmentAcceptErrorActiveUsersLimitReached"
    error: string
}
export interface PkiEnrollmentAcceptErrorAuthorNotAllowed {
    tag: "PkiEnrollmentAcceptErrorAuthorNotAllowed"
    error: string
}
export interface PkiEnrollmentAcceptErrorEnrollmentNoLongerAvailable {
    tag: "PkiEnrollmentAcceptErrorEnrollmentNoLongerAvailable"
    error: string
}
export interface PkiEnrollmentAcceptErrorEnrollmentNotFound {
    tag: "PkiEnrollmentAcceptErrorEnrollmentNotFound"
    error: string
}
export interface PkiEnrollmentAcceptErrorHumanHandleAlreadyTaken {
    tag: "PkiEnrollmentAcceptErrorHumanHandleAlreadyTaken"
    error: string
}
export interface PkiEnrollmentAcceptErrorInternal {
    tag: "PkiEnrollmentAcceptErrorInternal"
    error: string
}
export interface PkiEnrollmentAcceptErrorOffline {
    tag: "PkiEnrollmentAcceptErrorOffline"
    error: string
}
export interface PkiEnrollmentAcceptErrorPkiOperationError {
    tag: "PkiEnrollmentAcceptErrorPkiOperationError"
    error: string
}
export type PkiEnrollmentAcceptError =
  | PkiEnrollmentAcceptErrorActiveUsersLimitReached
  | PkiEnrollmentAcceptErrorAuthorNotAllowed
  | PkiEnrollmentAcceptErrorEnrollmentNoLongerAvailable
  | PkiEnrollmentAcceptErrorEnrollmentNotFound
  | PkiEnrollmentAcceptErrorHumanHandleAlreadyTaken
  | PkiEnrollmentAcceptErrorInternal
  | PkiEnrollmentAcceptErrorOffline
  | PkiEnrollmentAcceptErrorPkiOperationError


// PkiEnrollmentFinalizeError
export interface PkiEnrollmentFinalizeErrorInternal {
    tag: "PkiEnrollmentFinalizeErrorInternal"
    error: string
}
export interface PkiEnrollmentFinalizeErrorSaveError {
    tag: "PkiEnrollmentFinalizeErrorSaveError"
    error: string
}
export type PkiEnrollmentFinalizeError =
  | PkiEnrollmentFinalizeErrorInternal
  | PkiEnrollmentFinalizeErrorSaveError


// PkiEnrollmentInfoError
export interface PkiEnrollmentInfoErrorEnrollmentNotFound {
    tag: "PkiEnrollmentInfoErrorEnrollmentNotFound"
    error: string
}
export interface PkiEnrollmentInfoErrorInternal {
    tag: "PkiEnrollmentInfoErrorInternal"
    error: string
}
export interface PkiEnrollmentInfoErrorInvalidAcceptPayload {
    tag: "PkiEnrollmentInfoErrorInvalidAcceptPayload"
    error: string
}
export interface PkiEnrollmentInfoErrorOffline {
    tag: "PkiEnrollmentInfoErrorOffline"
    error: string
}
export type PkiEnrollmentInfoError =
  | PkiEnrollmentInfoErrorEnrollmentNotFound
  | PkiEnrollmentInfoErrorInternal
  | PkiEnrollmentInfoErrorInvalidAcceptPayload
  | PkiEnrollmentInfoErrorOffline


// PkiEnrollmentListError
export interface PkiEnrollmentListErrorAuthorNotAllowed {
    tag: "PkiEnrollmentListErrorAuthorNotAllowed"
    error: string
}
export interface PkiEnrollmentListErrorInternal {
    tag: "PkiEnrollmentListErrorInternal"
    error: string
}
export interface PkiEnrollmentListErrorOffline {
    tag: "PkiEnrollmentListErrorOffline"
    error: string
}
export type PkiEnrollmentListError =
  | PkiEnrollmentListErrorAuthorNotAllowed
  | PkiEnrollmentListErrorInternal
  | PkiEnrollmentListErrorOffline


// PkiEnrollmentListItem
export interface PkiEnrollmentListItemInvalid {
    tag: "PkiEnrollmentListItemInvalid"
    human_handle: HumanHandle | null
    enrollment_id: string
    submitted_on: number
    reason: InvalidityReason
    details: string
}
export interface PkiEnrollmentListItemValid {
    tag: "PkiEnrollmentListItemValid"
    human_handle: HumanHandle
    enrollment_id: string
    submitted_on: number
    submitter_der_cert: Uint8Array
    payload: PkiEnrollmentSubmitPayload
}
export type PkiEnrollmentListItem =
  | PkiEnrollmentListItemInvalid
  | PkiEnrollmentListItemValid


// PkiEnrollmentRejectError
export interface PkiEnrollmentRejectErrorAuthorNotAllowed {
    tag: "PkiEnrollmentRejectErrorAuthorNotAllowed"
    error: string
}
export interface PkiEnrollmentRejectErrorEnrollmentNoLongerAvailable {
    tag: "PkiEnrollmentRejectErrorEnrollmentNoLongerAvailable"
    error: string
}
export interface PkiEnrollmentRejectErrorEnrollmentNotFound {
    tag: "PkiEnrollmentRejectErrorEnrollmentNotFound"
    error: string
}
export interface PkiEnrollmentRejectErrorInternal {
    tag: "PkiEnrollmentRejectErrorInternal"
    error: string
}
export interface PkiEnrollmentRejectErrorOffline {
    tag: "PkiEnrollmentRejectErrorOffline"
    error: string
}
export type PkiEnrollmentRejectError =
  | PkiEnrollmentRejectErrorAuthorNotAllowed
  | PkiEnrollmentRejectErrorEnrollmentNoLongerAvailable
  | PkiEnrollmentRejectErrorEnrollmentNotFound
  | PkiEnrollmentRejectErrorInternal
  | PkiEnrollmentRejectErrorOffline


// PkiEnrollmentSubmitError
export interface PkiEnrollmentSubmitErrorAlreadyEnrolled {
    tag: "PkiEnrollmentSubmitErrorAlreadyEnrolled"
    error: string
}
export interface PkiEnrollmentSubmitErrorAlreadySubmitted {
    tag: "PkiEnrollmentSubmitErrorAlreadySubmitted"
    error: string
}
export interface PkiEnrollmentSubmitErrorEmailAlreadyUsed {
    tag: "PkiEnrollmentSubmitErrorEmailAlreadyUsed"
    error: string
}
export interface PkiEnrollmentSubmitErrorIdAlreadyUsed {
    tag: "PkiEnrollmentSubmitErrorIdAlreadyUsed"
    error: string
}
export interface PkiEnrollmentSubmitErrorInternal {
    tag: "PkiEnrollmentSubmitErrorInternal"
    error: string
}
export interface PkiEnrollmentSubmitErrorInvalidPayload {
    tag: "PkiEnrollmentSubmitErrorInvalidPayload"
    error: string
}
export interface PkiEnrollmentSubmitErrorOffline {
    tag: "PkiEnrollmentSubmitErrorOffline"
    error: string
}
export interface PkiEnrollmentSubmitErrorPkiOperationError {
    tag: "PkiEnrollmentSubmitErrorPkiOperationError"
    error: string
}
export type PkiEnrollmentSubmitError =
  | PkiEnrollmentSubmitErrorAlreadyEnrolled
  | PkiEnrollmentSubmitErrorAlreadySubmitted
  | PkiEnrollmentSubmitErrorEmailAlreadyUsed
  | PkiEnrollmentSubmitErrorIdAlreadyUsed
  | PkiEnrollmentSubmitErrorInternal
  | PkiEnrollmentSubmitErrorInvalidPayload
  | PkiEnrollmentSubmitErrorOffline
  | PkiEnrollmentSubmitErrorPkiOperationError


// PkiGetAddrError
export interface PkiGetAddrErrorInternal {
    tag: "PkiGetAddrErrorInternal"
    error: string
}
export type PkiGetAddrError =
  | PkiGetAddrErrorInternal


// RemoveDeviceDataError
export interface RemoveDeviceDataErrorFailedToRemoveData {
    tag: "RemoveDeviceDataErrorFailedToRemoveData"
    error: string
}
export type RemoveDeviceDataError =
  | RemoveDeviceDataErrorFailedToRemoveData


// RemoveDeviceError
export interface RemoveDeviceErrorInternal {
    tag: "RemoveDeviceErrorInternal"
    error: string
}
export interface RemoveDeviceErrorNotFound {
    tag: "RemoveDeviceErrorNotFound"
    error: string
}
export interface RemoveDeviceErrorStorageNotAvailable {
    tag: "RemoveDeviceErrorStorageNotAvailable"
    error: string
}
export type RemoveDeviceError =
  | RemoveDeviceErrorInternal
  | RemoveDeviceErrorNotFound
  | RemoveDeviceErrorStorageNotAvailable


// SelfShamirRecoveryInfo
export interface SelfShamirRecoveryInfoDeleted {
    tag: "SelfShamirRecoveryInfoDeleted"
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    deleted_on: number
    deleted_by: string
}
export interface SelfShamirRecoveryInfoNeverSetup {
    tag: "SelfShamirRecoveryInfoNeverSetup"
}
export interface SelfShamirRecoveryInfoSetupAllValid {
    tag: "SelfShamirRecoveryInfoSetupAllValid"
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
}
export interface SelfShamirRecoveryInfoSetupButUnusable {
    tag: "SelfShamirRecoveryInfoSetupButUnusable"
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    revoked_recipients: Array<string>
}
export interface SelfShamirRecoveryInfoSetupWithRevokedRecipients {
    tag: "SelfShamirRecoveryInfoSetupWithRevokedRecipients"
    created_on: number
    created_by: string
    threshold: number
    per_recipient_shares: Map<string, number>
    revoked_recipients: Array<string>
}
export type SelfShamirRecoveryInfo =
  | SelfShamirRecoveryInfoDeleted
  | SelfShamirRecoveryInfoNeverSetup
  | SelfShamirRecoveryInfoSetupAllValid
  | SelfShamirRecoveryInfoSetupButUnusable
  | SelfShamirRecoveryInfoSetupWithRevokedRecipients


// ShamirRecoveryClaimAddShareError
export interface ShamirRecoveryClaimAddShareErrorCorruptedSecret {
    tag: "ShamirRecoveryClaimAddShareErrorCorruptedSecret"
    error: string
}
export interface ShamirRecoveryClaimAddShareErrorInternal {
    tag: "ShamirRecoveryClaimAddShareErrorInternal"
    error: string
}
export interface ShamirRecoveryClaimAddShareErrorRecipientNotFound {
    tag: "ShamirRecoveryClaimAddShareErrorRecipientNotFound"
    error: string
}
export type ShamirRecoveryClaimAddShareError =
  | ShamirRecoveryClaimAddShareErrorCorruptedSecret
  | ShamirRecoveryClaimAddShareErrorInternal
  | ShamirRecoveryClaimAddShareErrorRecipientNotFound


// ShamirRecoveryClaimMaybeFinalizeInfo
export interface ShamirRecoveryClaimMaybeFinalizeInfoFinalize {
    tag: "ShamirRecoveryClaimMaybeFinalizeInfoFinalize"
    handle: number
}
export interface ShamirRecoveryClaimMaybeFinalizeInfoOffline {
    tag: "ShamirRecoveryClaimMaybeFinalizeInfoOffline"
    handle: number
}
export type ShamirRecoveryClaimMaybeFinalizeInfo =
  | ShamirRecoveryClaimMaybeFinalizeInfoFinalize
  | ShamirRecoveryClaimMaybeFinalizeInfoOffline


// ShamirRecoveryClaimMaybeRecoverDeviceInfo
export interface ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient {
    tag: "ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient"
    handle: number
    claimer_user_id: string
    claimer_human_handle: HumanHandle
    shamir_recovery_created_on: number
    recipients: Array<ShamirRecoveryRecipient>
    threshold: number
    recovered_shares: Map<string, number>
    is_recoverable: boolean
}
export interface ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice {
    tag: "ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice"
    handle: number
    claimer_user_id: string
    claimer_human_handle: HumanHandle
}
export type ShamirRecoveryClaimMaybeRecoverDeviceInfo =
  | ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient
  | ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice


// ShamirRecoveryClaimPickRecipientError
export interface ShamirRecoveryClaimPickRecipientErrorInternal {
    tag: "ShamirRecoveryClaimPickRecipientErrorInternal"
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked {
    tag: "ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked"
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientNotFound {
    tag: "ShamirRecoveryClaimPickRecipientErrorRecipientNotFound"
    error: string
}
export interface ShamirRecoveryClaimPickRecipientErrorRecipientRevoked {
    tag: "ShamirRecoveryClaimPickRecipientErrorRecipientRevoked"
    error: string
}
export type ShamirRecoveryClaimPickRecipientError =
  | ShamirRecoveryClaimPickRecipientErrorInternal
  | ShamirRecoveryClaimPickRecipientErrorRecipientAlreadyPicked
  | ShamirRecoveryClaimPickRecipientErrorRecipientNotFound
  | ShamirRecoveryClaimPickRecipientErrorRecipientRevoked


// ShamirRecoveryClaimRecoverDeviceError
export interface ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed {
    tag: "ShamirRecoveryClaimRecoverDeviceErrorAlreadyUsed"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound {
    tag: "ShamirRecoveryClaimRecoverDeviceErrorCipheredDataNotFound"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData {
    tag: "ShamirRecoveryClaimRecoverDeviceErrorCorruptedCipheredData"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorInternal {
    tag: "ShamirRecoveryClaimRecoverDeviceErrorInternal"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorNotFound {
    tag: "ShamirRecoveryClaimRecoverDeviceErrorNotFound"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired {
    tag: "ShamirRecoveryClaimRecoverDeviceErrorOrganizationExpired"
    error: string
}
export interface ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError {
    tag: "ShamirRecoveryClaimRecoverDeviceErrorRegisterNewDeviceError"
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
export interface ShowCertificateSelectionDialogErrorCannotGetCertificateInfo {
    tag: "ShowCertificateSelectionDialogErrorCannotGetCertificateInfo"
    error: string
}
export interface ShowCertificateSelectionDialogErrorCannotOpenStore {
    tag: "ShowCertificateSelectionDialogErrorCannotOpenStore"
    error: string
}
export type ShowCertificateSelectionDialogError =
  | ShowCertificateSelectionDialogErrorCannotGetCertificateInfo
  | ShowCertificateSelectionDialogErrorCannotOpenStore


// SubmitAsyncEnrollmentError
export interface SubmitAsyncEnrollmentErrorEmailAlreadyEnrolled {
    tag: "SubmitAsyncEnrollmentErrorEmailAlreadyEnrolled"
    error: string
}
export interface SubmitAsyncEnrollmentErrorEmailAlreadySubmitted {
    tag: "SubmitAsyncEnrollmentErrorEmailAlreadySubmitted"
    error: string
}
export interface SubmitAsyncEnrollmentErrorInternal {
    tag: "SubmitAsyncEnrollmentErrorInternal"
    error: string
}
export interface SubmitAsyncEnrollmentErrorInvalidPath {
    tag: "SubmitAsyncEnrollmentErrorInvalidPath"
    error: string
}
export interface SubmitAsyncEnrollmentErrorOffline {
    tag: "SubmitAsyncEnrollmentErrorOffline"
    error: string
}
export interface SubmitAsyncEnrollmentErrorOpenBaoBadServerResponse {
    tag: "SubmitAsyncEnrollmentErrorOpenBaoBadServerResponse"
    error: string
}
export interface SubmitAsyncEnrollmentErrorOpenBaoBadURL {
    tag: "SubmitAsyncEnrollmentErrorOpenBaoBadURL"
    error: string
}
export interface SubmitAsyncEnrollmentErrorOpenBaoNoServerResponse {
    tag: "SubmitAsyncEnrollmentErrorOpenBaoNoServerResponse"
    error: string
}
export interface SubmitAsyncEnrollmentErrorPKICannotOpenCertificateStore {
    tag: "SubmitAsyncEnrollmentErrorPKICannotOpenCertificateStore"
    error: string
}
export interface SubmitAsyncEnrollmentErrorPKIServerInvalidX509Trustchain {
    tag: "SubmitAsyncEnrollmentErrorPKIServerInvalidX509Trustchain"
    error: string
}
export interface SubmitAsyncEnrollmentErrorPKIUnusableX509CertificateReference {
    tag: "SubmitAsyncEnrollmentErrorPKIUnusableX509CertificateReference"
    error: string
}
export interface SubmitAsyncEnrollmentErrorStorageNotAvailable {
    tag: "SubmitAsyncEnrollmentErrorStorageNotAvailable"
    error: string
}
export type SubmitAsyncEnrollmentError =
  | SubmitAsyncEnrollmentErrorEmailAlreadyEnrolled
  | SubmitAsyncEnrollmentErrorEmailAlreadySubmitted
  | SubmitAsyncEnrollmentErrorInternal
  | SubmitAsyncEnrollmentErrorInvalidPath
  | SubmitAsyncEnrollmentErrorOffline
  | SubmitAsyncEnrollmentErrorOpenBaoBadServerResponse
  | SubmitAsyncEnrollmentErrorOpenBaoBadURL
  | SubmitAsyncEnrollmentErrorOpenBaoNoServerResponse
  | SubmitAsyncEnrollmentErrorPKICannotOpenCertificateStore
  | SubmitAsyncEnrollmentErrorPKIServerInvalidX509Trustchain
  | SubmitAsyncEnrollmentErrorPKIUnusableX509CertificateReference
  | SubmitAsyncEnrollmentErrorStorageNotAvailable


// SubmitAsyncEnrollmentIdentityStrategy
export interface SubmitAsyncEnrollmentIdentityStrategyOpenBao {
    tag: "SubmitAsyncEnrollmentIdentityStrategyOpenBao"
    requested_human_handle: HumanHandle
    openbao_server_url: string
    openbao_transit_mount_path: string
    openbao_secret_mount_path: string
    openbao_entity_id: string
    openbao_auth_token: string
    openbao_preferred_auth_id: string
}
export interface SubmitAsyncEnrollmentIdentityStrategyPKI {
    tag: "SubmitAsyncEnrollmentIdentityStrategyPKI"
    certificate_reference: X509CertificateReference
}
export type SubmitAsyncEnrollmentIdentityStrategy =
  | SubmitAsyncEnrollmentIdentityStrategyOpenBao
  | SubmitAsyncEnrollmentIdentityStrategyPKI


// SubmitterFinalizeAsyncEnrollmentError
export interface SubmitterFinalizeAsyncEnrollmentErrorBadAcceptPayload {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorBadAcceptPayload"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileCannotRetrieveCiphertextKey {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileCannotRetrieveCiphertextKey"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidData {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidData"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidPath {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorEnrollmentFileInvalidPath"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorEnrollmentNotFoundOnServer {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorEnrollmentNotFoundOnServer"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorIdentityStrategyMismatch {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorIdentityStrategyMismatch"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorInternal {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorInternal"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorNotAccepted {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorNotAccepted"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorOffline {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorOffline"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadServerResponse {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadServerResponse"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadURL {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorOpenBaoBadURL"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorOpenBaoNoServerResponse {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorOpenBaoNoServerResponse"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorPKICannotOpenCertificateStore {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorPKICannotOpenCertificateStore"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorPKIUnusableX509CertificateReference {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorPKIUnusableX509CertificateReference"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceInvalidPath {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceInvalidPath"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadFailed {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadFailed"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadOffline {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorSaveDeviceRemoteOpaqueKeyUploadOffline"
    error: string
}
export interface SubmitterFinalizeAsyncEnrollmentErrorStorageNotAvailable {
    tag: "SubmitterFinalizeAsyncEnrollmentErrorStorageNotAvailable"
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
  | SubmitterFinalizeAsyncEnrollmentErrorStorageNotAvailable


// SubmitterForgetAsyncEnrollmentError
export interface SubmitterForgetAsyncEnrollmentErrorInternal {
    tag: "SubmitterForgetAsyncEnrollmentErrorInternal"
    error: string
}
export interface SubmitterForgetAsyncEnrollmentErrorNotFound {
    tag: "SubmitterForgetAsyncEnrollmentErrorNotFound"
    error: string
}
export interface SubmitterForgetAsyncEnrollmentErrorStorageNotAvailable {
    tag: "SubmitterForgetAsyncEnrollmentErrorStorageNotAvailable"
    error: string
}
export type SubmitterForgetAsyncEnrollmentError =
  | SubmitterForgetAsyncEnrollmentErrorInternal
  | SubmitterForgetAsyncEnrollmentErrorNotFound
  | SubmitterForgetAsyncEnrollmentErrorStorageNotAvailable


// SubmitterGetAsyncEnrollmentInfoError
export interface SubmitterGetAsyncEnrollmentInfoErrorEnrollmentNotFound {
    tag: "SubmitterGetAsyncEnrollmentInfoErrorEnrollmentNotFound"
    error: string
}
export interface SubmitterGetAsyncEnrollmentInfoErrorInternal {
    tag: "SubmitterGetAsyncEnrollmentInfoErrorInternal"
    error: string
}
export interface SubmitterGetAsyncEnrollmentInfoErrorOffline {
    tag: "SubmitterGetAsyncEnrollmentInfoErrorOffline"
    error: string
}
export type SubmitterGetAsyncEnrollmentInfoError =
  | SubmitterGetAsyncEnrollmentInfoErrorEnrollmentNotFound
  | SubmitterGetAsyncEnrollmentInfoErrorInternal
  | SubmitterGetAsyncEnrollmentInfoErrorOffline


// SubmitterListLocalAsyncEnrollmentsError
export interface SubmitterListLocalAsyncEnrollmentsErrorInternal {
    tag: "SubmitterListLocalAsyncEnrollmentsErrorInternal"
    error: string
}
export interface SubmitterListLocalAsyncEnrollmentsErrorStorageNotAvailable {
    tag: "SubmitterListLocalAsyncEnrollmentsErrorStorageNotAvailable"
    error: string
}
export type SubmitterListLocalAsyncEnrollmentsError =
  | SubmitterListLocalAsyncEnrollmentsErrorInternal
  | SubmitterListLocalAsyncEnrollmentsErrorStorageNotAvailable


// TestbedError
export interface TestbedErrorDisabled {
    tag: "TestbedErrorDisabled"
    error: string
}
export interface TestbedErrorInternal {
    tag: "TestbedErrorInternal"
    error: string
}
export type TestbedError =
  | TestbedErrorDisabled
  | TestbedErrorInternal


// UpdateDeviceError
export interface UpdateDeviceErrorDecryptionFailed {
    tag: "UpdateDeviceErrorDecryptionFailed"
    error: string
}
export interface UpdateDeviceErrorInternal {
    tag: "UpdateDeviceErrorInternal"
    error: string
}
export interface UpdateDeviceErrorInvalidData {
    tag: "UpdateDeviceErrorInvalidData"
    error: string
}
export interface UpdateDeviceErrorInvalidPath {
    tag: "UpdateDeviceErrorInvalidPath"
    error: string
}
export interface UpdateDeviceErrorRemoteOpaqueKeyOperationFailed {
    tag: "UpdateDeviceErrorRemoteOpaqueKeyOperationFailed"
    error: string
}
export interface UpdateDeviceErrorRemoteOpaqueKeyOperationOffline {
    tag: "UpdateDeviceErrorRemoteOpaqueKeyOperationOffline"
    error: string
}
export interface UpdateDeviceErrorStorageNotAvailable {
    tag: "UpdateDeviceErrorStorageNotAvailable"
    error: string
}
export type UpdateDeviceError =
  | UpdateDeviceErrorDecryptionFailed
  | UpdateDeviceErrorInternal
  | UpdateDeviceErrorInvalidData
  | UpdateDeviceErrorInvalidPath
  | UpdateDeviceErrorRemoteOpaqueKeyOperationFailed
  | UpdateDeviceErrorRemoteOpaqueKeyOperationOffline
  | UpdateDeviceErrorStorageNotAvailable


// UserClaimListInitialInfosError
export interface UserClaimListInitialInfosErrorInternal {
    tag: "UserClaimListInitialInfosErrorInternal"
    error: string
}
export type UserClaimListInitialInfosError =
  | UserClaimListInitialInfosErrorInternal


// WaitForDeviceAvailableError
export interface WaitForDeviceAvailableErrorInternal {
    tag: "WaitForDeviceAvailableErrorInternal"
    error: string
}
export type WaitForDeviceAvailableError =
  | WaitForDeviceAvailableErrorInternal


// WorkspaceCreateFileError
export interface WorkspaceCreateFileErrorEntryExists {
    tag: "WorkspaceCreateFileErrorEntryExists"
    error: string
}
export interface WorkspaceCreateFileErrorInternal {
    tag: "WorkspaceCreateFileErrorInternal"
    error: string
}
export interface WorkspaceCreateFileErrorInvalidCertificate {
    tag: "WorkspaceCreateFileErrorInvalidCertificate"
    error: string
}
export interface WorkspaceCreateFileErrorInvalidKeysBundle {
    tag: "WorkspaceCreateFileErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceCreateFileErrorInvalidManifest {
    tag: "WorkspaceCreateFileErrorInvalidManifest"
    error: string
}
export interface WorkspaceCreateFileErrorNoRealmAccess {
    tag: "WorkspaceCreateFileErrorNoRealmAccess"
    error: string
}
export interface WorkspaceCreateFileErrorOffline {
    tag: "WorkspaceCreateFileErrorOffline"
    error: string
}
export interface WorkspaceCreateFileErrorParentNotAFolder {
    tag: "WorkspaceCreateFileErrorParentNotAFolder"
    error: string
}
export interface WorkspaceCreateFileErrorParentNotFound {
    tag: "WorkspaceCreateFileErrorParentNotFound"
    error: string
}
export interface WorkspaceCreateFileErrorReadOnlyRealm {
    tag: "WorkspaceCreateFileErrorReadOnlyRealm"
    error: string
}
export interface WorkspaceCreateFileErrorStopped {
    tag: "WorkspaceCreateFileErrorStopped"
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
export interface WorkspaceCreateFolderErrorEntryExists {
    tag: "WorkspaceCreateFolderErrorEntryExists"
    error: string
}
export interface WorkspaceCreateFolderErrorInternal {
    tag: "WorkspaceCreateFolderErrorInternal"
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidCertificate {
    tag: "WorkspaceCreateFolderErrorInvalidCertificate"
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidKeysBundle {
    tag: "WorkspaceCreateFolderErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceCreateFolderErrorInvalidManifest {
    tag: "WorkspaceCreateFolderErrorInvalidManifest"
    error: string
}
export interface WorkspaceCreateFolderErrorNoRealmAccess {
    tag: "WorkspaceCreateFolderErrorNoRealmAccess"
    error: string
}
export interface WorkspaceCreateFolderErrorOffline {
    tag: "WorkspaceCreateFolderErrorOffline"
    error: string
}
export interface WorkspaceCreateFolderErrorParentNotAFolder {
    tag: "WorkspaceCreateFolderErrorParentNotAFolder"
    error: string
}
export interface WorkspaceCreateFolderErrorParentNotFound {
    tag: "WorkspaceCreateFolderErrorParentNotFound"
    error: string
}
export interface WorkspaceCreateFolderErrorReadOnlyRealm {
    tag: "WorkspaceCreateFolderErrorReadOnlyRealm"
    error: string
}
export interface WorkspaceCreateFolderErrorStopped {
    tag: "WorkspaceCreateFolderErrorStopped"
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
export interface WorkspaceDecryptPathAddrErrorCorruptedData {
    tag: "WorkspaceDecryptPathAddrErrorCorruptedData"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorCorruptedKey {
    tag: "WorkspaceDecryptPathAddrErrorCorruptedKey"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInternal {
    tag: "WorkspaceDecryptPathAddrErrorInternal"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInvalidCertificate {
    tag: "WorkspaceDecryptPathAddrErrorInvalidCertificate"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorInvalidKeysBundle {
    tag: "WorkspaceDecryptPathAddrErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorKeyNotFound {
    tag: "WorkspaceDecryptPathAddrErrorKeyNotFound"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorNotAllowed {
    tag: "WorkspaceDecryptPathAddrErrorNotAllowed"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorOffline {
    tag: "WorkspaceDecryptPathAddrErrorOffline"
    error: string
}
export interface WorkspaceDecryptPathAddrErrorStopped {
    tag: "WorkspaceDecryptPathAddrErrorStopped"
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
export interface WorkspaceFdCloseErrorBadFileDescriptor {
    tag: "WorkspaceFdCloseErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceFdCloseErrorInternal {
    tag: "WorkspaceFdCloseErrorInternal"
    error: string
}
export interface WorkspaceFdCloseErrorStopped {
    tag: "WorkspaceFdCloseErrorStopped"
    error: string
}
export type WorkspaceFdCloseError =
  | WorkspaceFdCloseErrorBadFileDescriptor
  | WorkspaceFdCloseErrorInternal
  | WorkspaceFdCloseErrorStopped


// WorkspaceFdFlushError
export interface WorkspaceFdFlushErrorBadFileDescriptor {
    tag: "WorkspaceFdFlushErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceFdFlushErrorInternal {
    tag: "WorkspaceFdFlushErrorInternal"
    error: string
}
export interface WorkspaceFdFlushErrorNotInWriteMode {
    tag: "WorkspaceFdFlushErrorNotInWriteMode"
    error: string
}
export interface WorkspaceFdFlushErrorStopped {
    tag: "WorkspaceFdFlushErrorStopped"
    error: string
}
export type WorkspaceFdFlushError =
  | WorkspaceFdFlushErrorBadFileDescriptor
  | WorkspaceFdFlushErrorInternal
  | WorkspaceFdFlushErrorNotInWriteMode
  | WorkspaceFdFlushErrorStopped


// WorkspaceFdReadError
export interface WorkspaceFdReadErrorBadFileDescriptor {
    tag: "WorkspaceFdReadErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceFdReadErrorInternal {
    tag: "WorkspaceFdReadErrorInternal"
    error: string
}
export interface WorkspaceFdReadErrorInvalidBlockAccess {
    tag: "WorkspaceFdReadErrorInvalidBlockAccess"
    error: string
}
export interface WorkspaceFdReadErrorInvalidCertificate {
    tag: "WorkspaceFdReadErrorInvalidCertificate"
    error: string
}
export interface WorkspaceFdReadErrorInvalidKeysBundle {
    tag: "WorkspaceFdReadErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceFdReadErrorNoRealmAccess {
    tag: "WorkspaceFdReadErrorNoRealmAccess"
    error: string
}
export interface WorkspaceFdReadErrorNotInReadMode {
    tag: "WorkspaceFdReadErrorNotInReadMode"
    error: string
}
export interface WorkspaceFdReadErrorOffline {
    tag: "WorkspaceFdReadErrorOffline"
    error: string
}
export interface WorkspaceFdReadErrorServerBlockstoreUnavailable {
    tag: "WorkspaceFdReadErrorServerBlockstoreUnavailable"
    error: string
}
export interface WorkspaceFdReadErrorStopped {
    tag: "WorkspaceFdReadErrorStopped"
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
export interface WorkspaceFdResizeErrorBadFileDescriptor {
    tag: "WorkspaceFdResizeErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceFdResizeErrorInternal {
    tag: "WorkspaceFdResizeErrorInternal"
    error: string
}
export interface WorkspaceFdResizeErrorNotInWriteMode {
    tag: "WorkspaceFdResizeErrorNotInWriteMode"
    error: string
}
export type WorkspaceFdResizeError =
  | WorkspaceFdResizeErrorBadFileDescriptor
  | WorkspaceFdResizeErrorInternal
  | WorkspaceFdResizeErrorNotInWriteMode


// WorkspaceFdStatError
export interface WorkspaceFdStatErrorBadFileDescriptor {
    tag: "WorkspaceFdStatErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceFdStatErrorInternal {
    tag: "WorkspaceFdStatErrorInternal"
    error: string
}
export type WorkspaceFdStatError =
  | WorkspaceFdStatErrorBadFileDescriptor
  | WorkspaceFdStatErrorInternal


// WorkspaceFdWriteError
export interface WorkspaceFdWriteErrorBadFileDescriptor {
    tag: "WorkspaceFdWriteErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceFdWriteErrorInternal {
    tag: "WorkspaceFdWriteErrorInternal"
    error: string
}
export interface WorkspaceFdWriteErrorNotInWriteMode {
    tag: "WorkspaceFdWriteErrorNotInWriteMode"
    error: string
}
export type WorkspaceFdWriteError =
  | WorkspaceFdWriteErrorBadFileDescriptor
  | WorkspaceFdWriteErrorInternal
  | WorkspaceFdWriteErrorNotInWriteMode


// WorkspaceGeneratePathAddrError
export interface WorkspaceGeneratePathAddrErrorInternal {
    tag: "WorkspaceGeneratePathAddrErrorInternal"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorInvalidKeysBundle {
    tag: "WorkspaceGeneratePathAddrErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorNoKey {
    tag: "WorkspaceGeneratePathAddrErrorNoKey"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorNotAllowed {
    tag: "WorkspaceGeneratePathAddrErrorNotAllowed"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorOffline {
    tag: "WorkspaceGeneratePathAddrErrorOffline"
    error: string
}
export interface WorkspaceGeneratePathAddrErrorStopped {
    tag: "WorkspaceGeneratePathAddrErrorStopped"
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
export interface WorkspaceHistoryEntryStatFile {
    tag: "WorkspaceHistoryEntryStatFile"
    id: string
    parent: string
    created: number
    updated: number
    version: number
    size: number
    last_updater: string
}
export interface WorkspaceHistoryEntryStatFolder {
    tag: "WorkspaceHistoryEntryStatFolder"
    id: string
    parent: string
    created: number
    updated: number
    version: number
    last_updater: string
}
export type WorkspaceHistoryEntryStat =
  | WorkspaceHistoryEntryStatFile
  | WorkspaceHistoryEntryStatFolder


// WorkspaceHistoryFdCloseError
export interface WorkspaceHistoryFdCloseErrorBadFileDescriptor {
    tag: "WorkspaceHistoryFdCloseErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceHistoryFdCloseErrorInternal {
    tag: "WorkspaceHistoryFdCloseErrorInternal"
    error: string
}
export type WorkspaceHistoryFdCloseError =
  | WorkspaceHistoryFdCloseErrorBadFileDescriptor
  | WorkspaceHistoryFdCloseErrorInternal


// WorkspaceHistoryFdReadError
export interface WorkspaceHistoryFdReadErrorBadFileDescriptor {
    tag: "WorkspaceHistoryFdReadErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceHistoryFdReadErrorInternal {
    tag: "WorkspaceHistoryFdReadErrorInternal"
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidBlockAccess {
    tag: "WorkspaceHistoryFdReadErrorInvalidBlockAccess"
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidCertificate {
    tag: "WorkspaceHistoryFdReadErrorInvalidCertificate"
    error: string
}
export interface WorkspaceHistoryFdReadErrorInvalidKeysBundle {
    tag: "WorkspaceHistoryFdReadErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryFdReadErrorNoRealmAccess {
    tag: "WorkspaceHistoryFdReadErrorNoRealmAccess"
    error: string
}
export interface WorkspaceHistoryFdReadErrorOffline {
    tag: "WorkspaceHistoryFdReadErrorOffline"
    error: string
}
export interface WorkspaceHistoryFdReadErrorServerBlockstoreUnavailable {
    tag: "WorkspaceHistoryFdReadErrorServerBlockstoreUnavailable"
    error: string
}
export interface WorkspaceHistoryFdReadErrorStopped {
    tag: "WorkspaceHistoryFdReadErrorStopped"
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
export interface WorkspaceHistoryFdStatErrorBadFileDescriptor {
    tag: "WorkspaceHistoryFdStatErrorBadFileDescriptor"
    error: string
}
export interface WorkspaceHistoryFdStatErrorInternal {
    tag: "WorkspaceHistoryFdStatErrorInternal"
    error: string
}
export type WorkspaceHistoryFdStatError =
  | WorkspaceHistoryFdStatErrorBadFileDescriptor
  | WorkspaceHistoryFdStatErrorInternal


// WorkspaceHistoryInternalOnlyError
export interface WorkspaceHistoryInternalOnlyErrorInternal {
    tag: "WorkspaceHistoryInternalOnlyErrorInternal"
    error: string
}
export type WorkspaceHistoryInternalOnlyError =
  | WorkspaceHistoryInternalOnlyErrorInternal


// WorkspaceHistoryOpenFileError
export interface WorkspaceHistoryOpenFileErrorEntryNotAFile {
    tag: "WorkspaceHistoryOpenFileErrorEntryNotAFile"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorEntryNotFound {
    tag: "WorkspaceHistoryOpenFileErrorEntryNotFound"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInternal {
    tag: "WorkspaceHistoryOpenFileErrorInternal"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidCertificate {
    tag: "WorkspaceHistoryOpenFileErrorInvalidCertificate"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidHistory {
    tag: "WorkspaceHistoryOpenFileErrorInvalidHistory"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidKeysBundle {
    tag: "WorkspaceHistoryOpenFileErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorInvalidManifest {
    tag: "WorkspaceHistoryOpenFileErrorInvalidManifest"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorNoRealmAccess {
    tag: "WorkspaceHistoryOpenFileErrorNoRealmAccess"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorOffline {
    tag: "WorkspaceHistoryOpenFileErrorOffline"
    error: string
}
export interface WorkspaceHistoryOpenFileErrorStopped {
    tag: "WorkspaceHistoryOpenFileErrorStopped"
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
export interface WorkspaceHistoryRealmExportDecryptorSequesterService {
    tag: "WorkspaceHistoryRealmExportDecryptorSequesterService"
    sequester_service_id: string
    private_key_pem_path: string
}
export interface WorkspaceHistoryRealmExportDecryptorUser {
    tag: "WorkspaceHistoryRealmExportDecryptorUser"
    access: DeviceAccessStrategy
}
export type WorkspaceHistoryRealmExportDecryptor =
  | WorkspaceHistoryRealmExportDecryptorSequesterService
  | WorkspaceHistoryRealmExportDecryptorUser


// WorkspaceHistorySetTimestampOfInterestError
export interface WorkspaceHistorySetTimestampOfInterestErrorEntryNotFound {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorEntryNotFound"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInternal {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorInternal"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInvalidCertificate {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorInvalidCertificate"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInvalidHistory {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorInvalidHistory"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInvalidKeysBundle {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorInvalidManifest {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorInvalidManifest"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorNewerThanHigherBound {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorNewerThanHigherBound"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorNoRealmAccess {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorNoRealmAccess"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorOffline {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorOffline"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorOlderThanLowerBound {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorOlderThanLowerBound"
    error: string
}
export interface WorkspaceHistorySetTimestampOfInterestErrorStopped {
    tag: "WorkspaceHistorySetTimestampOfInterestErrorStopped"
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
export interface WorkspaceHistoryStartErrorCannotOpenRealmExportDatabase {
    tag: "WorkspaceHistoryStartErrorCannotOpenRealmExportDatabase"
    error: string
}
export interface WorkspaceHistoryStartErrorIncompleteRealmExportDatabase {
    tag: "WorkspaceHistoryStartErrorIncompleteRealmExportDatabase"
    error: string
}
export interface WorkspaceHistoryStartErrorInternal {
    tag: "WorkspaceHistoryStartErrorInternal"
    error: string
}
export interface WorkspaceHistoryStartErrorInvalidCertificate {
    tag: "WorkspaceHistoryStartErrorInvalidCertificate"
    error: string
}
export interface WorkspaceHistoryStartErrorInvalidKeysBundle {
    tag: "WorkspaceHistoryStartErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryStartErrorInvalidManifest {
    tag: "WorkspaceHistoryStartErrorInvalidManifest"
    error: string
}
export interface WorkspaceHistoryStartErrorInvalidRealmExportDatabase {
    tag: "WorkspaceHistoryStartErrorInvalidRealmExportDatabase"
    error: string
}
export interface WorkspaceHistoryStartErrorNoHistory {
    tag: "WorkspaceHistoryStartErrorNoHistory"
    error: string
}
export interface WorkspaceHistoryStartErrorNoRealmAccess {
    tag: "WorkspaceHistoryStartErrorNoRealmAccess"
    error: string
}
export interface WorkspaceHistoryStartErrorOffline {
    tag: "WorkspaceHistoryStartErrorOffline"
    error: string
}
export interface WorkspaceHistoryStartErrorStopped {
    tag: "WorkspaceHistoryStartErrorStopped"
    error: string
}
export interface WorkspaceHistoryStartErrorUnsupportedRealmExportDatabaseVersion {
    tag: "WorkspaceHistoryStartErrorUnsupportedRealmExportDatabaseVersion"
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
export interface WorkspaceHistoryStatEntryErrorEntryNotFound {
    tag: "WorkspaceHistoryStatEntryErrorEntryNotFound"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInternal {
    tag: "WorkspaceHistoryStatEntryErrorInternal"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidCertificate {
    tag: "WorkspaceHistoryStatEntryErrorInvalidCertificate"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidHistory {
    tag: "WorkspaceHistoryStatEntryErrorInvalidHistory"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidKeysBundle {
    tag: "WorkspaceHistoryStatEntryErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorInvalidManifest {
    tag: "WorkspaceHistoryStatEntryErrorInvalidManifest"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorNoRealmAccess {
    tag: "WorkspaceHistoryStatEntryErrorNoRealmAccess"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorOffline {
    tag: "WorkspaceHistoryStatEntryErrorOffline"
    error: string
}
export interface WorkspaceHistoryStatEntryErrorStopped {
    tag: "WorkspaceHistoryStatEntryErrorStopped"
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
export interface WorkspaceHistoryStatFolderChildrenErrorEntryIsFile {
    tag: "WorkspaceHistoryStatFolderChildrenErrorEntryIsFile"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorEntryNotFound {
    tag: "WorkspaceHistoryStatFolderChildrenErrorEntryNotFound"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInternal {
    tag: "WorkspaceHistoryStatFolderChildrenErrorInternal"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate {
    tag: "WorkspaceHistoryStatFolderChildrenErrorInvalidCertificate"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidHistory {
    tag: "WorkspaceHistoryStatFolderChildrenErrorInvalidHistory"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle {
    tag: "WorkspaceHistoryStatFolderChildrenErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorInvalidManifest {
    tag: "WorkspaceHistoryStatFolderChildrenErrorInvalidManifest"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess {
    tag: "WorkspaceHistoryStatFolderChildrenErrorNoRealmAccess"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorOffline {
    tag: "WorkspaceHistoryStatFolderChildrenErrorOffline"
    error: string
}
export interface WorkspaceHistoryStatFolderChildrenErrorStopped {
    tag: "WorkspaceHistoryStatFolderChildrenErrorStopped"
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
export interface WorkspaceInfoErrorInternal {
    tag: "WorkspaceInfoErrorInternal"
    error: string
}
export type WorkspaceInfoError =
  | WorkspaceInfoErrorInternal


// WorkspaceIsFileContentLocalError
export interface WorkspaceIsFileContentLocalErrorEntryNotFound {
    tag: "WorkspaceIsFileContentLocalErrorEntryNotFound"
    error: string
}
export interface WorkspaceIsFileContentLocalErrorInternal {
    tag: "WorkspaceIsFileContentLocalErrorInternal"
    error: string
}
export interface WorkspaceIsFileContentLocalErrorInvalidCertificate {
    tag: "WorkspaceIsFileContentLocalErrorInvalidCertificate"
    error: string
}
export interface WorkspaceIsFileContentLocalErrorInvalidKeysBundle {
    tag: "WorkspaceIsFileContentLocalErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceIsFileContentLocalErrorInvalidManifest {
    tag: "WorkspaceIsFileContentLocalErrorInvalidManifest"
    error: string
}
export interface WorkspaceIsFileContentLocalErrorNoRealmAccess {
    tag: "WorkspaceIsFileContentLocalErrorNoRealmAccess"
    error: string
}
export interface WorkspaceIsFileContentLocalErrorNotAFile {
    tag: "WorkspaceIsFileContentLocalErrorNotAFile"
    error: string
}
export interface WorkspaceIsFileContentLocalErrorOffline {
    tag: "WorkspaceIsFileContentLocalErrorOffline"
    error: string
}
export interface WorkspaceIsFileContentLocalErrorStopped {
    tag: "WorkspaceIsFileContentLocalErrorStopped"
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
export interface WorkspaceMountErrorDisabled {
    tag: "WorkspaceMountErrorDisabled"
    error: string
}
export interface WorkspaceMountErrorInternal {
    tag: "WorkspaceMountErrorInternal"
    error: string
}
export type WorkspaceMountError =
  | WorkspaceMountErrorDisabled
  | WorkspaceMountErrorInternal


// WorkspaceMoveEntryError
export interface WorkspaceMoveEntryErrorCannotMoveRoot {
    tag: "WorkspaceMoveEntryErrorCannotMoveRoot"
    error: string
}
export interface WorkspaceMoveEntryErrorDestinationExists {
    tag: "WorkspaceMoveEntryErrorDestinationExists"
    error: string
}
export interface WorkspaceMoveEntryErrorDestinationNotFound {
    tag: "WorkspaceMoveEntryErrorDestinationNotFound"
    error: string
}
export interface WorkspaceMoveEntryErrorInternal {
    tag: "WorkspaceMoveEntryErrorInternal"
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidCertificate {
    tag: "WorkspaceMoveEntryErrorInvalidCertificate"
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidKeysBundle {
    tag: "WorkspaceMoveEntryErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceMoveEntryErrorInvalidManifest {
    tag: "WorkspaceMoveEntryErrorInvalidManifest"
    error: string
}
export interface WorkspaceMoveEntryErrorNoRealmAccess {
    tag: "WorkspaceMoveEntryErrorNoRealmAccess"
    error: string
}
export interface WorkspaceMoveEntryErrorOffline {
    tag: "WorkspaceMoveEntryErrorOffline"
    error: string
}
export interface WorkspaceMoveEntryErrorReadOnlyRealm {
    tag: "WorkspaceMoveEntryErrorReadOnlyRealm"
    error: string
}
export interface WorkspaceMoveEntryErrorSourceNotFound {
    tag: "WorkspaceMoveEntryErrorSourceNotFound"
    error: string
}
export interface WorkspaceMoveEntryErrorStopped {
    tag: "WorkspaceMoveEntryErrorStopped"
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
export interface WorkspaceOpenFileErrorEntryExistsInCreateNewMode {
    tag: "WorkspaceOpenFileErrorEntryExistsInCreateNewMode"
    error: string
}
export interface WorkspaceOpenFileErrorEntryNotAFile {
    tag: "WorkspaceOpenFileErrorEntryNotAFile"
    error: string
}
export interface WorkspaceOpenFileErrorEntryNotFound {
    tag: "WorkspaceOpenFileErrorEntryNotFound"
    error: string
}
export interface WorkspaceOpenFileErrorInternal {
    tag: "WorkspaceOpenFileErrorInternal"
    error: string
}
export interface WorkspaceOpenFileErrorInvalidCertificate {
    tag: "WorkspaceOpenFileErrorInvalidCertificate"
    error: string
}
export interface WorkspaceOpenFileErrorInvalidKeysBundle {
    tag: "WorkspaceOpenFileErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceOpenFileErrorInvalidManifest {
    tag: "WorkspaceOpenFileErrorInvalidManifest"
    error: string
}
export interface WorkspaceOpenFileErrorNoRealmAccess {
    tag: "WorkspaceOpenFileErrorNoRealmAccess"
    error: string
}
export interface WorkspaceOpenFileErrorOffline {
    tag: "WorkspaceOpenFileErrorOffline"
    error: string
}
export interface WorkspaceOpenFileErrorReadOnlyRealm {
    tag: "WorkspaceOpenFileErrorReadOnlyRealm"
    error: string
}
export interface WorkspaceOpenFileErrorStopped {
    tag: "WorkspaceOpenFileErrorStopped"
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
export interface WorkspaceRemoveEntryErrorCannotRemoveRoot {
    tag: "WorkspaceRemoveEntryErrorCannotRemoveRoot"
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsFile {
    tag: "WorkspaceRemoveEntryErrorEntryIsFile"
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsFolder {
    tag: "WorkspaceRemoveEntryErrorEntryIsFolder"
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder {
    tag: "WorkspaceRemoveEntryErrorEntryIsNonEmptyFolder"
    error: string
}
export interface WorkspaceRemoveEntryErrorEntryNotFound {
    tag: "WorkspaceRemoveEntryErrorEntryNotFound"
    error: string
}
export interface WorkspaceRemoveEntryErrorInternal {
    tag: "WorkspaceRemoveEntryErrorInternal"
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidCertificate {
    tag: "WorkspaceRemoveEntryErrorInvalidCertificate"
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidKeysBundle {
    tag: "WorkspaceRemoveEntryErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceRemoveEntryErrorInvalidManifest {
    tag: "WorkspaceRemoveEntryErrorInvalidManifest"
    error: string
}
export interface WorkspaceRemoveEntryErrorNoRealmAccess {
    tag: "WorkspaceRemoveEntryErrorNoRealmAccess"
    error: string
}
export interface WorkspaceRemoveEntryErrorOffline {
    tag: "WorkspaceRemoveEntryErrorOffline"
    error: string
}
export interface WorkspaceRemoveEntryErrorReadOnlyRealm {
    tag: "WorkspaceRemoveEntryErrorReadOnlyRealm"
    error: string
}
export interface WorkspaceRemoveEntryErrorStopped {
    tag: "WorkspaceRemoveEntryErrorStopped"
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
export interface WorkspaceStatEntryErrorEntryNotFound {
    tag: "WorkspaceStatEntryErrorEntryNotFound"
    error: string
}
export interface WorkspaceStatEntryErrorInternal {
    tag: "WorkspaceStatEntryErrorInternal"
    error: string
}
export interface WorkspaceStatEntryErrorInvalidCertificate {
    tag: "WorkspaceStatEntryErrorInvalidCertificate"
    error: string
}
export interface WorkspaceStatEntryErrorInvalidKeysBundle {
    tag: "WorkspaceStatEntryErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceStatEntryErrorInvalidManifest {
    tag: "WorkspaceStatEntryErrorInvalidManifest"
    error: string
}
export interface WorkspaceStatEntryErrorNoRealmAccess {
    tag: "WorkspaceStatEntryErrorNoRealmAccess"
    error: string
}
export interface WorkspaceStatEntryErrorOffline {
    tag: "WorkspaceStatEntryErrorOffline"
    error: string
}
export interface WorkspaceStatEntryErrorStopped {
    tag: "WorkspaceStatEntryErrorStopped"
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
export interface WorkspaceStatFolderChildrenErrorEntryIsFile {
    tag: "WorkspaceStatFolderChildrenErrorEntryIsFile"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorEntryNotFound {
    tag: "WorkspaceStatFolderChildrenErrorEntryNotFound"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInternal {
    tag: "WorkspaceStatFolderChildrenErrorInternal"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidCertificate {
    tag: "WorkspaceStatFolderChildrenErrorInvalidCertificate"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidKeysBundle {
    tag: "WorkspaceStatFolderChildrenErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorInvalidManifest {
    tag: "WorkspaceStatFolderChildrenErrorInvalidManifest"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorNoRealmAccess {
    tag: "WorkspaceStatFolderChildrenErrorNoRealmAccess"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorOffline {
    tag: "WorkspaceStatFolderChildrenErrorOffline"
    error: string
}
export interface WorkspaceStatFolderChildrenErrorStopped {
    tag: "WorkspaceStatFolderChildrenErrorStopped"
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
export interface WorkspaceStopErrorInternal {
    tag: "WorkspaceStopErrorInternal"
    error: string
}
export type WorkspaceStopError =
  | WorkspaceStopErrorInternal


// WorkspaceStorageCacheSize
export interface WorkspaceStorageCacheSizeCustom {
    tag: "WorkspaceStorageCacheSizeCustom"
    size: number
}
export interface WorkspaceStorageCacheSizeDefault {
    tag: "WorkspaceStorageCacheSizeDefault"
}
export type WorkspaceStorageCacheSize =
  | WorkspaceStorageCacheSizeCustom
  | WorkspaceStorageCacheSizeDefault


// WorkspaceWatchEntryOneShotError
export interface WorkspaceWatchEntryOneShotErrorEntryNotFound {
    tag: "WorkspaceWatchEntryOneShotErrorEntryNotFound"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInternal {
    tag: "WorkspaceWatchEntryOneShotErrorInternal"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidCertificate {
    tag: "WorkspaceWatchEntryOneShotErrorInvalidCertificate"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidKeysBundle {
    tag: "WorkspaceWatchEntryOneShotErrorInvalidKeysBundle"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorInvalidManifest {
    tag: "WorkspaceWatchEntryOneShotErrorInvalidManifest"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorNoRealmAccess {
    tag: "WorkspaceWatchEntryOneShotErrorNoRealmAccess"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorOffline {
    tag: "WorkspaceWatchEntryOneShotErrorOffline"
    error: string
}
export interface WorkspaceWatchEntryOneShotErrorStopped {
    tag: "WorkspaceWatchEntryOneShotErrorStopped"
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
export interface X509URIFlavorValuePKCS11 {
    tag: "X509URIFlavorValuePKCS11"
    x1: X509Pkcs11URI
}
export interface X509URIFlavorValueWindowsCNG {
    tag: "X509URIFlavorValueWindowsCNG"
    x1: X509WindowsCngURI
}
export type X509URIFlavorValue =
  | X509URIFlavorValuePKCS11
  | X509URIFlavorValueWindowsCNG


export function accountCreate1SendValidationEmail(
    config_dir: string,
    addr: string,
    email: string
): Promise<Result<null, AccountCreateSendValidationEmailError>>
export function accountCreate2CheckValidationCode(
    config_dir: string,
    addr: string,
    validation_code: string,
    email: string
): Promise<Result<null, AccountCreateError>>
export function accountCreate3Proceed(
    config_dir: string,
    addr: string,
    validation_code: string,
    human_handle: HumanHandle,
    auth_method_strategy: AccountAuthMethodStrategy
): Promise<Result<null, AccountCreateError>>
export function accountCreateAuthMethod(
    account: number,
    auth_method_strategy: AccountAuthMethodStrategy
): Promise<Result<null, AccountCreateAuthMethodError>>
export function accountCreateRegistrationDevice(
    account: number,
    existing_local_device_access: DeviceAccessStrategy
): Promise<Result<null, AccountCreateRegistrationDeviceError>>
export function accountDelete1SendValidationEmail(
    account: number
): Promise<Result<null, AccountDeleteSendValidationEmailError>>
export function accountDelete2Proceed(
    account: number,
    validation_code: string
): Promise<Result<null, AccountDeleteProceedError>>
export function accountDisableAuthMethod(
    account: number,
    auth_method_id: string
): Promise<Result<null, AccountDisableAuthMethodError>>
export function accountInfo(
    account: number
): Promise<Result<AccountInfo, AccountInfoError>>
export function accountListAuthMethods(
    account: number
): Promise<Result<Array<AuthMethodInfo>, AccountListAuthMethodsError>>
export function accountListInvitations(
    account: number
): Promise<Result<Array<[string, string, string, InvitationType]>, AccountListInvitationsError>>
export function accountListOrganizations(
    account: number
): Promise<Result<AccountOrganizations, AccountListOrganizationsError>>
export function accountListRegistrationDevices(
    account: number
): Promise<Result<Array<[string, string]>, AccountListRegistrationDevicesError>>
export function accountLogin(
    config_dir: string,
    addr: string,
    login_strategy: AccountLoginStrategy
): Promise<Result<number, AccountLoginError>>
export function accountLogout(
    account: number
): Promise<Result<null, AccountLogoutError>>
export function accountRecover1SendValidationEmail(
    config_dir: string,
    addr: string,
    email: string
): Promise<Result<null, AccountRecoverSendValidationEmailError>>
export function accountRecover2Proceed(
    config_dir: string,
    addr: string,
    validation_code: string,
    email: string,
    auth_method_strategy: AccountAuthMethodStrategy
): Promise<Result<null, AccountRecoverProceedError>>
export function accountRegisterNewDevice(
    account: number,
    organization_id: string,
    user_id: string,
    new_device_label: string,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, AccountRegisterNewDeviceError>>
export function archiveDevice(
    config_dir: string,
    device_path: string
): Promise<Result<null, ArchiveDeviceError>>
export function bootstrapOrganization(
    config: ClientConfig,
    bootstrap_organization_addr: string,
    save_strategy: DeviceSaveStrategy,
    human_handle: HumanHandle,
    device_label: string,
    sequester_authority_verify_key_pem: string | null
): Promise<Result<AvailableDevice, BootstrapOrganizationError>>
export function buildParsecAddr(
    hostname: string,
    port: number | null,
    use_ssl: boolean
): Promise<string>
export function buildParsecOrganizationBootstrapAddr(
    addr: string,
    organization_id: string
): Promise<string>
export function cancel(
    canceller: number
): Promise<Result<null, CancelError>>
export function claimerDeviceFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimFinalizeError>>
export function claimerDeviceInProgress1DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, ClaimInProgressError>>
export function claimerDeviceInProgress1DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress2Info, ClaimInProgressError>>
export function claimerDeviceInProgress2DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress3Info, ClaimInProgressError>>
export function claimerDeviceInProgress3DoClaim(
    canceller: number,
    handle: number,
    requested_device_label: string
): Promise<Result<DeviceClaimFinalizeInfo, ClaimInProgressError>>
export function claimerDeviceInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<DeviceClaimInProgress1Info, ClaimInProgressError>>
export function claimerGreeterAbortOperation(
    handle: number
): Promise<Result<null, ClaimerGreeterAbortOperationError>>
export function claimerRetrieveInfo(
    config: ClientConfig,
    addr: string
): Promise<Result<AnyClaimRetrievedInfo, ClaimerRetrieveInfoError>>
export function claimerShamirRecoveryAddShare(
    recipient_pick_handle: number,
    share_handle: number
): Promise<Result<ShamirRecoveryClaimMaybeRecoverDeviceInfo, ShamirRecoveryClaimAddShareError>>
export function claimerShamirRecoveryFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimFinalizeError>>
export function claimerShamirRecoveryInProgress1DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, ClaimInProgressError>>
export function claimerShamirRecoveryInProgress1DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryClaimInProgress2Info, ClaimInProgressError>>
export function claimerShamirRecoveryInProgress2DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryClaimInProgress3Info, ClaimInProgressError>>
export function claimerShamirRecoveryInProgress3DoClaim(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryClaimShareInfo, ClaimInProgressError>>
export function claimerShamirRecoveryInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryClaimInProgress1Info, ClaimInProgressError>>
export function claimerShamirRecoveryPickRecipient(
    handle: number,
    recipient_user_id: string
): Promise<Result<ShamirRecoveryClaimInitialInfo, ShamirRecoveryClaimPickRecipientError>>
export function claimerShamirRecoveryRecoverDevice(
    handle: number,
    requested_device_label: string
): Promise<Result<ShamirRecoveryClaimMaybeFinalizeInfo, ShamirRecoveryClaimRecoverDeviceError>>
export function claimerUserFinalizeSaveLocalDevice(
    handle: number,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ClaimFinalizeError>>
export function claimerUserInProgress1DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, ClaimInProgressError>>
export function claimerUserInProgress1DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>>
export function claimerUserInProgress2DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>>
export function claimerUserInProgress3DoClaim(
    canceller: number,
    handle: number,
    requested_device_label: string,
    requested_human_handle: HumanHandle
): Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>>
export function claimerUserInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
export function claimerUserListInitialInfo(
    handle: number
): Promise<Result<Array<UserClaimInitialInfo>, UserClaimListInitialInfosError>>
export function claimerUserWaitAllPeers(
    canceller: number,
    handle: number
): Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>
export function clientAcceptAsyncEnrollment(
    client: number,
    profile: UserProfile,
    enrollment_id: string,
    identity_strategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy
): Promise<Result<null, ClientAcceptAsyncEnrollmentError>>
export function clientAcceptTos(
    client: number,
    tos_updated_on: number
): Promise<Result<null, ClientAcceptTosError>>
export function clientCancelInvitation(
    client: number,
    token: string
): Promise<Result<null, ClientCancelInvitationError>>
export function clientCreateWorkspace(
    client: number,
    name: string
): Promise<Result<string, ClientCreateWorkspaceError>>
export function clientDeleteShamirRecovery(
    client_handle: number
): Promise<Result<null, ClientDeleteShamirRecoveryError>>
export function clientExportRecoveryDevice(
    client_handle: number,
    device_label: string
): Promise<Result<[string, Uint8Array], ClientExportRecoveryDeviceError>>
export function clientForgetAllCertificates(
    client: number
): Promise<Result<null, ClientForgetAllCertificatesError>>
export function clientGetAsyncEnrollmentAddr(
    client: number
): Promise<Result<string, ClientGetAsyncEnrollmentAddrError>>
export function clientGetOrganizationBootstrapDate(
    client_handle: number
): Promise<Result<number, ClientGetOrganizationBootstrapDateError>>
export function clientGetSelfShamirRecovery(
    client_handle: number
): Promise<Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError>>
export function clientGetTos(
    client: number
): Promise<Result<Tos, ClientGetTosError>>
export function clientGetUserDevice(
    client: number,
    device: string
): Promise<Result<[UserInfo, DeviceInfo], ClientGetUserDeviceError>>
export function clientGetUserInfo(
    client: number,
    user_id: string
): Promise<Result<UserInfo, ClientGetUserInfoError>>
export function clientInfo(
    client: number
): Promise<Result<ClientInfo, ClientInfoError>>
export function clientListAsyncEnrollments(
    client: number
): Promise<Result<Array<AsyncEnrollmentUntrusted>, ClientListAsyncEnrollmentsError>>
export function clientListFrozenUsers(
    client_handle: number
): Promise<Result<Array<string>, ClientListFrozenUsersError>>
export function clientListInvitations(
    client: number
): Promise<Result<Array<InviteListItem>, ListInvitationsError>>
export function clientListShamirRecoveriesForOthers(
    client_handle: number
): Promise<Result<Array<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError>>
export function clientListUserDevices(
    client: number,
    user: string
): Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>>
export function clientListUsers(
    client: number,
    skip_revoked: boolean
): Promise<Result<Array<UserInfo>, ClientListUsersError>>
export function clientListWorkspaceUsers(
    client: number,
    realm_id: string
): Promise<Result<Array<WorkspaceUserAccessInfo>, ClientListWorkspaceUsersError>>
export function clientListWorkspaces(
    client: number
): Promise<Result<Array<WorkspaceInfo>, ClientListWorkspacesError>>
export function clientNewDeviceInvitation(
    client: number,
    send_email: boolean
): Promise<Result<NewInvitationInfo, ClientNewDeviceInvitationError>>
export function clientNewShamirRecoveryInvitation(
    client: number,
    claimer_user_id: string,
    send_email: boolean
): Promise<Result<NewInvitationInfo, ClientNewShamirRecoveryInvitationError>>
export function clientNewUserInvitation(
    client: number,
    claimer_email: string,
    send_email: boolean
): Promise<Result<NewInvitationInfo, ClientNewUserInvitationError>>
export function clientOrganizationInfo(
    client_handle: number
): Promise<Result<OrganizationInfo, ClientOrganizationInfoError>>
export function clientPkiEnrollmentAccept(
    client_handle: number,
    profile: UserProfile,
    enrollment_id: string,
    accepter_cert_ref: X509CertificateReference,
    submitter_der_cert: Uint8Array,
    submit_payload: PkiEnrollmentSubmitPayload
): Promise<Result<null, PkiEnrollmentAcceptError>>
export function clientPkiEnrollmentReject(
    client_handle: number,
    enrollment_id: string
): Promise<Result<null, PkiEnrollmentRejectError>>
export function clientPkiGetAddr(
    client: number
): Promise<Result<string, PkiGetAddrError>>
export function clientPkiListEnrollmentsUntrusted(
    client_handle: number
): Promise<Result<Array<RawPkiEnrollmentListItem>, PkiEnrollmentListError>>
export function clientPkiListVerifyItems(
    client_handle: number,
    cert_ref: X509CertificateReference,
    untrusted_items: Array<RawPkiEnrollmentListItem>
): Promise<Result<Array<PkiEnrollmentListItem>, PkiEnrollmentListError>>
export function clientRejectAsyncEnrollment(
    client: number,
    enrollment_id: string
): Promise<Result<null, ClientRejectAsyncEnrollmentError>>
export function clientRenameWorkspace(
    client: number,
    realm_id: string,
    new_name: string
): Promise<Result<null, ClientRenameWorkspaceError>>
export function clientRevokeUser(
    client: number,
    user: string
): Promise<Result<null, ClientRevokeUserError>>
export function clientSetupShamirRecovery(
    client_handle: number,
    per_recipient_shares: Map<string, number>,
    threshold: number
): Promise<Result<null, ClientSetupShamirRecoveryError>>
export function clientShareWorkspace(
    client: number,
    realm_id: string,
    recipient: string,
    role: RealmRole | null
): Promise<Result<null, ClientShareWorkspaceError>>
export function clientStart(
    config: ClientConfig,
    access: DeviceAccessStrategy
): Promise<Result<number, ClientStartError>>
export function clientStartDeviceInvitationGreet(
    client: number,
    token: string
): Promise<Result<DeviceGreetInitialInfo, ClientStartInvitationGreetError>>
export function clientStartShamirRecoveryInvitationGreet(
    client: number,
    token: string
): Promise<Result<ShamirRecoveryGreetInitialInfo, ClientStartShamirRecoveryInvitationGreetError>>
export function clientStartUserInvitationGreet(
    client: number,
    token: string
): Promise<Result<UserGreetInitialInfo, ClientStartInvitationGreetError>>
export function clientStartWorkspace(
    client: number,
    realm_id: string
): Promise<Result<number, ClientStartWorkspaceError>>
export function clientStartWorkspaceHistory(
    client: number,
    realm_id: string
): Promise<Result<number, WorkspaceHistoryStartError>>
export function clientStop(
    client: number
): Promise<Result<null, ClientStopError>>
export function clientUpdateUserProfile(
    client_handle: number,
    user: string,
    new_profile: UserProfile
): Promise<Result<null, ClientUserUpdateProfileError>>
export function getDefaultConfigDir(
): Promise<string>
export function getDefaultDataBaseDir(
): Promise<string>
export function getDefaultMountpointBaseDir(
): Promise<string>
export function getPlatform(
): Promise<Platform>
export function getServerConfig(
    config_dir: string,
    addr: string
): Promise<Result<ServerConfig, GetServerConfigError>>
export function greeterDeviceInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress2Info, GreetInProgressError>>
export function greeterDeviceInProgress2DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, GreetInProgressError>>
export function greeterDeviceInProgress2DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress3Info, GreetInProgressError>>
export function greeterDeviceInProgress3DoGetClaimRequests(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress4Info, GreetInProgressError>>
export function greeterDeviceInProgress4DoCreate(
    canceller: number,
    handle: number,
    device_label: string
): Promise<Result<null, GreetInProgressError>>
export function greeterDeviceInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<DeviceGreetInProgress1Info, GreetInProgressError>>
export function greeterShamirRecoveryInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryGreetInProgress2Info, GreetInProgressError>>
export function greeterShamirRecoveryInProgress2DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, GreetInProgressError>>
export function greeterShamirRecoveryInProgress2DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryGreetInProgress3Info, GreetInProgressError>>
export function greeterShamirRecoveryInProgress3DoGetClaimRequests(
    canceller: number,
    handle: number
): Promise<Result<null, GreetInProgressError>>
export function greeterShamirRecoveryInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<ShamirRecoveryGreetInProgress1Info, GreetInProgressError>>
export function greeterUserInProgress1DoWaitPeerTrust(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress2Info, GreetInProgressError>>
export function greeterUserInProgress2DoDenyTrust(
    canceller: number,
    handle: number
): Promise<Result<null, GreetInProgressError>>
export function greeterUserInProgress2DoSignifyTrust(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress3Info, GreetInProgressError>>
export function greeterUserInProgress3DoGetClaimRequests(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress4Info, GreetInProgressError>>
export function greeterUserInProgress4DoCreate(
    canceller: number,
    handle: number,
    human_handle: HumanHandle,
    device_label: string,
    profile: UserProfile
): Promise<Result<null, GreetInProgressError>>
export function greeterUserInitialDoWaitPeer(
    canceller: number,
    handle: number
): Promise<Result<UserGreetInProgress1Info, GreetInProgressError>>
export function importRecoveryDevice(
    config: ClientConfig,
    recovery_device: Uint8Array,
    passphrase: string,
    device_label: string,
    save_strategy: DeviceSaveStrategy
): Promise<Result<AvailableDevice, ImportRecoveryDeviceError>>
export function isKeyringAvailable(
): Promise<boolean>
export function isPkiAvailable(
): Promise<boolean>
export function libparsecInitNativeOnlyInit(
    config: ClientConfig
): Promise<null>
export function libparsecInitSetOnEventCallback(
    on_event_callback: (handle: number, event: ClientEvent) => void
): Promise<null>
export function listAvailableDevices(
    path: string
): Promise<Result<Array<AvailableDevice>, ListAvailableDeviceError>>
export function listPkiLocalPendingEnrollments(
    config_dir: string
): Promise<Result<Array<PKILocalPendingEnrollment>, ListPkiLocalPendingError>>
export function listStartedAccounts(
): Promise<Array<number>>
export function listStartedClients(
): Promise<Array<[number, string]>>
export function mountpointToOsPath(
    mountpoint: number,
    parsec_path: string
): Promise<Result<string, MountpointToOsPathError>>
export function mountpointUnmount(
    mountpoint: number
): Promise<Result<null, MountpointUnmountError>>
export function newCanceller(
): Promise<number>
export function openbaoListSelfEmails(
    openbao_server_url: string,
    openbao_secret_mount_path: string,
    openbao_transit_mount_path: string,
    openbao_entity_id: string,
    openbao_auth_token: string
): Promise<Result<Array<string>, OpenBaoListSelfEmailsError>>
export function parseParsecAddr(
    url: string
): Promise<Result<ParsedParsecAddr, ParseParsecAddrError>>
export function pathFilename(
    path: string
): Promise<string | null>
export function pathJoin(
    parent: string,
    child: string
): Promise<string>
export function pathNormalize(
    path: string
): Promise<string>
export function pathParent(
    path: string
): Promise<string>
export function pathSplit(
    path: string
): Promise<Array<string>>
export function pkiEnrollmentFinalize(
    config: ClientConfig,
    save_strategy: DeviceSaveStrategy,
    accepted: PkiEnrollmentAnswerPayload,
    local_pending: PKILocalPendingEnrollment
): Promise<Result<AvailableDevice, PkiEnrollmentFinalizeError>>
export function pkiEnrollmentInfo(
    config: ClientConfig,
    addr: string,
    cert_ref: X509CertificateReference,
    enrollment_id: string
): Promise<Result<PKIInfoItem, PkiEnrollmentInfoError>>
export function pkiEnrollmentSubmit(
    config: ClientConfig,
    addr: string,
    cert_ref: X509CertificateReference,
    device_label: string,
    force: boolean
): Promise<Result<number, PkiEnrollmentSubmitError>>
export function pkiRemoveLocalPending(
    config: ClientConfig,
    id: string
): Promise<Result<null, RemoveDeviceError>>
export function removeDeviceData(
    config: ClientConfig,
    device_id: string
): Promise<Result<null, RemoveDeviceDataError>>
export function showCertificateSelectionDialogWindowsOnly(
): Promise<Result<X509CertificateReference | null, ShowCertificateSelectionDialogError>>
export function submitAsyncEnrollment(
    config: ClientConfig,
    addr: string,
    force: boolean,
    requested_device_label: string,
    identity_strategy: SubmitAsyncEnrollmentIdentityStrategy
): Promise<Result<AvailablePendingAsyncEnrollment, SubmitAsyncEnrollmentError>>
export function submitterFinalizeAsyncEnrollment(
    config: ClientConfig,
    enrollment_file: string,
    new_device_save_strategy: DeviceSaveStrategy,
    identity_strategy: AcceptFinalizeAsyncEnrollmentIdentityStrategy
): Promise<Result<AvailableDevice, SubmitterFinalizeAsyncEnrollmentError>>
export function submitterForgetAsyncEnrollment(
    config_dir: string,
    enrollment_id: string
): Promise<Result<null, SubmitterForgetAsyncEnrollmentError>>
export function submitterGetAsyncEnrollmentInfo(
    config: ClientConfig,
    addr: string,
    enrollment_id: string
): Promise<Result<PendingAsyncEnrollmentInfo, SubmitterGetAsyncEnrollmentInfoError>>
export function submitterListAsyncEnrollments(
    config_dir: string
): Promise<Result<Array<AvailablePendingAsyncEnrollment>, SubmitterListLocalAsyncEnrollmentsError>>
export function testCheckMailbox(
    server_addr: string,
    email: string
): Promise<Result<Array<[string, number, string]>, TestbedError>>
export function testDropTestbed(
    path: string
): Promise<Result<null, TestbedError>>
export function testGetTestbedBootstrapOrganizationAddr(
    discriminant_dir: string
): Promise<Result<string | null, TestbedError>>
export function testGetTestbedOrganizationId(
    discriminant_dir: string
): Promise<Result<string | null, TestbedError>>
export function testNewAccount(
    server_addr: string
): Promise<Result<[HumanHandle, Uint8Array], TestbedError>>
export function testNewTestbed(
    template: string,
    test_server: string | null
): Promise<Result<string, TestbedError>>
export function updateDeviceChangeAuthentication(
    config_dir: string,
    current_auth: DeviceAccessStrategy,
    new_auth: DeviceSaveStrategy
): Promise<Result<AvailableDevice, UpdateDeviceError>>
export function updateDeviceOverwriteServerAddr(
    config_dir: string,
    access: DeviceAccessStrategy,
    new_server_addr: string
): Promise<Result<string, UpdateDeviceError>>
export function validateDeviceLabel(
    raw: string
): Promise<boolean>
export function validateEmail(
    raw: string
): Promise<boolean>
export function validateEntryName(
    raw: string
): Promise<boolean>
export function validateHumanHandleLabel(
    raw: string
): Promise<boolean>
export function validateInvitationToken(
    raw: string
): Promise<boolean>
export function validateOrganizationId(
    raw: string
): Promise<boolean>
export function validatePath(
    raw: string
): Promise<boolean>
export function waitForDeviceAvailable(
    config_dir: string,
    device_id: string
): Promise<Result<null, WaitForDeviceAvailableError>>
export function workspaceCreateFile(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceCreateFileError>>
export function workspaceCreateFolder(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceCreateFolderError>>
export function workspaceCreateFolderAll(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceCreateFolderError>>
export function workspaceDecryptPathAddr(
    workspace: number,
    link: string
): Promise<Result<string, WorkspaceDecryptPathAddrError>>
export function workspaceFdClose(
    workspace: number,
    fd: number
): Promise<Result<null, WorkspaceFdCloseError>>
export function workspaceFdFlush(
    workspace: number,
    fd: number
): Promise<Result<null, WorkspaceFdFlushError>>
export function workspaceFdRead(
    workspace: number,
    fd: number,
    offset: number,
    size: number
): Promise<Result<Uint8Array, WorkspaceFdReadError>>
export function workspaceFdResize(
    workspace: number,
    fd: number,
    length: number,
    truncate_only: boolean
): Promise<Result<null, WorkspaceFdResizeError>>
export function workspaceFdStat(
    workspace: number,
    fd: number
): Promise<Result<FileStat, WorkspaceFdStatError>>
export function workspaceFdWrite(
    workspace: number,
    fd: number,
    offset: number,
    data: Uint8Array
): Promise<Result<number, WorkspaceFdWriteError>>
export function workspaceFdWriteConstrainedIo(
    workspace: number,
    fd: number,
    offset: number,
    data: Uint8Array
): Promise<Result<number, WorkspaceFdWriteError>>
export function workspaceFdWriteStartEof(
    workspace: number,
    fd: number,
    data: Uint8Array
): Promise<Result<number, WorkspaceFdWriteError>>
export function workspaceGeneratePathAddr(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceGeneratePathAddrError>>
export function workspaceHistoryFdClose(
    workspace_history: number,
    fd: number
): Promise<Result<null, WorkspaceHistoryFdCloseError>>
export function workspaceHistoryFdRead(
    workspace_history: number,
    fd: number,
    offset: number,
    size: number
): Promise<Result<Uint8Array, WorkspaceHistoryFdReadError>>
export function workspaceHistoryFdStat(
    workspace_history: number,
    fd: number
): Promise<Result<WorkspaceHistoryFileStat, WorkspaceHistoryFdStatError>>
export function workspaceHistoryGetTimestampHigherBound(
    workspace_history: number
): Promise<Result<number, WorkspaceHistoryInternalOnlyError>>
export function workspaceHistoryGetTimestampLowerBound(
    workspace_history: number
): Promise<Result<number, WorkspaceHistoryInternalOnlyError>>
export function workspaceHistoryGetTimestampOfInterest(
    workspace_history: number
): Promise<Result<number, WorkspaceHistoryInternalOnlyError>>
export function workspaceHistoryOpenFile(
    workspace_history: number,
    path: string
): Promise<Result<number, WorkspaceHistoryOpenFileError>>
export function workspaceHistoryOpenFileAndGetId(
    workspace_history: number,
    path: string
): Promise<Result<[number, string], WorkspaceHistoryOpenFileError>>
export function workspaceHistoryOpenFileById(
    workspace_history: number,
    entry_id: string
): Promise<Result<number, WorkspaceHistoryOpenFileError>>
export function workspaceHistorySetTimestampOfInterest(
    workspace_history: number,
    toi: number
): Promise<Result<null, WorkspaceHistorySetTimestampOfInterestError>>
export function workspaceHistoryStartWithRealmExport(
    config: ClientConfig,
    export_db_path: string,
    decryptors: Array<WorkspaceHistoryRealmExportDecryptor>
): Promise<Result<number, WorkspaceHistoryStartError>>
export function workspaceHistoryStatEntry(
    workspace_history: number,
    path: string
): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
export function workspaceHistoryStatEntryById(
    workspace_history: number,
    entry_id: string
): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>>
export function workspaceHistoryStatFolderChildren(
    workspace_history: number,
    path: string
): Promise<Result<Array<[string, WorkspaceHistoryEntryStat]>, WorkspaceHistoryStatFolderChildrenError>>
export function workspaceHistoryStatFolderChildrenById(
    workspace_history: number,
    entry_id: string
): Promise<Result<Array<[string, WorkspaceHistoryEntryStat]>, WorkspaceHistoryStatFolderChildrenError>>
export function workspaceHistoryStop(
    workspace_history: number
): Promise<Result<null, WorkspaceHistoryInternalOnlyError>>
export function workspaceInfo(
    workspace: number
): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError>>
export function workspaceIsFileContentLocal(
    workspace: number,
    path: string
): Promise<Result<boolean, WorkspaceIsFileContentLocalError>>
export function workspaceMount(
    workspace: number
): Promise<Result<[number, string], WorkspaceMountError>>
export function workspaceMoveEntry(
    workspace: number,
    src: string,
    dst: string,
    mode: MoveEntryMode
): Promise<Result<null, WorkspaceMoveEntryError>>
export function workspaceOpenFile(
    workspace: number,
    path: string,
    mode: OpenOptions
): Promise<Result<number, WorkspaceOpenFileError>>
export function workspaceOpenFileAndGetId(
    workspace: number,
    path: string,
    mode: OpenOptions
): Promise<Result<[number, string], WorkspaceOpenFileError>>
export function workspaceOpenFileById(
    workspace: number,
    entry_id: string,
    mode: OpenOptions
): Promise<Result<number, WorkspaceOpenFileError>>
export function workspaceRemoveEntry(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceRemoveEntryError>>
export function workspaceRemoveFile(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceRemoveEntryError>>
export function workspaceRemoveFolder(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceRemoveEntryError>>
export function workspaceRemoveFolderAll(
    workspace: number,
    path: string
): Promise<Result<null, WorkspaceRemoveEntryError>>
export function workspaceRenameEntryById(
    workspace: number,
    src_parent_id: string,
    src_name: string,
    dst_name: string,
    mode: MoveEntryMode
): Promise<Result<null, WorkspaceMoveEntryError>>
export function workspaceStatEntry(
    workspace: number,
    path: string
): Promise<Result<EntryStat, WorkspaceStatEntryError>>
export function workspaceStatEntryById(
    workspace: number,
    entry_id: string
): Promise<Result<EntryStat, WorkspaceStatEntryError>>
export function workspaceStatEntryByIdIgnoreConfinementPoint(
    workspace: number,
    entry_id: string
): Promise<Result<EntryStat, WorkspaceStatEntryError>>
export function workspaceStatFolderChildren(
    workspace: number,
    path: string
): Promise<Result<Array<[string, EntryStat]>, WorkspaceStatFolderChildrenError>>
export function workspaceStatFolderChildrenById(
    workspace: number,
    entry_id: string
): Promise<Result<Array<[string, EntryStat]>, WorkspaceStatFolderChildrenError>>
export function workspaceStop(
    workspace: number
): Promise<Result<null, WorkspaceStopError>>
export function workspaceWatchEntryOneshot(
    workspace: number,
    path: string
): Promise<Result<string, WorkspaceWatchEntryOneShotError>>
