# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec._parsec import anonymous_cmds, authenticated_cmds, invited_cmds

#
# Authenticated cmds
#

# human_find
ApiV2V3_HumanFindReq = authenticated_cmds.v3.human_find.Req
ApiV2V3_HumanFindRep = authenticated_cmds.v3.human_find.Rep
ApiV2V3_HumanFindRepUnknownStatus = authenticated_cmds.v3.human_find.RepUnknownStatus
ApiV2V3_HumanFindRepOk = authenticated_cmds.v3.human_find.RepOk
ApiV2V3_HumanFindRepNotAllowed = authenticated_cmds.v3.human_find.RepNotAllowed
ApiV2V3_HumanFindResultItem = authenticated_cmds.v3.human_find.HumanFindResultItem

# vlob_maintenance_save_reencryption_batch
VlobMaintenanceSaveReencryptionBatchReq = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.Req
)
VlobMaintenanceSaveReencryptionBatchRep = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.Rep
)
VlobMaintenanceSaveReencryptionBatchRepUnknownStatus = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepUnknownStatus
)
VlobMaintenanceSaveReencryptionBatchRepOk = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepOk
)
VlobMaintenanceSaveReencryptionBatchRepNotAllowed = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepNotAllowed
)
VlobMaintenanceSaveReencryptionBatchRepNotFound = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepNotFound
)
VlobMaintenanceSaveReencryptionBatchRepNotInMaintenance = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepNotInMaintenance
)
VlobMaintenanceSaveReencryptionBatchRepBadEncryptionRevision = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepBadEncryptionRevision
)
VlobMaintenanceSaveReencryptionBatchRepMaintenanceError = (
    authenticated_cmds.latest.vlob_maintenance_save_reencryption_batch.RepMaintenanceError
)

# organization_stats
OrganizationStatsReq = authenticated_cmds.latest.organization_stats.Req
OrganizationStatsRep = authenticated_cmds.latest.organization_stats.Rep
OrganizationStatsRepUnknownStatus = authenticated_cmds.latest.organization_stats.RepUnknownStatus
OrganizationStatsRepOk = authenticated_cmds.latest.organization_stats.RepOk
OrganizationStatsRepNotAllowed = authenticated_cmds.latest.organization_stats.RepNotAllowed
OrganizationStatsRepNotFound = authenticated_cmds.latest.organization_stats.RepNotFound

# pki_enrollment_reject
PkiEnrollmentRejectReq = authenticated_cmds.latest.pki_enrollment_reject.Req
PkiEnrollmentRejectRep = authenticated_cmds.latest.pki_enrollment_reject.Rep
PkiEnrollmentRejectRepUnknownStatus = (
    authenticated_cmds.latest.pki_enrollment_reject.RepUnknownStatus
)
PkiEnrollmentRejectRepOk = authenticated_cmds.latest.pki_enrollment_reject.RepOk
PkiEnrollmentRejectRepNotAllowed = authenticated_cmds.latest.pki_enrollment_reject.RepNotAllowed
PkiEnrollmentRejectRepNotFound = authenticated_cmds.latest.pki_enrollment_reject.RepNotFound
PkiEnrollmentRejectRepNoLongerAvailable = (
    authenticated_cmds.latest.pki_enrollment_reject.RepNoLongerAvailable
)

# block_create
BlockCreateReq = authenticated_cmds.latest.block_create.Req
BlockCreateRep = authenticated_cmds.latest.block_create.Rep
BlockCreateRepUnknownStatus = authenticated_cmds.latest.block_create.RepUnknownStatus
BlockCreateRepOk = authenticated_cmds.latest.block_create.RepOk
BlockCreateRepAlreadyExists = authenticated_cmds.latest.block_create.RepAlreadyExists
BlockCreateRepNotFound = authenticated_cmds.latest.block_create.RepNotFound
BlockCreateRepTimeout = authenticated_cmds.latest.block_create.RepTimeout
BlockCreateRepNotAllowed = authenticated_cmds.latest.block_create.RepNotAllowed
BlockCreateRepInMaintenance = authenticated_cmds.latest.block_create.RepInMaintenance

# realm_stats
RealmStatsReq = authenticated_cmds.latest.realm_stats.Req
RealmStatsRep = authenticated_cmds.latest.realm_stats.Rep
RealmStatsRepUnknownStatus = authenticated_cmds.latest.realm_stats.RepUnknownStatus
RealmStatsRepOk = authenticated_cmds.latest.realm_stats.RepOk
RealmStatsRepNotAllowed = authenticated_cmds.latest.realm_stats.RepNotAllowed
RealmStatsRepNotFound = authenticated_cmds.latest.realm_stats.RepNotFound

# user_get
ApiV2V3_UserGetReq = authenticated_cmds.v3.user_get.Req
ApiV2V3_UserGetRep = authenticated_cmds.v3.user_get.Rep
ApiV2V3_UserGetRepUnknownStatus = authenticated_cmds.v3.user_get.RepUnknownStatus
ApiV2V3_UserGetRepOk = authenticated_cmds.v3.user_get.RepOk
ApiV2V3_UserGetRepNotFound = authenticated_cmds.v3.user_get.RepNotFound
ApiV2V3_Trustchain = authenticated_cmds.v3.user_get.Trustchain

# invite_list
InviteListReq = authenticated_cmds.latest.invite_list.Req
InviteListRep = authenticated_cmds.latest.invite_list.Rep
InviteListRepUnknownStatus = authenticated_cmds.latest.invite_list.RepUnknownStatus
InviteListRepOk = authenticated_cmds.latest.invite_list.RepOk
InviteListItem = authenticated_cmds.latest.invite_list.InviteListItem
InviteListItemDevice = authenticated_cmds.latest.invite_list.InviteListItemDevice
InviteListItemUser = authenticated_cmds.latest.invite_list.InviteListItemUser

# pki_enrollment_accept
PkiEnrollmentAcceptReq = authenticated_cmds.latest.pki_enrollment_accept.Req
PkiEnrollmentAcceptRep = authenticated_cmds.latest.pki_enrollment_accept.Rep
PkiEnrollmentAcceptRepUnknownStatus = (
    authenticated_cmds.latest.pki_enrollment_accept.RepUnknownStatus
)
PkiEnrollmentAcceptRepOk = authenticated_cmds.latest.pki_enrollment_accept.RepOk
PkiEnrollmentAcceptRepNotAllowed = authenticated_cmds.latest.pki_enrollment_accept.RepNotAllowed
PkiEnrollmentAcceptRepInvalidPayloadData = (
    authenticated_cmds.latest.pki_enrollment_accept.RepInvalidPayloadData
)
PkiEnrollmentAcceptRepInvalidCertification = (
    authenticated_cmds.latest.pki_enrollment_accept.RepInvalidCertification
)
PkiEnrollmentAcceptRepInvalidData = authenticated_cmds.latest.pki_enrollment_accept.RepInvalidData
PkiEnrollmentAcceptRepNotFound = authenticated_cmds.latest.pki_enrollment_accept.RepNotFound
PkiEnrollmentAcceptRepNoLongerAvailable = (
    authenticated_cmds.latest.pki_enrollment_accept.RepNoLongerAvailable
)
PkiEnrollmentAcceptRepAlreadyExists = (
    authenticated_cmds.latest.pki_enrollment_accept.RepAlreadyExists
)
PkiEnrollmentAcceptRepActiveUsersLimitReached = (
    authenticated_cmds.latest.pki_enrollment_accept.RepActiveUsersLimitReached
)

# realm_get_role_certificates
ApiV2V3_RealmGetRoleCertificatesReq = authenticated_cmds.v3.realm_get_role_certificates.Req
ApiV2V3_RealmGetRoleCertificatesRep = authenticated_cmds.v3.realm_get_role_certificates.Rep
ApiV2V3_RealmGetRoleCertificatesRepUnknownStatus = (
    authenticated_cmds.v3.realm_get_role_certificates.RepUnknownStatus
)
ApiV2V3_RealmGetRoleCertificatesRepOk = authenticated_cmds.v3.realm_get_role_certificates.RepOk
ApiV2V3_RealmGetRoleCertificatesRepNotAllowed = (
    authenticated_cmds.v3.realm_get_role_certificates.RepNotAllowed
)
ApiV2V3_RealmGetRoleCertificatesRepNotFound = (
    authenticated_cmds.v3.realm_get_role_certificates.RepNotFound
)

# invite_3b_greeter_signify_trust
Invite3bGreeterSignifyTrustReq = authenticated_cmds.latest.invite_3b_greeter_signify_trust.Req
Invite3bGreeterSignifyTrustRep = authenticated_cmds.latest.invite_3b_greeter_signify_trust.Rep
Invite3bGreeterSignifyTrustRepUnknownStatus = (
    authenticated_cmds.latest.invite_3b_greeter_signify_trust.RepUnknownStatus
)
Invite3bGreeterSignifyTrustRepOk = authenticated_cmds.latest.invite_3b_greeter_signify_trust.RepOk
Invite3bGreeterSignifyTrustRepNotFound = (
    authenticated_cmds.latest.invite_3b_greeter_signify_trust.RepNotFound
)
Invite3bGreeterSignifyTrustRepAlreadyDeleted = (
    authenticated_cmds.latest.invite_3b_greeter_signify_trust.RepAlreadyDeleted
)
Invite3bGreeterSignifyTrustRepInvalidState = (
    authenticated_cmds.latest.invite_3b_greeter_signify_trust.RepInvalidState
)

# vlob_update
VlobUpdateReq = authenticated_cmds.latest.vlob_update.Req
VlobUpdateRep = authenticated_cmds.latest.vlob_update.Rep
VlobUpdateRepUnknownStatus = authenticated_cmds.latest.vlob_update.RepUnknownStatus
VlobUpdateRepOk = authenticated_cmds.latest.vlob_update.RepOk
VlobUpdateRepNotFound = authenticated_cmds.latest.vlob_update.RepNotFound
VlobUpdateRepNotAllowed = authenticated_cmds.latest.vlob_update.RepNotAllowed
VlobUpdateRepBadVersion = authenticated_cmds.latest.vlob_update.RepBadVersion
VlobUpdateRepBadEncryptionRevision = authenticated_cmds.latest.vlob_update.RepBadEncryptionRevision
VlobUpdateRepInMaintenance = authenticated_cmds.latest.vlob_update.RepInMaintenance
VlobUpdateRepRequireGreaterTimestamp = (
    authenticated_cmds.latest.vlob_update.RepRequireGreaterTimestamp
)
VlobUpdateRepBadTimestamp = authenticated_cmds.latest.vlob_update.RepBadTimestamp
VlobUpdateRepNotASequesteredOrganization = (
    authenticated_cmds.latest.vlob_update.RepNotASequesteredOrganization
)
VlobUpdateRepSequesterInconsistency = (
    authenticated_cmds.latest.vlob_update.RepSequesterInconsistency
)
VlobUpdateRepRejectedBySequesterService = (
    authenticated_cmds.latest.vlob_update.RepRejectedBySequesterService
)
VlobUpdateRepTimeout = authenticated_cmds.latest.vlob_update.RepTimeout

# realm_finish_reencryption_maintenance
RealmFinishReencryptionMaintenanceReq = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.Req
)
RealmFinishReencryptionMaintenanceRep = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.Rep
)
RealmFinishReencryptionMaintenanceRepUnknownStatus = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.RepUnknownStatus
)
RealmFinishReencryptionMaintenanceRepOk = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.RepOk
)
RealmFinishReencryptionMaintenanceRepNotAllowed = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.RepNotAllowed
)
RealmFinishReencryptionMaintenanceRepNotFound = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.RepNotFound
)
RealmFinishReencryptionMaintenanceRepBadEncryptionRevision = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.RepBadEncryptionRevision
)
RealmFinishReencryptionMaintenanceRepNotInMaintenance = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.RepNotInMaintenance
)
RealmFinishReencryptionMaintenanceRepMaintenanceError = (
    authenticated_cmds.latest.realm_finish_reencryption_maintenance.RepMaintenanceError
)

# invite_delete
InviteDeleteReq = authenticated_cmds.latest.invite_delete.Req
InviteDeleteRep = authenticated_cmds.latest.invite_delete.Rep
InviteDeleteRepUnknownStatus = authenticated_cmds.latest.invite_delete.RepUnknownStatus
InviteDeleteRepOk = authenticated_cmds.latest.invite_delete.RepOk
InviteDeleteRepNotFound = authenticated_cmds.latest.invite_delete.RepNotFound
InviteDeleteRepAlreadyDeleted = authenticated_cmds.latest.invite_delete.RepAlreadyDeleted
InvitationDeletedReason = authenticated_cmds.latest.invite_delete.InvitationDeletedReason

# events_listen
EventsListenReq = authenticated_cmds.latest.events_listen.Req
EventsListenRep = authenticated_cmds.latest.events_listen.Rep
EventsListenRepUnknownStatus = authenticated_cmds.latest.events_listen.RepUnknownStatus
EventsListenRepOk = authenticated_cmds.latest.events_listen.RepOk
EventsListenRepNotAvailable = authenticated_cmds.latest.events_listen.RepNotAvailable
APIEvent = authenticated_cmds.latest.events_listen.APIEvent
APIEventCertificatesUpdated = authenticated_cmds.latest.events_listen.APIEventCertificatesUpdated
APIEventPinged = authenticated_cmds.latest.events_listen.APIEventPinged
APIEventMessageReceived = authenticated_cmds.latest.events_listen.APIEventMessageReceived
APIEventInviteStatusChanged = authenticated_cmds.latest.events_listen.APIEventInviteStatusChanged
APIEventRealmMaintenanceStarted = (
    authenticated_cmds.latest.events_listen.APIEventRealmMaintenanceStarted
)
APIEventRealmMaintenanceFinished = (
    authenticated_cmds.latest.events_listen.APIEventRealmMaintenanceFinished
)
APIEventRealmVlobsUpdated = authenticated_cmds.latest.events_listen.APIEventRealmVlobsUpdated
APIEventPkiEnrollmentUpdated = authenticated_cmds.latest.events_listen.APIEventPkiEnrollmentUpdated

ApiV2V3_EventsListenReq = authenticated_cmds.v3.events_listen.Req
ApiV2V3_EventsListenRep = authenticated_cmds.v3.events_listen.Rep
ApiV2V3_EventsListenRepUnknownStatus = authenticated_cmds.v3.events_listen.RepUnknownStatus
ApiV2V3_EventsListenRepOk = authenticated_cmds.v3.events_listen.RepOk
ApiV2V3_EventsListenRepCancelled = authenticated_cmds.v3.events_listen.RepCancelled
ApiV2V3_EventsListenRepNoEvents = authenticated_cmds.v3.events_listen.RepNoEvents
ApiV2V3_APIEvent = authenticated_cmds.v3.events_listen.APIEvent
ApiV2V3_APIEventPinged = authenticated_cmds.v3.events_listen.APIEventPinged
ApiV2V3_APIEventMessageReceived = authenticated_cmds.v3.events_listen.APIEventMessageReceived
ApiV2V3_APIEventInviteStatusChanged = (
    authenticated_cmds.v3.events_listen.APIEventInviteStatusChanged
)
ApiV2V3_APIEventRealmMaintenanceStarted = (
    authenticated_cmds.v3.events_listen.APIEventRealmMaintenanceStarted
)
ApiV2V3_APIEventRealmMaintenanceFinished = (
    authenticated_cmds.v3.events_listen.APIEventRealmMaintenanceFinished
)
ApiV2V3_APIEventRealmVlobsUpdated = authenticated_cmds.v3.events_listen.APIEventRealmVlobsUpdated
ApiV2V3_APIEventRealmRolesUpdated = authenticated_cmds.v3.events_listen.APIEventRealmRolesUpdated
ApiV2V3_APIEventPkiEnrollmentUpdated = (
    authenticated_cmds.v3.events_listen.APIEventPkiEnrollmentUpdated
)

# vlob_maintenance_get_reencryption_batch
VlobMaintenanceGetReencryptionBatchReq = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.Req
)
VlobMaintenanceGetReencryptionBatchRep = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.Rep
)
VlobMaintenanceGetReencryptionBatchRepUnknownStatus = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepUnknownStatus
)
VlobMaintenanceGetReencryptionBatchRepOk = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepOk
)
VlobMaintenanceGetReencryptionBatchRepNotAllowed = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepNotAllowed
)
VlobMaintenanceGetReencryptionBatchRepNotFound = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepNotFound
)
VlobMaintenanceGetReencryptionBatchRepNotInMaintenance = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepNotInMaintenance
)
VlobMaintenanceGetReencryptionBatchRepBadEncryptionRevision = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepBadEncryptionRevision
)
VlobMaintenanceGetReencryptionBatchRepMaintenanceError = (
    authenticated_cmds.latest.vlob_maintenance_get_reencryption_batch.RepMaintenanceError
)

# realm_start_reencryption_maintenance
RealmStartReencryptionMaintenanceReq = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.Req
)
RealmStartReencryptionMaintenanceRep = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.Rep
)
RealmStartReencryptionMaintenanceRepUnknownStatus = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepUnknownStatus
)
RealmStartReencryptionMaintenanceRepOk = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepOk
)
RealmStartReencryptionMaintenanceRepNotAllowed = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepNotAllowed
)
RealmStartReencryptionMaintenanceRepNotFound = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepNotFound
)
RealmStartReencryptionMaintenanceRepBadEncryptionRevision = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepBadEncryptionRevision
)
RealmStartReencryptionMaintenanceRepParticipantMismatch = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepParticipantMismatch
)
RealmStartReencryptionMaintenanceRepMaintenanceError = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepMaintenanceError
)
RealmStartReencryptionMaintenanceRepInMaintenance = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepInMaintenance
)
RealmStartReencryptionMaintenanceRepBadTimestamp = (
    authenticated_cmds.latest.realm_start_reencryption_maintenance.RepBadTimestamp
)

# vlob_read
VlobReadReq = authenticated_cmds.latest.vlob_read.Req
VlobReadRep = authenticated_cmds.latest.vlob_read.Rep
VlobReadRepUnknownStatus = authenticated_cmds.latest.vlob_read.RepUnknownStatus
VlobReadRepOk = authenticated_cmds.latest.vlob_read.RepOk
VlobReadRepNotFound = authenticated_cmds.latest.vlob_read.RepNotFound
VlobReadRepNotAllowed = authenticated_cmds.latest.vlob_read.RepNotAllowed
VlobReadRepBadVersion = authenticated_cmds.latest.vlob_read.RepBadVersion
VlobReadRepBadEncryptionRevision = authenticated_cmds.latest.vlob_read.RepBadEncryptionRevision
VlobReadRepInMaintenance = authenticated_cmds.latest.vlob_read.RepInMaintenance

# invite_4_greeter_communicate
Invite4GreeterCommunicateReq = authenticated_cmds.latest.invite_4_greeter_communicate.Req
Invite4GreeterCommunicateRep = authenticated_cmds.latest.invite_4_greeter_communicate.Rep
Invite4GreeterCommunicateRepUnknownStatus = (
    authenticated_cmds.latest.invite_4_greeter_communicate.RepUnknownStatus
)
Invite4GreeterCommunicateRepOk = authenticated_cmds.latest.invite_4_greeter_communicate.RepOk
Invite4GreeterCommunicateRepNotFound = (
    authenticated_cmds.latest.invite_4_greeter_communicate.RepNotFound
)
Invite4GreeterCommunicateRepAlreadyDeleted = (
    authenticated_cmds.latest.invite_4_greeter_communicate.RepAlreadyDeleted
)
Invite4GreeterCommunicateRepInvalidState = (
    authenticated_cmds.latest.invite_4_greeter_communicate.RepInvalidState
)

# invite_2b_greeter_send_nonce
Invite2bGreeterSendNonceReq = authenticated_cmds.latest.invite_2b_greeter_send_nonce.Req
Invite2bGreeterSendNonceRep = authenticated_cmds.latest.invite_2b_greeter_send_nonce.Rep
Invite2bGreeterSendNonceRepUnknownStatus = (
    authenticated_cmds.latest.invite_2b_greeter_send_nonce.RepUnknownStatus
)
Invite2bGreeterSendNonceRepOk = authenticated_cmds.latest.invite_2b_greeter_send_nonce.RepOk
Invite2bGreeterSendNonceRepNotFound = (
    authenticated_cmds.latest.invite_2b_greeter_send_nonce.RepNotFound
)
Invite2bGreeterSendNonceRepAlreadyDeleted = (
    authenticated_cmds.latest.invite_2b_greeter_send_nonce.RepAlreadyDeleted
)
Invite2bGreeterSendNonceRepInvalidState = (
    authenticated_cmds.latest.invite_2b_greeter_send_nonce.RepInvalidState
)

# invite_2a_greeter_get_hashed_nonce
Invite2aGreeterGetHashedNonceReq = authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.Req
Invite2aGreeterGetHashedNonceRep = authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.Rep
Invite2aGreeterGetHashedNonceRepUnknownStatus = (
    authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.RepUnknownStatus
)
Invite2aGreeterGetHashedNonceRepOk = (
    authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.RepOk
)
Invite2aGreeterGetHashedNonceRepNotFound = (
    authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.RepNotFound
)
Invite2aGreeterGetHashedNonceRepAlreadyDeleted = (
    authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.RepAlreadyDeleted
)
Invite2aGreeterGetHashedNonceRepInvalidState = (
    authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.RepInvalidState
)

# events_subscribe
ApiV2V3_EventsSubscribeReq = authenticated_cmds.v3.events_subscribe.Req
ApiV2V3_EventsSubscribeRep = authenticated_cmds.v3.events_subscribe.Rep
ApiV2V3_EventsSubscribeRepUnknownStatus = authenticated_cmds.v3.events_subscribe.RepUnknownStatus
ApiV2V3_EventsSubscribeRepOk = authenticated_cmds.v3.events_subscribe.RepOk

# device_create
DeviceCreateReq = authenticated_cmds.latest.device_create.Req
DeviceCreateRep = authenticated_cmds.latest.device_create.Rep
DeviceCreateRepUnknownStatus = authenticated_cmds.latest.device_create.RepUnknownStatus
DeviceCreateRepOk = authenticated_cmds.latest.device_create.RepOk
DeviceCreateRepInvalidCertification = (
    authenticated_cmds.latest.device_create.RepInvalidCertification
)
DeviceCreateRepBadUserId = authenticated_cmds.latest.device_create.RepBadUserId
DeviceCreateRepInvalidData = authenticated_cmds.latest.device_create.RepInvalidData
DeviceCreateRepAlreadyExists = authenticated_cmds.latest.device_create.RepAlreadyExists

# vlob_list_versions
VlobListVersionsReq = authenticated_cmds.latest.vlob_list_versions.Req
VlobListVersionsRep = authenticated_cmds.latest.vlob_list_versions.Rep
VlobListVersionsRepUnknownStatus = authenticated_cmds.latest.vlob_list_versions.RepUnknownStatus
VlobListVersionsRepOk = authenticated_cmds.latest.vlob_list_versions.RepOk
VlobListVersionsRepNotAllowed = authenticated_cmds.latest.vlob_list_versions.RepNotAllowed
VlobListVersionsRepNotFound = authenticated_cmds.latest.vlob_list_versions.RepNotFound
VlobListVersionsRepInMaintenance = authenticated_cmds.latest.vlob_list_versions.RepInMaintenance

# realm_status
RealmStatusReq = authenticated_cmds.latest.realm_status.Req
RealmStatusRep = authenticated_cmds.latest.realm_status.Rep
RealmStatusRepUnknownStatus = authenticated_cmds.latest.realm_status.RepUnknownStatus
RealmStatusRepOk = authenticated_cmds.latest.realm_status.RepOk
RealmStatusRepNotAllowed = authenticated_cmds.latest.realm_status.RepNotAllowed
RealmStatusRepNotFound = authenticated_cmds.latest.realm_status.RepNotFound
MaintenanceType = authenticated_cmds.latest.realm_status.MaintenanceType

# ping
AuthenticatedPingReq = authenticated_cmds.latest.ping.Req
AuthenticatedPingRep = authenticated_cmds.latest.ping.Rep
AuthenticatedPingRepUnknownStatus = authenticated_cmds.latest.ping.RepUnknownStatus
AuthenticatedPingRepOk = authenticated_cmds.latest.ping.RepOk

# vlob_create
VlobCreateReq = authenticated_cmds.latest.vlob_create.Req
VlobCreateRep = authenticated_cmds.latest.vlob_create.Rep
VlobCreateRepUnknownStatus = authenticated_cmds.latest.vlob_create.RepUnknownStatus
VlobCreateRepOk = authenticated_cmds.latest.vlob_create.RepOk
VlobCreateRepAlreadyExists = authenticated_cmds.latest.vlob_create.RepAlreadyExists
VlobCreateRepNotAllowed = authenticated_cmds.latest.vlob_create.RepNotAllowed
VlobCreateRepBadEncryptionRevision = authenticated_cmds.latest.vlob_create.RepBadEncryptionRevision
VlobCreateRepInMaintenance = authenticated_cmds.latest.vlob_create.RepInMaintenance
VlobCreateRepRequireGreaterTimestamp = (
    authenticated_cmds.latest.vlob_create.RepRequireGreaterTimestamp
)
VlobCreateRepBadTimestamp = authenticated_cmds.latest.vlob_create.RepBadTimestamp
VlobCreateRepNotASequesteredOrganization = (
    authenticated_cmds.latest.vlob_create.RepNotASequesteredOrganization
)
VlobCreateRepSequesterInconsistency = (
    authenticated_cmds.latest.vlob_create.RepSequesterInconsistency
)
VlobCreateRepRejectedBySequesterService = (
    authenticated_cmds.latest.vlob_create.RepRejectedBySequesterService
)
VlobCreateRepTimeout = authenticated_cmds.latest.vlob_create.RepTimeout

# organization_config
OrganizationConfigReq = authenticated_cmds.latest.organization_config.Req
OrganizationConfigRep = authenticated_cmds.latest.organization_config.Rep
OrganizationConfigRepUnknownStatus = authenticated_cmds.latest.organization_config.RepUnknownStatus
OrganizationConfigRepOk = authenticated_cmds.latest.organization_config.RepOk
OrganizationConfigRepNotFound = authenticated_cmds.latest.organization_config.RepNotFound

# invite_new
InviteNewReq = authenticated_cmds.latest.invite_new.Req
InviteNewRep = authenticated_cmds.latest.invite_new.Rep
InviteNewRepUnknownStatus = authenticated_cmds.latest.invite_new.RepUnknownStatus
InviteNewRepOk = authenticated_cmds.latest.invite_new.RepOk
InviteNewRepNotAllowed = authenticated_cmds.latest.invite_new.RepNotAllowed
InviteNewRepAlreadyMember = authenticated_cmds.latest.invite_new.RepAlreadyMember
InviteNewRepNotAvailable = authenticated_cmds.latest.invite_new.RepNotAvailable
InvitationEmailSentStatus = authenticated_cmds.latest.invite_new.InvitationEmailSentStatus

# invite_1_greeter_wait_peer
Invite1GreeterWaitPeerReq = authenticated_cmds.latest.invite_1_greeter_wait_peer.Req
Invite1GreeterWaitPeerRep = authenticated_cmds.latest.invite_1_greeter_wait_peer.Rep
Invite1GreeterWaitPeerRepUnknownStatus = (
    authenticated_cmds.latest.invite_1_greeter_wait_peer.RepUnknownStatus
)
Invite1GreeterWaitPeerRepOk = authenticated_cmds.latest.invite_1_greeter_wait_peer.RepOk
Invite1GreeterWaitPeerRepNotFound = authenticated_cmds.latest.invite_1_greeter_wait_peer.RepNotFound
Invite1GreeterWaitPeerRepAlreadyDeleted = (
    authenticated_cmds.latest.invite_1_greeter_wait_peer.RepAlreadyDeleted
)
Invite1GreeterWaitPeerRepInvalidState = (
    authenticated_cmds.latest.invite_1_greeter_wait_peer.RepInvalidState
)

# vlob_poll_changes
VlobPollChangesReq = authenticated_cmds.latest.vlob_poll_changes.Req
VlobPollChangesRep = authenticated_cmds.latest.vlob_poll_changes.Rep
VlobPollChangesRepUnknownStatus = authenticated_cmds.latest.vlob_poll_changes.RepUnknownStatus
VlobPollChangesRepOk = authenticated_cmds.latest.vlob_poll_changes.RepOk
VlobPollChangesRepNotAllowed = authenticated_cmds.latest.vlob_poll_changes.RepNotAllowed
VlobPollChangesRepNotFound = authenticated_cmds.latest.vlob_poll_changes.RepNotFound
VlobPollChangesRepInMaintenance = authenticated_cmds.latest.vlob_poll_changes.RepInMaintenance

# block_read
BlockReadReq = authenticated_cmds.latest.block_read.Req
BlockReadRep = authenticated_cmds.latest.block_read.Rep
BlockReadRepUnknownStatus = authenticated_cmds.latest.block_read.RepUnknownStatus
BlockReadRepOk = authenticated_cmds.latest.block_read.RepOk
BlockReadRepNotFound = authenticated_cmds.latest.block_read.RepNotFound
BlockReadRepTimeout = authenticated_cmds.latest.block_read.RepTimeout
BlockReadRepNotAllowed = authenticated_cmds.latest.block_read.RepNotAllowed
BlockReadRepInMaintenance = authenticated_cmds.latest.block_read.RepInMaintenance

# pki_enrollment_list
PkiEnrollmentListReq = authenticated_cmds.latest.pki_enrollment_list.Req
PkiEnrollmentListRep = authenticated_cmds.latest.pki_enrollment_list.Rep
PkiEnrollmentListRepUnknownStatus = authenticated_cmds.latest.pki_enrollment_list.RepUnknownStatus
PkiEnrollmentListRepOk = authenticated_cmds.latest.pki_enrollment_list.RepOk
PkiEnrollmentListRepNotAllowed = authenticated_cmds.latest.pki_enrollment_list.RepNotAllowed

# message_get
MessageGetReq = authenticated_cmds.latest.message_get.Req
MessageGetRep = authenticated_cmds.latest.message_get.Rep
MessageGetRepUnknownStatus = authenticated_cmds.latest.message_get.RepUnknownStatus
MessageGetRepOk = authenticated_cmds.latest.message_get.RepOk
Message = authenticated_cmds.latest.message_get.Message

# invite_3a_greeter_wait_peer_trust
Invite3aGreeterWaitPeerTrustReq = authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.Req
Invite3aGreeterWaitPeerTrustRep = authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.Rep
Invite3aGreeterWaitPeerTrustRepUnknownStatus = (
    authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.RepUnknownStatus
)
Invite3aGreeterWaitPeerTrustRepOk = (
    authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.RepOk
)
Invite3aGreeterWaitPeerTrustRepNotFound = (
    authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.RepNotFound
)
Invite3aGreeterWaitPeerTrustRepAlreadyDeleted = (
    authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.RepAlreadyDeleted
)
Invite3aGreeterWaitPeerTrustRepInvalidState = (
    authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.RepInvalidState
)

# realm_create
RealmCreateReq = authenticated_cmds.latest.realm_create.Req
RealmCreateRep = authenticated_cmds.latest.realm_create.Rep
RealmCreateRepUnknownStatus = authenticated_cmds.latest.realm_create.RepUnknownStatus
RealmCreateRepOk = authenticated_cmds.latest.realm_create.RepOk
RealmCreateRepInvalidCertification = authenticated_cmds.latest.realm_create.RepInvalidCertification
RealmCreateRepInvalidData = authenticated_cmds.latest.realm_create.RepInvalidData
RealmCreateRepNotFound = authenticated_cmds.latest.realm_create.RepNotFound
RealmCreateRepAlreadyExists = authenticated_cmds.latest.realm_create.RepAlreadyExists
RealmCreateRepBadTimestamp = authenticated_cmds.latest.realm_create.RepBadTimestamp

# user_revoke
UserRevokeReq = authenticated_cmds.latest.user_revoke.Req
UserRevokeRep = authenticated_cmds.latest.user_revoke.Rep
UserRevokeRepUnknownStatus = authenticated_cmds.latest.user_revoke.RepUnknownStatus
UserRevokeRepOk = authenticated_cmds.latest.user_revoke.RepOk
UserRevokeRepNotAllowed = authenticated_cmds.latest.user_revoke.RepNotAllowed
UserRevokeRepInvalidCertification = authenticated_cmds.latest.user_revoke.RepInvalidCertification
UserRevokeRepNotFound = authenticated_cmds.latest.user_revoke.RepNotFound
UserRevokeRepAlreadyRevoked = authenticated_cmds.latest.user_revoke.RepAlreadyRevoked

# user_create
UserCreateReq = authenticated_cmds.latest.user_create.Req
UserCreateRep = authenticated_cmds.latest.user_create.Rep
UserCreateRepUnknownStatus = authenticated_cmds.latest.user_create.RepUnknownStatus
UserCreateRepOk = authenticated_cmds.latest.user_create.RepOk
UserCreateRepNotAllowed = authenticated_cmds.latest.user_create.RepNotAllowed
UserCreateRepInvalidCertification = authenticated_cmds.latest.user_create.RepInvalidCertification
UserCreateRepInvalidData = authenticated_cmds.latest.user_create.RepInvalidData
UserCreateRepAlreadyExists = authenticated_cmds.latest.user_create.RepAlreadyExists
UserCreateRepActiveUsersLimitReached = (
    authenticated_cmds.latest.user_create.RepActiveUsersLimitReached
)

# realm_update_roles
RealmUpdateRolesReq = authenticated_cmds.latest.realm_update_roles.Req
RealmUpdateRolesRep = authenticated_cmds.latest.realm_update_roles.Rep
RealmUpdateRolesRepUnknownStatus = authenticated_cmds.latest.realm_update_roles.RepUnknownStatus
RealmUpdateRolesRepOk = authenticated_cmds.latest.realm_update_roles.RepOk
RealmUpdateRolesRepNotAllowed = authenticated_cmds.latest.realm_update_roles.RepNotAllowed
RealmUpdateRolesRepInvalidCertification = (
    authenticated_cmds.latest.realm_update_roles.RepInvalidCertification
)
RealmUpdateRolesRepInvalidData = authenticated_cmds.latest.realm_update_roles.RepInvalidData
RealmUpdateRolesRepAlreadyGranted = authenticated_cmds.latest.realm_update_roles.RepAlreadyGranted
RealmUpdateRolesRepIncompatibleProfile = (
    authenticated_cmds.latest.realm_update_roles.RepIncompatibleProfile
)
RealmUpdateRolesRepNotFound = authenticated_cmds.latest.realm_update_roles.RepNotFound
RealmUpdateRolesRepInMaintenance = authenticated_cmds.latest.realm_update_roles.RepInMaintenance
RealmUpdateRolesRepUserRevoked = authenticated_cmds.latest.realm_update_roles.RepUserRevoked
RealmUpdateRolesRepRequireGreaterTimestamp = (
    authenticated_cmds.latest.realm_update_roles.RepRequireGreaterTimestamp
)
RealmUpdateRolesRepBadTimestamp = authenticated_cmds.latest.realm_update_roles.RepBadTimestamp

#
# Invited cmds
#

# invite_1_claimer_wait_peer
Invite1ClaimerWaitPeerReq = invited_cmds.latest.invite_1_claimer_wait_peer.Req
Invite1ClaimerWaitPeerRep = invited_cmds.latest.invite_1_claimer_wait_peer.Rep
Invite1ClaimerWaitPeerRepUnknownStatus = (
    invited_cmds.latest.invite_1_claimer_wait_peer.RepUnknownStatus
)
Invite1ClaimerWaitPeerRepOk = invited_cmds.latest.invite_1_claimer_wait_peer.RepOk
Invite1ClaimerWaitPeerRepAlreadyDeleted = (
    invited_cmds.latest.invite_1_claimer_wait_peer.RepAlreadyDeleted
)
Invite1ClaimerWaitPeerRepNotFound = invited_cmds.latest.invite_1_claimer_wait_peer.RepNotFound
Invite1ClaimerWaitPeerRepInvalidState = (
    invited_cmds.latest.invite_1_claimer_wait_peer.RepInvalidState
)

# invite_3a_claimer_signify_trust
Invite3aClaimerSignifyTrustReq = invited_cmds.latest.invite_3a_claimer_signify_trust.Req
Invite3aClaimerSignifyTrustRep = invited_cmds.latest.invite_3a_claimer_signify_trust.Rep
Invite3aClaimerSignifyTrustRepUnknownStatus = (
    invited_cmds.latest.invite_3a_claimer_signify_trust.RepUnknownStatus
)
Invite3aClaimerSignifyTrustRepOk = invited_cmds.latest.invite_3a_claimer_signify_trust.RepOk
Invite3aClaimerSignifyTrustRepAlreadyDeleted = (
    invited_cmds.latest.invite_3a_claimer_signify_trust.RepAlreadyDeleted
)
Invite3aClaimerSignifyTrustRepNotFound = (
    invited_cmds.latest.invite_3a_claimer_signify_trust.RepNotFound
)
Invite3aClaimerSignifyTrustRepInvalidState = (
    invited_cmds.latest.invite_3a_claimer_signify_trust.RepInvalidState
)

# invite_info
InviteInfoReq = invited_cmds.latest.invite_info.Req
InviteInfoRep = invited_cmds.latest.invite_info.Rep
InviteInfoRepOk = invited_cmds.latest.invite_info.RepOk
InviteInfoRepUnknownStatus = invited_cmds.latest.invite_info.RepUnknownStatus
UserOrDevice = invited_cmds.latest.invite_info.UserOrDevice
UserOrDeviceUser = invited_cmds.latest.invite_info.UserOrDeviceUser
UserOrDeviceDevice = invited_cmds.latest.invite_info.UserOrDeviceDevice

# invite_2a_claimer_send_hashed_nonce
Invite2aClaimerSendHashedNonceReq = invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.Req
Invite2aClaimerSendHashedNonceRep = invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.Rep
Invite2aClaimerSendHashedNonceRepUnknownStatus = (
    invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.RepUnknownStatus
)
Invite2aClaimerSendHashedNonceRepOk = invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.RepOk
Invite2aClaimerSendHashedNonceRepNotFound = (
    invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.RepNotFound
)
Invite2aClaimerSendHashedNonceRepAlreadyDeleted = (
    invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.RepAlreadyDeleted
)
Invite2aClaimerSendHashedNonceRepInvalidState = (
    invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.RepInvalidState
)

# invite_2b_claimer_send_nonce
Invite2bClaimerSendNonceReq = invited_cmds.latest.invite_2b_claimer_send_nonce.Req
Invite2bClaimerSendNonceRep = invited_cmds.latest.invite_2b_claimer_send_nonce.Rep
Invite2bClaimerSendNonceRepUnknownStatus = (
    invited_cmds.latest.invite_2b_claimer_send_nonce.RepUnknownStatus
)
Invite2bClaimerSendNonceRepOk = invited_cmds.latest.invite_2b_claimer_send_nonce.RepOk
Invite2bClaimerSendNonceRepAlreadyDeleted = (
    invited_cmds.latest.invite_2b_claimer_send_nonce.RepAlreadyDeleted
)
Invite2bClaimerSendNonceRepNotFound = invited_cmds.latest.invite_2b_claimer_send_nonce.RepNotFound
Invite2bClaimerSendNonceRepInvalidState = (
    invited_cmds.latest.invite_2b_claimer_send_nonce.RepInvalidState
)

# invite_3b_claimer_wait_peer_trust
Invite3bClaimerWaitPeerTrustReq = invited_cmds.latest.invite_3b_claimer_wait_peer_trust.Req
Invite3bClaimerWaitPeerTrustRep = invited_cmds.latest.invite_3b_claimer_wait_peer_trust.Rep
Invite3bClaimerWaitPeerTrustRepUnknownStatus = (
    invited_cmds.latest.invite_3b_claimer_wait_peer_trust.RepUnknownStatus
)
Invite3bClaimerWaitPeerTrustRepOk = invited_cmds.latest.invite_3b_claimer_wait_peer_trust.RepOk
Invite3bClaimerWaitPeerTrustRepAlreadyDeleted = (
    invited_cmds.latest.invite_3b_claimer_wait_peer_trust.RepAlreadyDeleted
)
Invite3bClaimerWaitPeerTrustRepNotFound = (
    invited_cmds.latest.invite_3b_claimer_wait_peer_trust.RepNotFound
)
Invite3bClaimerWaitPeerTrustRepInvalidState = (
    invited_cmds.latest.invite_3b_claimer_wait_peer_trust.RepInvalidState
)

# ping
InvitedPingReq = invited_cmds.latest.ping.Req
InvitedPingRep = invited_cmds.latest.ping.Rep
InvitedPingRepUnknownStatus = invited_cmds.latest.ping.RepUnknownStatus
InvitedPingRepOk = invited_cmds.latest.ping.RepOk

# invite_4_claimer_communicate
Invite4ClaimerCommunicateReq = invited_cmds.latest.invite_4_claimer_communicate.Req
Invite4ClaimerCommunicateRep = invited_cmds.latest.invite_4_claimer_communicate.Rep
Invite4ClaimerCommunicateRepUnknownStatus = (
    invited_cmds.latest.invite_4_claimer_communicate.RepUnknownStatus
)
Invite4ClaimerCommunicateRepOk = invited_cmds.latest.invite_4_claimer_communicate.RepOk
Invite4ClaimerCommunicateRepAlreadyDeleted = (
    invited_cmds.latest.invite_4_claimer_communicate.RepAlreadyDeleted
)
Invite4ClaimerCommunicateRepNotFound = invited_cmds.latest.invite_4_claimer_communicate.RepNotFound
Invite4ClaimerCommunicateRepInvalidState = (
    invited_cmds.latest.invite_4_claimer_communicate.RepInvalidState
)

#
# Anonymous cmds
#

# pki_enrollment_submit
PkiEnrollmentSubmitReq = anonymous_cmds.latest.pki_enrollment_submit.Req
PkiEnrollmentSubmitRep = anonymous_cmds.latest.pki_enrollment_submit.Rep
PkiEnrollmentSubmitRepUnknownStatus = anonymous_cmds.latest.pki_enrollment_submit.RepUnknownStatus
PkiEnrollmentSubmitRepOk = anonymous_cmds.latest.pki_enrollment_submit.RepOk
PkiEnrollmentSubmitRepAlreadySubmitted = (
    anonymous_cmds.latest.pki_enrollment_submit.RepAlreadySubmitted
)
PkiEnrollmentSubmitRepIdAlreadyUsed = anonymous_cmds.latest.pki_enrollment_submit.RepIdAlreadyUsed
PkiEnrollmentSubmitRepEmailAlreadyUsed = (
    anonymous_cmds.latest.pki_enrollment_submit.RepEmailAlreadyUsed
)
PkiEnrollmentSubmitRepAlreadyEnrolled = (
    anonymous_cmds.latest.pki_enrollment_submit.RepAlreadyEnrolled
)
PkiEnrollmentSubmitRepInvalidPayloadData = (
    anonymous_cmds.latest.pki_enrollment_submit.RepInvalidPayloadData
)

# organization_bootstrap
OrganizationBootstrapReq = anonymous_cmds.latest.organization_bootstrap.Req
OrganizationBootstrapRep = anonymous_cmds.latest.organization_bootstrap.Rep
OrganizationBootstrapRepUnknownStatus = (
    anonymous_cmds.latest.organization_bootstrap.RepUnknownStatus
)
OrganizationBootstrapRepOk = anonymous_cmds.latest.organization_bootstrap.RepOk
OrganizationBootstrapRepInvalidCertification = (
    anonymous_cmds.latest.organization_bootstrap.RepInvalidCertification
)
OrganizationBootstrapRepInvalidData = anonymous_cmds.latest.organization_bootstrap.RepInvalidData
OrganizationBootstrapRepBadTimestamp = anonymous_cmds.latest.organization_bootstrap.RepBadTimestamp
OrganizationBootstrapRepAlreadyBootstrapped = (
    anonymous_cmds.latest.organization_bootstrap.RepAlreadyBootstrapped
)
OrganizationBootstrapRepNotFound = anonymous_cmds.latest.organization_bootstrap.RepNotFound

# pki_enrollment_info
PkiEnrollmentInfoReq = anonymous_cmds.latest.pki_enrollment_info.Req
PkiEnrollmentInfoRep = anonymous_cmds.latest.pki_enrollment_info.Rep
PkiEnrollmentInfoRepUnknownStatus = anonymous_cmds.latest.pki_enrollment_info.RepUnknownStatus
PkiEnrollmentInfoRepOk = anonymous_cmds.latest.pki_enrollment_info.RepOk
PkiEnrollmentInfoRepNotFound = anonymous_cmds.latest.pki_enrollment_info.RepNotFound
PkiEnrollmentInfoStatus = anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatus
PkiEnrollmentInfoStatusAccepted = (
    anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusAccepted
)
PkiEnrollmentInfoStatusCancelled = (
    anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusCancelled
)
PkiEnrollmentInfoStatusRejected = (
    anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusRejected
)
PkiEnrollmentInfoStatusSubmitted = (
    anonymous_cmds.latest.pki_enrollment_info.PkiEnrollmentInfoStatusSubmitted
)
