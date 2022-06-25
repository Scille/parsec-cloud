# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from enum import Enum
from typing import TYPE_CHECKING, Optional, Any, Dict, Type, TypeVar, Literal
from marshmallow import ValidationError
from pendulum import DateTime

from parsec.crypto import VerifyKey, PublicKey
from parsec.serde import fields, post_load
from parsec.api.protocol import (
    RealmID,
    RealmIDField,
    DeviceID,
    UserID,
    HumanHandle,
    RealmRole,
    DeviceIDField,
    UserIDField,
    HumanHandleField,
    RealmRoleField,
    UserProfileField,
    UserProfile,
    DeviceLabel,
    DeviceLabelField,
)
from parsec.api.data.base import DataValidationError, BaseAPISignedData, BaseSignedDataSchema
import attr


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class UserCertificateContent(BaseAPISignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        # Override author field to allow for None value if signed by the root key
        author = DeviceIDField(required=True, allow_none=True)

        type = fields.CheckedConstant("user_certificate", required=True)
        user_id = UserIDField(required=True)
        # Added in Parsec v1.13
        # Human handle can be none in case of redacted certificate
        human_handle = HumanHandleField(allow_none=True, missing=None)
        public_key = fields.PublicKey(required=True)
        # `profile` replaces `is_admin` field (which is still required for backward
        # compatibility), hence `None` is not allowed
        is_admin = fields.Boolean(required=True)
        # Added in Parsec v1.14
        profile = UserProfileField(allow_none=False)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "UserCertificateContent":
            data.pop("type")

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

            return UserCertificateContent(**data)

    # Override author attribute to allow for None value if signed by the root key
    author: Optional[DeviceID]  # type: ignore[assignment]

    user_id: UserID
    human_handle: Optional[HumanHandle]
    public_key: PublicKey
    profile: UserProfile

    # Only used during schema serialization
    @property
    def is_admin(self) -> bool:
        return self.profile == UserProfile.ADMIN

    @classmethod
    def verify_and_load(
        cls,
        *args,
        expected_user: Optional[UserID] = None,
        expected_human_handle: Optional[HumanHandle] = None,
        **kwargs,
    ) -> "UserCertificateContent":
        data = super().verify_and_load(*args, **kwargs)
        if expected_user is not None and data.user_id != expected_user:
            raise DataValidationError(
                f"Invalid user ID: expected `{expected_user}`, got `{data.user_id}`"
            )
        if expected_human_handle is not None and data.human_handle != expected_human_handle:
            raise DataValidationError(
                f"Invalid human handle: expected `{expected_human_handle}`, got `{data.human_handle}`"
            )
        return data


_PyUserCertificateContent = UserCertificateContent
if not TYPE_CHECKING:
    try:
        from libparsec.types import UserCertificate as _RsUserCertificateContent
    except:
        pass
    else:
        UserCertificateContent = _RsUserCertificateContent


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class RevokedUserCertificateContent(BaseAPISignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("revoked_user_certificate", required=True)
        user_id = UserIDField(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "RevokedUserCertificateContent":
            data.pop("type")
            return RevokedUserCertificateContent(**data)

    user_id: UserID

    @classmethod
    def verify_and_load(
        cls, *args, expected_user: Optional[UserID] = None, **kwargs
    ) -> "RevokedUserCertificateContent":
        data = super().verify_and_load(*args, **kwargs)
        if expected_user is not None and data.user_id != expected_user:
            raise DataValidationError(
                f"Invalid user ID: expected `{expected_user}`, got `{data.user_id}`"
            )
        return data


DeviceCertificateContentTypeVar = TypeVar(
    "DeviceCertificateContentTypeVar", bound="DeviceCertificateContent"
)

_PyRevokedUserCertificateContent = RevokedUserCertificateContent
if not TYPE_CHECKING:
    try:
        from libparsec.types import RevokedUserCertificate as _RsRevokedUserCertificateContent
    except:
        pass
    else:
        RevokedUserCertificateContent = _RsRevokedUserCertificateContent


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class DeviceCertificateContent(BaseAPISignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        # Override author field to allow for None value if signed by the root key
        author = DeviceIDField(required=True, allow_none=True)

        type = fields.CheckedConstant("device_certificate", required=True)
        device_id = DeviceIDField(required=True)
        # Added in Parsec v1.14
        # Device label can be none in case of redacted certificate
        device_label = DeviceLabelField(allow_none=True, missing=None)
        verify_key = fields.VerifyKey(required=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "DeviceCertificateContent":
            data.pop("type")
            return DeviceCertificateContent(**data)

    # Override author attribute to allow for None value if signed by the root key
    author: Optional[DeviceID]  # type: ignore[assignment]

    device_id: DeviceID
    device_label: Optional[DeviceLabel]
    verify_key: VerifyKey

    @classmethod
    def verify_and_load(
        cls: Type[DeviceCertificateContentTypeVar],
        *args: Any,
        expected_device: Optional[DeviceID] = None,
        **kwargs: Any,
    ) -> "DeviceCertificateContent":
        data = super().verify_and_load(*args, **kwargs)
        if expected_device is not None and data.device_id != expected_device:
            raise DataValidationError(
                f"Invalid device ID: expected `{expected_device}`, got `{data.device_id}`"
            )
        return data


_PyDeviceCertificateContent = DeviceCertificateContent
if not TYPE_CHECKING:
    try:
        from libparsec.types import DeviceCertificate as _RsDeviceCertificateContent
    except:
        pass
    else:
        DeviceCertificateContent = _RsDeviceCertificateContent


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class RealmRoleCertificateContent(BaseAPISignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("realm_role_certificate", required=True)
        realm_id = RealmIDField(required=True)
        user_id = UserIDField(required=True)
        role = RealmRoleField(required=True, allow_none=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "RealmRoleCertificateContent":
            data.pop("type")
            return RealmRoleCertificateContent(**data)

    realm_id: RealmID
    user_id: UserID
    role: Optional[RealmRole]  # Set to None if role removed

    @classmethod
    def build_realm_root_certif(cls, author: DeviceID, timestamp: DateTime, realm_id: RealmID):
        return cls(
            author=author,
            timestamp=timestamp,
            realm_id=realm_id,
            user_id=author.user_id,
            role=RealmRole.OWNER,
        )

    @classmethod
    def verify_and_load(
        cls,
        *args,
        expected_realm: Optional[RealmID] = None,
        expected_user: Optional[UserID] = None,
        expected_role: Optional[RealmRole] = None,
        **kwargs,
    ) -> "RealmRoleCertificateContent":
        data = super().verify_and_load(*args, **kwargs)
        if expected_user is not None and data.user_id != expected_user:
            raise DataValidationError(
                f"Invalid user ID: expected `{expected_user}`, got `{data.user_id}`"
            )
        if expected_realm is not None and data.realm_id != expected_realm:
            raise DataValidationError(
                f"Invalid realm ID: expected `{expected_realm}`, got `{data.realm_id}`"
            )
        if expected_role is not None and data.role != expected_role:
            raise DataValidationError(
                f"Invalid role: expected `{expected_role}`, got `{data.role}`"
            )
        return data


class SequesterAuthorityKeyFormat(Enum):
    RSA = "RSA"


SequesterAuthorityKeyFormatFields = fields.enum_field_factory(SequesterAuthorityKeyFormat)


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class SequesterAuthorityKeyCertificate(BaseAPISignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        # Override author field to always uses None given this certificate can only be signed by the root key
        author = fields.CheckedConstant(
            None, required=True, allow_none=True
        )  # Constant None fields required to be allowed to be None !

        type = fields.CheckedConstant("sequester_authority_key_certificate", required=True)
        verify_key = fields.Bytes(required=True)
        verify_key_format = SequesterAuthorityKeyFormatFields(requied=True)

        @post_load
        def make_obj(self, data: Dict[str, Any]) -> "SequesterAuthorityKeyCertificate":
            data.pop("type")
            return SequesterAuthorityKeyCertificate(**data)

    # Override author field to always uses None given this certificate can only be signed by the root key
    author: Literal[None]  # type: ignore[assignment]
    verify_key: bytes
    verify_key_format: SequesterAuthorityKeyFormat
