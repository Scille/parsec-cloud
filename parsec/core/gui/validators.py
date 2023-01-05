# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from PyQt5.QtCore import QRegularExpression
from PyQt5.QtGui import QIntValidator, QRegularExpressionValidator, QValidator

from parsec.api.data import EntryName
from parsec.api.protocol import DeviceLabel, HumanHandle, OrganizationID, UserID
from parsec.core.types import (
    BackendActionAddr,
    BackendAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
)


def trim_string(s: str) -> str:
    s = s.strip()
    return " ".join(s.split())


class NetworkPortValidator(QIntValidator):
    def __init__(self) -> None:
        super().__init__(1, 65536)


class OrganizationIDValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            OrganizationID(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class BackendAddrValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            BackendAddr.from_url(string, allow_http_redirection=True)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class BackendOrganizationAddrValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            BackendOrganizationAddr.from_url(string, allow_http_redirection=True)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Intermediate, string, pos


class BackendOrganizationBootstrapAddrValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            BackendOrganizationBootstrapAddr.from_url(string, allow_http_redirection=True)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Intermediate, string, pos


class BackendActionAddrValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            BackendActionAddr.from_url(string, allow_http_redirection=True)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Intermediate, string, pos


class UserIDValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            UserID(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class DeviceLabelValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            DeviceLabel(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class UserNameValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        # HumanHandle does not like spaces. To be a bit nicer to the user, we remove
        # them.
        string = trim_string(string)

        if len(string) == 0:
            return QValidator.Intermediate, string, pos
        try:
            # HumanHandle raises the same ValueError if either email or label are incorrect.
            # We trick it by using an email we know will be valid, so that the only ValueError
            # that can be raised will be because of an incorrect label.
            HumanHandle(email="local@domain.com", label=string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class EmailValidator(QRegularExpressionValidator):
    def __init__(self) -> None:
        # Note: this regex is very approximative, but it's used for two reasons:
        # - to get a sensible `QValidator.Intermediate` status
        # - to ban weird-but-valid email addresses such as `example@example.com#` and `a@b.c`
        # However, it might not be able to accurately detect invalid email addresses.
        # For instance, it will falsely report `example@-example.com` as a valid email.
        # It will also report email with excessively long domain names as valid, although they are not.
        # Ultimately, the actual validation is performed by ANDing the results of the regex
        # and the `HumanHandle` constructor, which in turn uses the `email_address_parser` crate.
        email_regex = r"^([a-zA-Z0-9_%+-]+\.)*([a-zA-Z0-9_%+-]+)@([a-zA-Z0-9-]+\.)+([a-zA-Z]{2,})$"
        super().__init__(QRegularExpression(email_regex))

    def validate(self, input: str, pos: int) -> tuple[QValidator.State, str, int]:
        status, string, pos = super().validate(input, pos)
        if status != QValidator.Acceptable:
            return status, string, pos
        try:
            HumanHandle(email=input, label="Some Label")
        except ValueError:
            return QValidator.Invalid, string, pos
        else:
            return QValidator.Acceptable, string, pos


class WorkspaceNameValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            EntryName(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class NotEmptyValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        return QValidator.Acceptable if len(string) else QValidator.Invalid, string, pos


class FileNameValidator(QValidator):
    def validate(self, string: str, pos: int) -> tuple[QValidator.State, str, int]:
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            EntryName(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos
