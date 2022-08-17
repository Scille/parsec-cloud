# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import attr
from typing import Optional
from pendulum import DateTime

from parsec.api.protocol import UserID, DeviceID, HumanHandle, DeviceLabel, UserProfile

from parsec._parsec import LocalDevice


@attr.s(slots=True, frozen=True, auto_attribs=True)
class UserInfo:
    user_id: UserID
    human_handle: Optional[HumanHandle]
    profile: UserProfile
    created_on: DateTime
    revoked_on: Optional[DateTime]

    @property
    def user_display(self) -> str:
        return str(self.human_handle or self.user_id)

    @property
    def short_user_display(self) -> str:
        return str(self.human_handle.label if self.human_handle else self.user_id)

    @property
    def is_revoked(self):
        # Note that we might consider a user revoked even though our current time is still
        # below the revokation timestamp. This is because there is no clear causality between
        # our time and the production of the revokation timestamp (as it might have been produced
        # by another device). So we simply consider a user revoked if a revokation timestamp has
        # been issued.
        return bool(self.revoked_on)

    def __repr__(self):
        return f"<UserInfo {self.user_display}>"


@attr.s(slots=True, frozen=True, auto_attribs=True)
class DeviceInfo:
    device_id: DeviceID
    device_label: Optional[DeviceLabel]
    created_on: DateTime

    @property
    def device_name(self):
        return self.device_id.device_name

    @property
    def device_display(self) -> str:
        return str(self.device_label or self.device_id.device_name)

    def __repr__(self):
        return f"DeviceInfo({self.device_display})"


__all__ = ["LocalDevice", "DeviceInfo", "UserInfo"]
