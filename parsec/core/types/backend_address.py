# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from marshmallow import ValidationError
from parsec.serde import fields
from parsec._parsec import (
    BackendAddr,
    BackendActionAddr,
    BackendInvitationAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    BackendPkiEnrollmentAddr,
)
from typing import Optional, Union

PARSEC_SCHEME = "parsec"

BackendAddrType = Union[
    BackendAddr,
    BackendActionAddr,
    BackendInvitationAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    BackendPkiEnrollmentAddr,
]


class BackendOrganizationAddrField(fields.Field):
    def _deserialize(self, value: object, attr: str, data: object) -> BackendOrganizationAddr:
        try:
            if not isinstance(value, str):
                raise ValidationError(f"expected 'str' for got '{type(value)}'")

            return BackendOrganizationAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(
        self, value: Optional[BackendOrganizationAddr], attr: str, data: object
    ) -> Optional[str]:
        if value is None:
            return None

        return value.to_url()


class BackendAddrField(fields.Field):
    def _deserialize(self, value: object, attr: str, data: object) -> BackendAddr:
        try:
            if not isinstance(value, str):
                raise ValidationError(f"expected 'str' for got '{type(value)}'")

            return BackendAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value: Optional[BackendAddr], attr: str, data: object) -> Optional[str]:
        if value is None:
            return None

        return value.to_url()


class BackendPkiEnrollmentAddrField(fields.Field):
    def _deserialize(self, value: object, attr: str, data: object) -> BackendPkiEnrollmentAddr:
        try:
            if not isinstance(value, str):
                raise ValidationError(f"expected 'str' for got '{type(value)}'")

            return BackendPkiEnrollmentAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(
        self, value: Optional[BackendPkiEnrollmentAddr], attr: str, data: object
    ) -> Optional[str]:
        if value is None:
            return None

        return value.to_url()


__all__ = [
    "BackendAddr",
    "BackendActionAddr",
    "BackendInvitationAddr",
    "BackendOrganizationAddr",
    "BackendOrganizationBootstrapAddr",
    "BackendOrganizationFileLinkAddr",
    "BackendPkiEnrollmentAddr",
    "BackendAddrType",
]
