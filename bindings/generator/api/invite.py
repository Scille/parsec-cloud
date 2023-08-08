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
    OrganizationID,
    Password,
    Path,
    Ref,
    Result,
    SASCode,
    SequesterVerifyKeyDer,
    Structure,
    UserID,
    Variant,
)
from .config import ClientConfig
from .events import OnClientEventCallback


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


async def claimer_user_initial_ctx_do_wait_peer(
    handle: Handle,
) -> Result[UserClaimInProgress1Info, ClaimInProgressError]:
    ...


async def claimer_device_initial_ctx_do_wait_peer(
    handle: Handle,
) -> Result[DeviceClaimInProgress1Info, ClaimInProgressError]:
    ...


class UserClaimInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode


class DeviceClaimInProgress2Info(Structure):
    handle: Handle
    claimer_sas: SASCode


async def claimer_user_in_progress_2_do_signify_trust(
    handle: Handle,
) -> Result[UserClaimInProgress2Info, ClaimInProgressError]:
    ...


async def claimer_device_in_progress_2_do_signify_trust(
    handle: Handle,
) -> Result[DeviceClaimInProgress2Info, ClaimInProgressError]:
    ...


class UserClaimInProgress3Info(Structure):
    handle: Handle


class DeviceClaimInProgress3Info(Structure):
    handle: Handle


async def claimer_user_in_progress_2_do_wait_peer_trust(
    handle: Handle,
) -> Result[UserClaimInProgress3Info, ClaimInProgressError]:
    ...


async def claimer_device_in_progress_2_do_wait_peer_trust(
    handle: Handle,
) -> Result[DeviceClaimInProgress3Info, ClaimInProgressError]:
    ...


class UserClaimFinalizeInfo(Structure):
    handle: Handle


class DeviceClaimFinalizeInfo(Structure):
    handle: Handle


async def claimer_user_in_progress_3_do_claim(
    handle: Handle,
    requested_device_label: Optional[DeviceLabel],
    requested_human_handle: Optional[HumanHandle],
) -> Result[UserClaimFinalizeInfo, ClaimInProgressError]:
    ...


async def claimer_device_in_progress_3_do_claim(
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
