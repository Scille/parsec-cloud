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
else:
    from parsec.core.invite.greeter import (
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
else:
    from parsec.core.invite.claimer import (
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

__all__ = (
    # Exceptions
    "InviteError",
    "InvitePeerResetError",
    "InviteNotFoundError",
    "InviteAlreadyUsedError",
    "InviteActiveUsersLimitReachedError",
    # Claimer
    "claimer_retrieve_info",
    "UserClaimInitialCtx",
    "DeviceClaimInitialCtx",
    "UserClaimInProgress1Ctx",
    "DeviceClaimInProgress1Ctx",
    "UserClaimInProgress2Ctx",
    "DeviceClaimInProgress2Ctx",
    "UserClaimInProgress3Ctx",
    "DeviceClaimInProgress3Ctx",
    # Greeter
    "UserGreetInitialCtx",
    "DeviceGreetInitialCtx",
    "UserGreetInProgress1Ctx",
    "DeviceGreetInProgress1Ctx",
    "UserGreetInProgress2Ctx",
    "DeviceGreetInProgress2Ctx",
    "UserGreetInProgress3Ctx",
    "DeviceGreetInProgress3Ctx",
    "UserGreetInProgress4Ctx",
    "DeviceGreetInProgress4Ctx",
    # Organization
    "bootstrap_organization",
)
