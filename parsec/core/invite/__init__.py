# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import TYPE_CHECKING

from parsec import FEATURE_FLAGS
from parsec._parsec import (
    InviteActiveUsersLimitReachedError,
    InviteAlreadyUsedError,
    InviteError,
    InviteNotFoundError,
    InvitePeerResetError,
)

if not TYPE_CHECKING and FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"]:
    from parsec._parsec import (
        DeviceGreetInitialCtx,
        DeviceGreetInProgress1Ctx,
        DeviceGreetInProgress2Ctx,
        DeviceGreetInProgress3Ctx,
        DeviceGreetInProgress4Ctx,
        UserGreetInitialCtx,
        UserGreetInProgress1Ctx,
        UserGreetInProgress2Ctx,
        UserGreetInProgress3Ctx,
        UserGreetInProgress4Ctx,
    )

    # Not oxidized yet
    from parsec.core.invite.greeter import (
        ShamirRecoveryGreetInitialCtx,
        ShamirRecoveryGreetInProgress1Ctx,
        ShamirRecoveryGreetInProgress2Ctx,
        ShamirRecoveryGreetInProgress3Ctx,
        get_shamir_recovery_share_data,
    )
else:
    from parsec.core.invite.greeter import (
        DeviceGreetInitialCtx,
        DeviceGreetInProgress1Ctx,
        DeviceGreetInProgress2Ctx,
        DeviceGreetInProgress3Ctx,
        DeviceGreetInProgress4Ctx,
        ShamirRecoveryGreetInitialCtx,
        ShamirRecoveryGreetInProgress1Ctx,
        ShamirRecoveryGreetInProgress2Ctx,
        ShamirRecoveryGreetInProgress3Ctx,
        UserGreetInitialCtx,
        UserGreetInProgress1Ctx,
        UserGreetInProgress2Ctx,
        UserGreetInProgress3Ctx,
        UserGreetInProgress4Ctx,
        get_shamir_recovery_share_data,
    )
from parsec.core.invite.organization import bootstrap_organization

if not TYPE_CHECKING and FEATURE_FLAGS["UNSTABLE_OXIDIZED_CLIENT_CONNECTION"]:
    from parsec._parsec import (
        DeviceClaimInitialCtx,
        DeviceClaimInProgress1Ctx,
        DeviceClaimInProgress2Ctx,
        DeviceClaimInProgress3Ctx,
        UserClaimInitialCtx,
        UserClaimInProgress1Ctx,
        UserClaimInProgress2Ctx,
        UserClaimInProgress3Ctx,
        claimer_retrieve_info,
    )

    # Not oxidized yet
    from parsec.core.invite.claimer import (
        ShamirRecoveryClaimInitialCtx,
        ShamirRecoveryClaimInProgress1Ctx,
        ShamirRecoveryClaimInProgress2Ctx,
        ShamirRecoveryClaimInProgress3Ctx,
        ShamirRecoveryClaimPreludeCtx,
    )
else:
    from parsec.core.invite.claimer import (
        DeviceClaimInitialCtx,
        DeviceClaimInProgress1Ctx,
        DeviceClaimInProgress2Ctx,
        DeviceClaimInProgress3Ctx,
        ShamirRecoveryClaimInitialCtx,
        ShamirRecoveryClaimInProgress1Ctx,
        ShamirRecoveryClaimInProgress2Ctx,
        ShamirRecoveryClaimInProgress3Ctx,
        ShamirRecoveryClaimPreludeCtx,
        UserClaimInitialCtx,
        UserClaimInProgress1Ctx,
        UserClaimInProgress2Ctx,
        UserClaimInProgress3Ctx,
        claimer_retrieve_info,
    )

__all__ = (
    # Exceptions
    "InviteError",
    "InvitePeerResetError",
    "InviteNotFoundError",
    "InviteAlreadyUsedError",
    "InviteActiveUsersLimitReachedError",
    # Claimer
    "claimer_retrieve_info",
    "ShamirRecoveryClaimPreludeCtx",
    "UserClaimInitialCtx",
    "DeviceClaimInitialCtx",
    "ShamirRecoveryClaimInitialCtx",
    "UserClaimInProgress1Ctx",
    "DeviceClaimInProgress1Ctx",
    "ShamirRecoveryClaimInProgress1Ctx",
    "UserClaimInProgress2Ctx",
    "DeviceClaimInProgress2Ctx",
    "ShamirRecoveryClaimInProgress2Ctx",
    "UserClaimInProgress3Ctx",
    "DeviceClaimInProgress3Ctx",
    "ShamirRecoveryClaimInProgress3Ctx",
    # Greeter
    "get_shamir_recovery_share_data",
    "UserGreetInitialCtx",
    "DeviceGreetInitialCtx",
    "ShamirRecoveryGreetInitialCtx",
    "UserGreetInProgress1Ctx",
    "DeviceGreetInProgress1Ctx",
    "ShamirRecoveryGreetInProgress1Ctx",
    "UserGreetInProgress2Ctx",
    "DeviceGreetInProgress2Ctx",
    "ShamirRecoveryGreetInProgress2Ctx",
    "UserGreetInProgress3Ctx",
    "DeviceGreetInProgress3Ctx",
    "ShamirRecoveryGreetInProgress3Ctx",
    "UserGreetInProgress4Ctx",
    "DeviceGreetInProgress4Ctx",
    # Organization
    "bootstrap_organization",
)
