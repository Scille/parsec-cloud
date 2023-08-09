# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Optional

from .common import (
    BackendInvitationAddr,
    BackendOrganizationBootstrapAddr,
    DateTime,
    DeviceID,
    DeviceLabel,
    ErrorVariant,
    Handle,
    HumanHandle,
    InvitationToken,
    OrganizationID,
    Password,
    Path,
    Ref,
    Result,
    SASCode,
    SequesterVerifyKeyDer,
    Structure,
    UserID,
    UserProfile,
    Variant,
    VariantItemUnit,
)
from .config import ClientConfig
from .events import OnClientEventCallback


class ClaimerGreeterAbortOperationError(ErrorVariant):
    class Internal:
        pass


def claimer_greeter_abort_operation(
    handle: Handle,
) -> Result[None, ClaimerGreeterAbortOperationError]:
    ...


class DeviceFileType(Variant):
    class Password:
        pass

    class Recovery:
        pass

    class Smartcard:
        pass


class AvailableDevice(Structure):
    key_file_path: Path
    organization_id: OrganizationID
    device_id: DeviceID
    human_handle: Optional[HumanHandle]
    device_label: Optional[DeviceLabel]
    slug: str
    ty: DeviceFileType


async def list_available_devices(path: Ref[Path]) -> list[AvailableDevice]:
    ...


class DeviceSaveStrategy(Variant):
    class Password:
        password: Password

    class Smartcard:
        pass


#
# Bootstrap organization
#


class BootstrapOrganizationError(ErrorVariant):
    class Offline:
        pass

    class InvalidToken:
        pass

    class AlreadyUsedToken:
        pass

    class BadTimestamp:
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
    bootstrap_organization_addr: BackendOrganizationBootstrapAddr,
    save_strategy: DeviceSaveStrategy,
    human_handle: Optional[HumanHandle],
    device_label: Optional[DeviceLabel],
    sequester_authority_verify_key: Optional[SequesterVerifyKeyDer],
) -> Result[AvailableDevice, BootstrapOrganizationError]:
    ...


#
# Invitation claimer
#


class ClaimerRetrieveInfoError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class AlreadyUsed:
        pass

    class Internal:
        pass


class ClaimInProgressError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class AlreadyUsed:
        pass

    class PeerReset:
        pass

    class ActiveUsersLimitReached:
        pass

    class CorruptedConfirmation:
        pass

    class Internal:
        pass

    class Cancelled:
        pass


class UserOrDeviceClaimInitialInfo(Variant):
    class User:
        handle: Handle
        claimer_email: str
        greeter_user_id: UserID
        greeter_human_handle: Optional[HumanHandle]

    class Device:
        handle: Handle
        greeter_user_id: UserID
        greeter_human_handle: Optional[HumanHandle]


async def claimer_retrieve_info(
    config: ClientConfig,
    on_event_callback: OnClientEventCallback,
    addr: BackendInvitationAddr,
) -> Result[UserOrDeviceClaimInitialInfo, ClaimerRetrieveInfoError]:
    ...


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
    ...


async def claimer_device_initial_do_wait_peer(
    canceller: Handle,
    handle: Handle,
) -> Result[DeviceClaimInProgress1Info, ClaimInProgressError]:
    ...


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
    ...


async def claimer_device_in_progress_1_do_signify_trust(
    canceller: Handle,
    handle: Handle,
) -> Result[DeviceClaimInProgress2Info, ClaimInProgressError]:
    ...


class UserClaimInProgress3Info(Structure):
    handle: Handle


class DeviceClaimInProgress3Info(Structure):
    handle: Handle


async def claimer_user_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result[UserClaimInProgress3Info, ClaimInProgressError]:
    ...


async def claimer_device_in_progress_2_do_wait_peer_trust(
    canceller: Handle,
    handle: Handle,
) -> Result[DeviceClaimInProgress3Info, ClaimInProgressError]:
    ...


class UserClaimFinalizeInfo(Structure):
    handle: Handle


class DeviceClaimFinalizeInfo(Structure):
    handle: Handle


async def claimer_user_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: Optional[DeviceLabel],
    requested_human_handle: Optional[HumanHandle],
) -> Result[UserClaimFinalizeInfo, ClaimInProgressError]:
    ...


async def claimer_device_in_progress_3_do_claim(
    canceller: Handle,
    handle: Handle,
    requested_device_label: Optional[DeviceLabel],
) -> Result[DeviceClaimFinalizeInfo, ClaimInProgressError]:
    ...


async def claimer_user_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result[AvailableDevice, ClaimInProgressError]:
    ...


async def claimer_device_finalize_save_local_device(
    handle: Handle,
    save_strategy: DeviceSaveStrategy,
) -> Result[AvailableDevice, ClaimInProgressError]:
    ...


#
# Invitation greeter
#


class InvitationStatus(Variant):
    Idle = VariantItemUnit
    Ready = VariantItemUnit
    Deleted = VariantItemUnit


class InvitationEmailSentStatus(Variant):
    Success = VariantItemUnit
    NotAvailable = VariantItemUnit
    BadRecipient = VariantItemUnit


class NewUserInvitationError(ErrorVariant):
    class Offline:
        pass

    class NotAllowed:
        pass

    class AlreadyMember:
        pass

    class Internal:
        pass


async def client_new_user_invitation(
    client: Handle,
    claimer_email: str,
    send_email: bool,
) -> Result[tuple[InvitationToken, InvitationEmailSentStatus], NewUserInvitationError,]:
    ...


class NewDeviceInvitationError(ErrorVariant):
    class Offline:
        pass

    class SendEmailToUserWithoutEmail:
        pass

    class Internal:
        pass


async def client_new_device_invitation(
    client: Handle,
    send_email: bool,
) -> Result[tuple[InvitationToken, InvitationEmailSentStatus], NewDeviceInvitationError,]:
    ...


class DeleteInvitationError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class AlreadyDeleted:
        pass

    class Internal:
        pass


async def client_delete_invitation(
    client: Handle,
    token: InvitationToken,
) -> Result[None, DeleteInvitationError]:
    ...


class InviteListItem(Variant):
    class User:
        token: InvitationToken
        created_on: DateTime
        claimer_email: str
        status: InvitationStatus

    class Device:
        token: InvitationToken
        created_on: DateTime
        status: InvitationStatus


class ListInvitationsError(ErrorVariant):
    class Offline:
        pass

    class Internal:
        pass


async def client_list_invitations(
    client: Handle,
) -> Result[list[InviteListItem], ListInvitationsError]:
    ...


class UserGreetInitialInfo(Structure):
    handle: Handle


class DeviceGreetInitialInfo(Structure):
    handle: Handle


class ClientStartInvitationGreetError(ErrorVariant):
    class Internal:
        pass


async def client_start_user_invitation_greet(
    client: Handle, token: InvitationToken
) -> Result[UserGreetInitialInfo, ClientStartInvitationGreetError]:
    ...


async def client_start_device_invitation_greet(
    client: Handle, token: InvitationToken
) -> Result[DeviceGreetInitialInfo, ClientStartInvitationGreetError]:
    ...


class GreetInProgressError(ErrorVariant):
    class Offline:
        pass

    class NotFound:
        pass

    class AlreadyUsed:
        pass

    class PeerReset:
        pass

    class ActiveUsersLimitReached:
        pass

    class NonceMismatch:
        pass

    class UserAlreadyExists:
        pass

    class DeviceAlreadyExists:
        pass

    class UserCreateNotAllowed:
        pass

    class CorruptedInviteUserData:
        pass

    class BadTimestamp:
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


async def greeter_user_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> Result[UserGreetInProgress1Info, GreetInProgressError]:
    ...


async def greeter_device_initial_do_wait_peer(
    canceller: Handle, handle: Handle
) -> Result[DeviceGreetInProgress1Info, GreetInProgressError]:
    ...


class UserGreetInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode
    claimer_sas_choices: list[SASCode]


class DeviceGreetInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode
    claimer_sas_choices: list[SASCode]


async def greeter_user_in_progress_1_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> Result[UserGreetInProgress2Info, GreetInProgressError]:
    ...


async def greeter_device_in_progress_1_do_wait_peer_trust(
    canceller: Handle, handle: Handle
) -> Result[DeviceGreetInProgress2Info, GreetInProgressError]:
    ...


class UserGreetInProgress3Info(Structure):
    handle: Handle


class DeviceGreetInProgress3Info(Structure):
    handle: Handle


async def greeter_user_in_progress_2_do_signify_trust(
    canceller: Handle, handle: Handle
) -> Result[UserGreetInProgress3Info, GreetInProgressError]:
    ...


async def greeter_device_in_progress_2_do_signify_trust(
    canceller: Handle, handle: Handle
) -> Result[DeviceGreetInProgress3Info, GreetInProgressError]:
    ...


class UserGreetInProgress4Info(Structure):
    handle: Handle
    requested_human_handle: Optional[HumanHandle]
    requested_device_label: Optional[DeviceLabel]


class DeviceGreetInProgress4Info(Structure):
    handle: Handle
    requested_device_label: Optional[DeviceLabel]


async def greeter_user_in_progress_3_do_get_claim_requests(
    canceller: Handle, handle: Handle
) -> Result[UserGreetInProgress4Info, GreetInProgressError]:
    ...


async def greeter_device_in_progress_3_do_get_claim_requests(
    canceller: Handle, handle: Handle
) -> Result[DeviceGreetInProgress4Info, GreetInProgressError]:
    ...


async def greeter_user_in_progress_4_do_create(
    canceller: Handle,
    handle: Handle,
    human_handle: Optional[HumanHandle],
    device_label: Optional[DeviceLabel],
    profile: UserProfile,
) -> Result[None, GreetInProgressError]:
    ...


async def greeter_device_in_progress_4_do_create(
    canceller: Handle,
    handle: Handle,
    device_label: Optional[DeviceLabel],
) -> Result[None, GreetInProgressError]:
    ...
