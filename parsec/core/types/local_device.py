# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import attr
from typing import TYPE_CHECKING, Tuple, Optional
from hashlib import sha256
from marshmallow import ValidationError
from libparsec.types import DateTime

from parsec.crypto import SecretKey, PrivateKey, SigningKey
from parsec.serde import fields, post_load
from parsec.api.protocol import (
    UserID,
    DeviceID,
    OrganizationID,
    HumanHandle,
    DeviceIDField,
    HumanHandleField,
    DeviceLabel,
    DeviceLabelField,
)
from parsec.api.data import BaseSchema, EntryID, EntryIDField, UserProfile, UserProfileField
from parsec.core.types.base import BaseLocalData
from parsec.core.types.backend_address import BackendOrganizationAddr, BackendOrganizationAddrField


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class LocalDevice(BaseLocalData):
    class SCHEMA_CLS(BaseSchema):
        organization_addr = BackendOrganizationAddrField(required=True)
        device_id = DeviceIDField(required=True)
        # Added in Parsec v1.14
        device_label = DeviceLabelField(required=False, allow_none=True, missing=None)
        # Added in Parsec v1.13
        human_handle = HumanHandleField(required=False, allow_none=True, missing=None)
        signing_key = fields.SigningKey(required=True)
        private_key = fields.PrivateKey(required=True)
        # `profile` replaces `is_admin` field (which is still required for backward
        # compatibility), hence `None` is not allowed
        is_admin = fields.Boolean(required=True)
        # Added in Parsec v1.14
        profile = UserProfileField(required=False, allow_none=False)
        user_manifest_id = EntryIDField(required=True)
        user_manifest_key = fields.SecretKey(required=True)
        local_symkey = fields.SecretKey(required=True)

        @post_load
        def make_obj(self, data):
            # Handle legacy `is_admin` field
            default_profile = UserProfile.ADMIN if data.pop("is_admin") else UserProfile.STANDARD
            try:
                profile = data["profile"]
            except KeyError:
                data["profile"] = default_profile
            else:
                if default_profile == UserProfile.ADMIN and profile != UserProfile.ADMIN:
                    raise ValidationError(
                        "Fields `profile` and `is_admin` have incompatible values"
                    )

            return LocalDevice(**data)

    organization_addr: BackendOrganizationAddr
    device_id: DeviceID
    device_label: Optional[DeviceLabel]
    human_handle: Optional[HumanHandle]
    signing_key: SigningKey
    private_key: PrivateKey
    profile: UserProfile
    user_manifest_id: EntryID
    user_manifest_key: SecretKey
    local_symkey: SecretKey

    # Only used during schema serialization
    @property
    def is_admin(self) -> bool:
        return self.profile == UserProfile.ADMIN

    @property
    def is_outsider(self) -> bool:
        return self.profile == UserProfile.OUTSIDER

    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    @property
    def slug(self) -> str:
        """The slug is unique identifier for a particular device.

        It is composed of a small part of the RVK hash, the organization ID
        and the device ID, although it shouldn't be assumed that this information
        can be recovered from the slug as this might change in the future.

        The purpose of the slog is simply to tell whether `LocalDevice` and
        `AvailableDevice` objects corresponds to the same device.
        """
        # Add a hash to avoid clash when the backend is reseted
        # and we recreate a device with same organization/device_id
        # organization and device_id than a previous one
        hash_part = sha256(self.root_verify_key.encode()).hexdigest()[:10]
        return f"{hash_part}#{self.organization_id}#{self.device_id}"

    @property
    def slughash(self) -> str:
        """
        Slug is long and not readable enough (given device_id is made of uuids).
        Hence it's often simpler to rely on it hash instead (e.g. select the
        device to use in the CLI by providing the beginning of the hash)
        """
        return sha256(self.slug.encode()).hexdigest()

    @staticmethod
    def load_slug(slug: str) -> Tuple[OrganizationID, DeviceID]:
        """
        Raises: ValueError
        """
        _, raw_org_id, raw_device_id = slug.split("#")
        return OrganizationID(raw_org_id), DeviceID(raw_device_id)

    @property
    def root_verify_key(self):
        return self.organization_addr.root_verify_key

    @property
    def organization_id(self):
        return self.organization_addr.organization_id

    @property
    def device_name(self):
        return self.device_id.device_name

    @property
    def user_id(self):
        return self.device_id.user_id

    @property
    def verify_key(self):
        return self.signing_key.verify_key

    @property
    def public_key(self):
        return self.private_key.public_key

    @property
    def user_display(self) -> str:
        return str(self.human_handle or self.user_id)

    @property
    def short_user_display(self) -> str:
        return str(self.human_handle.label if self.human_handle else self.user_id)

    @property
    def device_display(self) -> str:
        return str(self.device_label or self.device_id.device_name)

    def timestamp(self) -> DateTime:
        """This method centralizes the production of parsec timestamps for a given device.
        At the moment it is simply an alias to `DateTime.now` but it has two main benefits:
        1. Allowing for easier testing by patching this method in device-sepecific way
        2. Allowing for other implementation in the future allowing to track, check and
           possibly alter the production of timestamps.
        """
        return DateTime.now()


_PyLocalDevice = LocalDevice
if not TYPE_CHECKING:
    try:
        from libparsec.types import LocalDevice as _RsLocalDevice
    except:
        pass
    else:
        LocalDevice = _RsLocalDevice


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
