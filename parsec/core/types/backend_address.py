# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

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

PARSEC_SCHEME = "parsec"


class BackendOrganizationAddrField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return BackendOrganizationAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value, attr, data):
        if value is None:
            return None

        return value.to_url()


class BackendAddrField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return BackendAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value, attr, data):
        if value is None:
            return None

        return value.to_url()


class BackendPkiEnrollmentAddrField(fields.Field):
    def _deserialize(self, value, attr, data):
        try:
            return BackendPkiEnrollmentAddr.from_url(value)
        except ValueError as exc:
            raise ValidationError(str(exc)) from exc

    def _serialize(self, value, attr, data):
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
]
