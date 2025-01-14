# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Optional

from .addr import (
    ParsecInvitationAddr,
    ParsecOrganizationBootstrapAddr,
)
from .common import (
    DateTime,
    DeviceLabel,
    Enum,
    EnumItemUnit,
    ErrorVariant,
    Handle,
    HumanHandle,
    InvitationStatus,
    InvitationToken,
    Result,
    SASCode,
    SequesterVerifyKeyDer,
    Structure,
    UserID,
    UserProfile,
    Variant,
)
from .config import ClientConfig
from .device import AvailableDevice, DeviceSaveStrategy
from .events import OnClientEventCallback


class ClaimerGreeterAbortOperationError(ErrorVariant):
    class Internal:
        pass


async def claimer_greeter_abort_operation(
    handle: Handle,
) -> Result[None, ClaimerGreeterAbortOperationError]:
    raise NotImplementedError


#
# Bootstrap organization
#


class BootstrapOrganizationError(ErrorVariant):
    class Offline:
        pass

    class OrganizationExpired:
        pass

    class InvalidToken:
        pass

    class AlreadyUsedToken:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class SaveDeviceError:
        pass

    class Internal:
        pass


async def bootstrap_organization(
    config: ClientConfig,
    on_event_callback: OnClientEventCallback,
    bootstrap_organization_addr: ParsecOrganizationBootstrapAddr,
    save_strategy: DeviceSaveStrategy,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    sequester_authority_verify_key: Optional[SequesterVerifyKeyDer],
) -> Result[AvailableDevice, BootstrapOrganizationError]:
    raise NotImplementedError


#
# Invitation claimer
#


class ClaimerRetrieveInfoError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class AlreadyUsedOrDeleted:
        pass

    class OrganizationExpired:
        pass

    class Internal:
        pass


class GreeterOrClaimer(Enum):
    Greeter = EnumItemUnit
    Claimer = EnumItemUnit


class CancelledGreetingAttemptReason(Enum):
    ManuallyCancelled = EnumItemUnit
    InvalidNonceHash = EnumItemUnit
    InvalidSasCode = EnumItemUnit
    UndecipherablePayload = EnumItemUnit
    UndeserializablePayload = EnumItemUnit
    InconsistentPayload = EnumItemUnit
    AutomaticallyCancelled = EnumItemUnit


class ClaimInProgressError(ErrorVariant):
    class Offline:
        pass

    class OrganizationExpired:
        pass

    class NotFound:
        pass

    class AlreadyUsedOrDeleted:
        pass

    class PeerReset:
        pass

    class ActiveUsersLimitReached:
        pass

    class GreeterNotAllowed:
        pass

    class GreetingAttemptCancelled:
        origin: GreeterOrClaimer
        reason: CancelledGreetingAttemptReason
        timestamp: DateTime

    class CorruptedConfirmation:
        pass

    class Internal:
        pass

    class Cancelled:
        pass


class AnyClaimRetrievedInfo(Variant):
    class User:
        handle: Handle
        claimer_email: str
        greeter_user_id: UserID
        greeter_human_handle: HumanHandle

    class Device:
        handle: Handle
        greeter_user_id: UserID
        greeter_human_handle: HumanHandle


async def claimer_retrieve_info(
    config: ClientConfig,
    on_event_callback: OnClientEventCallback,
    addr: ParsecInvitationAddr,
) -> Result[AnyClaimRetrievedInfo, ClaimerRetrieveInfoError]:
    raise NotImplementedError


class UserClaimInProgress1Info(Structure):
    handle: Handle
    greeter_sas: SASCode
    greeter_sas_choices: list[SASCode]


class DeviceClaimInProgress1Info(Structure):
    handle: Handle
    greeter_sas: SASCode
    greeter_sas_choices: list[SASCode]


async def claimer_user_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result[UserClaimInProgress1Info, ClaimInProgressError]:
    raise NotImplementedError


async def claimer_device_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result[DeviceClaimInProgress1Info, ClaimInProgressError]:
    raise NotImplementedError


async def claimer_user_in_progress_1_do_deny_trust(
    canceller: Handle, handle: Handle
) -> Result[None, ClaimInProgressError]:
    raise NotImplementedError


async def claimer_device_in_progress_1_do_deny_trust(
    canceller: Handle, handle: Handle
) -> Result[None, ClaimInProgressError]:
    raise NotImplementedError


class UserClaimInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode


class DeviceClaimInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode


async def claimer_user_in_progress_1_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result[UserClaimInProgress2Info, ClaimInProgressError]:
    raise NotImplementedError


async def claimer_device_in_progress_1_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result[DeviceClaimInProgress2Info, ClaimInProgressError]:
    raise NotImplementedError


class UserClaimInProgress3Info(Structure):
    handle: Handle


class DeviceClaimInProgress3Info(Structure):
    handle: Handle


async def claimer_user_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result[UserClaimInProgress3Info, ClaimInProgressError]:
    raise NotImplementedError


async def claimer_device_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result[DeviceClaimInProgress3Info, ClaimInProgressError]:
    raise NotImplementedError


class UserClaimFinalizeInfo(Structure):
    handle: Handle


class DeviceClaimFinalizeInfo(Structure):
    handle: Handle


async def claimer_user_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: DeviceLabel,
    requested_human_handle: HumanHandle,
) -> Result[UserClaimFinalizeInfo, ClaimInProgressError]:
    raise NotImplementedError


async def claimer_device_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: DeviceLabel,
) -> Result[DeviceClaimFinalizeInfo, ClaimInProgressError]:
    raise NotImplementedError


async def claimer_user_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result[AvailableDevice, ClaimInProgressError]:
    raise NotImplementedError


async def claimer_device_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result[AvailableDevice, ClaimInProgressError]:
    raise NotImplementedError


#
# Invitation greeter
#


class InvitationEmailSentStatus(Enum):
    Success = EnumItemUnit
    ServerUnavailable = EnumItemUnit
    RecipientRefused = EnumItemUnit


class ClientNewUserInvitationError(ErrorVariant):
    class Offline:
        pass

    class NotAllowed:
        pass

    class AlreadyMember:
        pass

    class Internal:
        pass


class NewInvitationInfo(Structure):
    addr: ParsecInvitationAddr
    token: InvitationToken
    email_sent_status: InvitationEmailSentStatus


async def client_new_user_invitation(
    client: Handle,
    claimer_email: str,
    send_email: bool,
) -> Result[
    NewInvitationInfo,
    ClientNewUserInvitationError,
]:
    raise NotImplementedError


class ClientNewDeviceInvitationError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def client_new_device_invitation(
    client: Handle,
    send_email: bool,
) -> Result[
    NewInvitationInfo,
    ClientNewDeviceInvitationError,
]:
    raise NotImplementedError


class ClientNewShamirRecoveryInvitationError(ErrorVariant):
    class Offline:
        pass

    class NotAllowed:
        pass

    class UserNotFound:
        pass

    class Internal:
        pass


async def client_new_shamir_recovery_invitation(
    client: Handle,
    claimer_user_id: UserID,
    send_email: bool,
) -> Result[
    NewInvitationInfo,
    ClientNewShamirRecoveryInvitationError,
]:
    raise NotImplementedError


class ClientCancelInvitationError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class AlreadyDeleted:
        pass

    class Internal:
        pass


async def client_cancel_invitation(
    client: Handle,
    token: InvitationToken,
) -> Result[None, ClientCancelInvitationError]:
    raise NotImplementedError


class InviteListItem(Variant):
    class User:
        addr: ParsecInvitationAddr
        token: InvitationToken
        created_on: DateTime
        claimer_email: str
        status: InvitationStatus

    class Device:
        addr: ParsecInvitationAddr
        token: InvitationToken
        created_on: DateTime
        status: InvitationStatus

    class ShamirRecovery:
        addr: ParsecInvitationAddr
        token: InvitationToken
        created_on: DateTime
        claimer_user_id: UserID
        shamir_recovery_created_on: DateTime
        status: InvitationStatus


class ListInvitationsError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def client_list_invitations(
    client: Handle,
) -> Result[list[InviteListItem], ListInvitationsError]:
    raise NotImplementedError


class UserGreetInitialInfo(Structure):
    handle: Handle


class DeviceGreetInitialInfo(Structure):
    handle: Handle


class ShamirRecoveryGreetInitialInfo(Structure):
    handle: Handle


class ClientStartInvitationGreetError(ErrorVariant):
    class Internal:
        pass


class ClientStartShamirRecoveryInvitationGreetError(ErrorVariant):
    class InvitationNotFound:
        pass

    class ShamirRecoveryNotFound:
        pass

    class ShamirRecoveryDeleted:
        pass

    class ShamirRecoveryUnusable:
        pass

    class CorruptedShareData:
        pass

    class InvalidCertificate:
        pass

    class Offline:
        pass

    class Stopped:
        pass

    class Internal:
        pass


async def client_start_user_invitation_greet(
    client: Handle, token: InvitationToken
) -> Result[UserGreetInitialInfo, ClientStartInvitationGreetError]:
    raise NotImplementedError


async def client_start_device_invitation_greet(
    client: Handle, token: InvitationToken
) -> Result[DeviceGreetInitialInfo, ClientStartInvitationGreetError]:
    raise NotImplementedError


async def client_start_shamir_recovery_invitation_greet(
    client: Handle, token: InvitationToken
) -> Result[ShamirRecoveryGreetInitialInfo, ClientStartShamirRecoveryInvitationGreetError]:
    raise NotImplementedError


class GreetInProgressError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class AlreadyDeleted:
        pass

    class PeerReset:
        pass

    class ActiveUsersLimitReached:
        pass

    class NonceMismatch:
        pass

    class HumanHandleAlreadyTaken:
        pass

    class UserAlreadyExists:
        pass

    class DeviceAlreadyExists:
        pass

    class UserCreateNotAllowed:
        pass

    class GreeterNotAllowed:
        pass

    class GreetingAttemptCancelled:
        origin: GreeterOrClaimer
        reason: CancelledGreetingAttemptReason
        timestamp: DateTime

    class CorruptedInviteUserData:
        pass

    class TimestampOutOfBallpark:
        server_timestamp: DateTime
        client_timestamp: DateTime
        ballpark_client_early_offset: float
        ballpark_client_late_offset: float

    class Internal:
        pass

    class Cancelled:
        pass


class UserGreetInProgress1Info(Structure):
    handle: Handle
    greeter_sas: SASCode


class DeviceGreetInProgress1Info(Structure):
    handle: Handle
    greeter_sas: SASCode


class ShamirRecoveryGreetInProgress1Info(Structure):
    handle: Handle
    greeter_sas: SASCode


async def greeter_user_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> Result[UserGreetInProgress1Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_device_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> Result[DeviceGreetInProgress1Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_shamir_recovery_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> Result[ShamirRecoveryGreetInProgress1Info, GreetInProgressError]:
    raise NotImplementedError


class UserGreetInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode
    claimer_sas_choices: list[SASCode]


class DeviceGreetInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode
    claimer_sas_choices: list[SASCode]


class ShamirRecoveryGreetInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode
    claimer_sas_choices: list[SASCode]


async def greeter_user_in_progress_1_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> Result[UserGreetInProgress2Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_device_in_progress_1_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> Result[DeviceGreetInProgress2Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_shamir_recovery_in_progress_1_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> Result[ShamirRecoveryGreetInProgress2Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_user_in_progress_2_do_deny_trust(
    canceller: Handle, handle: Handle
) -> Result[None, GreetInProgressError]:
    raise NotImplementedError


async def greeter_device_in_progress_2_do_deny_trust(
    canceller: Handle, handle: Handle
) -> Result[None, GreetInProgressError]:
    raise NotImplementedError


async def greeter_shamir_recovery_in_progress_2_do_deny_trust(
    canceller: Handle, handle: Handle
) -> Result[None, GreetInProgressError]:
    raise NotImplementedError


class UserGreetInProgress3Info(Structure):
    handle: Handle


class DeviceGreetInProgress3Info(Structure):
    handle: Handle


class ShamirRecoveryGreetInProgress3Info(Structure):
    handle: Handle


async def greeter_user_in_progress_2_do_signify_trust(
    canceller: Handle, handle: Handle
) -> Result[UserGreetInProgress3Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_device_in_progress_2_do_signify_trust(
    canceller: Handle, handle: Handle
) -> Result[DeviceGreetInProgress3Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_shamir_recovery_in_progress_2_do_signify_trust(
    canceller: Handle, handle: Handle
) -> Result[ShamirRecoveryGreetInProgress3Info, GreetInProgressError]:
    raise NotImplementedError


class UserGreetInProgress4Info(Structure):
    handle: Handle
    requested_human_handle: HumanHandle
    requested_device_label: DeviceLabel


class DeviceGreetInProgress4Info(Structure):
    handle: Handle
    requested_device_label: DeviceLabel


async def greeter_user_in_progress_3_do_get_claim_requests(
    canceller: Handle, handle: Handle
) -> Result[UserGreetInProgress4Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_device_in_progress_3_do_get_claim_requests(
    canceller: Handle, handle: Handle
) -> Result[DeviceGreetInProgress4Info, GreetInProgressError]:
    raise NotImplementedError


async def greeter_shamir_recovery_in_progress_3_do_get_claim_requests(
    canceller: Handle, handle: Handle
) -> Result[None, GreetInProgressError]:
    raise NotImplementedError


async def greeter_user_in_progress_4_do_create(
    canceller: Handle,
    handle: Handle,
    human_handle: HumanHandle,
    device_label: DeviceLabel,
    profile: UserProfile,
) -> Result[None, GreetInProgressError]:
    raise NotImplementedError


async def greeter_device_in_progress_4_do_create(
    canceller: Handle,
    handle: Handle,
    device_label: DeviceLabel,
) -> Result[None, GreetInProgressError]:
    raise NotImplementedError
