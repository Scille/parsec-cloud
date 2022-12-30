# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Union

from marshmallow import ValidationError

from parsec._parsec import (
    BackendActionAddr,
    BackendAddr,
    BackendInvitationAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    BackendPkiEnrollmentAddr,
)
from parsec.serde import fields

PARSEC_SCHEME = "parsec"

BackendAddrType = Union[
    BackendAddr,
    BackendInvitationAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
    BackendOrganizationFileLinkAddr,
    BackendPkiEnrollmentAddr,
]


class BackendAddrField(fields.Field[BackendAddr]):
    def _deserialize(self, value: object, attr: str, data: object) -> BackendAddr:
        if not isinstance(value, str):
            raise ValidationError(f"expected 'str' for got '{type(value)}'")
        try:
            return BackendAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value: BackendAddr | None, attr: str, data: object) -> str | None:
        if value is None:
            return None
        assert isinstance(value, BackendAddr)
        return value.to_url()


class BackendPkiEnrollmentAddrField(fields.Field[BackendPkiEnrollmentAddr]):
    def _deserialize(self, value: object, attr: str, data: object) -> BackendPkiEnrollmentAddr:
        if not isinstance(value, str):
            raise ValidationError(f"expected 'str' for got '{type(value)}'")
        try:
            return BackendPkiEnrollmentAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(
        self, value: BackendPkiEnrollmentAddr | None, attr: str, data: object
    ) -> str | None:
        if value is None:
            return None
        assert isinstance(value, BackendPkiEnrollmentAddr)
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
