# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Type, TypeVar

from marshmallow import ValidationError
from marshmallow.fields import Field

from parsec._parsec import (
    ApiVersion,
    DeviceID,
    DeviceLabel,
    HumanHandle,
    OrganizationID,
    UserID,
    UserProfile,
)
from parsec.serde import fields, validate

UserIDTypeVar = TypeVar("UserIDTypeVar", bound="UserID")
DeviceIDTypeVar = TypeVar("DeviceIDTypeVar", bound="DeviceID")


OrganizationIDField: Type[Field[OrganizationID]] = fields.str_based_field_factory(OrganizationID)
UserIDField: Type[Field[UserID]] = fields.str_based_field_factory(UserID)
DeviceIDField: Type[Field[DeviceID]] = fields.str_based_field_factory(DeviceID)
DeviceLabelField: Type[Field[DeviceLabel]] = fields.str_based_field_factory(DeviceLabel)


class HumanHandleField(fields.Field[HumanHandle]):
    email_field = fields.String(required=True)
    label_field = fields.String(required=True)
    args = (email_field, label_field)

    def _serialize(
        self, value: HumanHandle | None, attr: str, data: object
    ) -> tuple[str, str] | None:
        if value is None:
            return None
        return (value.email, value.label)

    def _deserialize(self, value: object, attr: str, obj: dict[str, object]) -> HumanHandle:
        if not isinstance(value, (list, tuple)):
            raise ValidationError("Expecting list or tuple")
        try:
            email_value, label_value = value
        except ValueError:
            raise ValidationError("Expecting two elements")
        email = self.email_field.deserialize(email_value)
        label = self.label_field.deserialize(label_value)
        try:
            return HumanHandle(email, label)
        except ValueError as exc:
            raise ValidationError(str(exc))


class ApiVersionField(Field[ApiVersion]):
    version = fields.Integer(required=True, validate=validate.Range(min=0))
    revision = fields.Integer(required=True, validate=validate.Range(min=0))

    """ApiVersion already handled by pack/unpack"""

    def _serialize(
        self, value: ApiVersion | None, attr: str, obj: object
    ) -> tuple[int, int] | None:
        if value is None:
            return None
        return (value.version, value.revision)

    def _deserialize(self, value: object, attr: str, data: dict[str, object]) -> ApiVersion:
        if isinstance(value, (ApiVersion)):
            return value

        if not isinstance(value, (tuple, list)):
            raise ValidationError("Expecting tuple or list")

        try:
            version_value, revision_value = value
        except ValueError as exc:
            raise ValidationError(str(exc))

        version = self.version.deserialize(version_value)
        revision = self.revision.deserialize(revision_value)

        return ApiVersion(version, revision)


UserProfileField = fields.rust_enum_field_factory(UserProfile)
