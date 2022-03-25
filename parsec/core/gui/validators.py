# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from PyQt5.QtCore import QRegularExpression
from PyQt5.QtGui import QValidator, QIntValidator, QRegularExpressionValidator

from parsec.api.data import EntryName
from parsec.api.protocol import OrganizationID, UserID, DeviceLabel
from parsec.core.types import (
    BackendAddr,
    BackendActionAddr,
    BackendOrganizationAddr,
    BackendOrganizationBootstrapAddr,
)


def trim_user_name(name):
    name = name.strip()
    return " ".join(name.split())


class NetworkPortValidator(QIntValidator):
    def __init__(self):
        super().__init__(1, 65536)


class OrganizationIDValidator(QValidator):
    def validate(self, string, pos):
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            OrganizationID(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class BackendAddrValidator(QValidator):
    def validate(self, string, pos):
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            BackendAddr.from_url(string, allow_http_redirection=True)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class BackendOrganizationAddrValidator(QValidator):
    def validate(self, string, pos):
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            BackendOrganizationAddr.from_url(string, allow_http_redirection=True)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Intermediate, string, pos


class BackendOrganizationBootstrapAddrValidator(QValidator):
    def validate(self, string, pos):
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            BackendOrganizationBootstrapAddr.from_url(string, allow_http_redirection=True)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Intermediate, string, pos


class BackendActionAddrValidator(QValidator):
    def validate(self, string, pos):
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            BackendActionAddr.from_url(string, allow_http_redirection=True)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Intermediate, string, pos


class UserIDValidator(QValidator):
    def validate(self, string, pos):
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            UserID(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class DeviceLabelValidator(QValidator):
    def validate(self, string, pos):
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            DeviceLabel(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class EmailValidator(QRegularExpressionValidator):
    def __init__(self):
        super().__init__(QRegularExpression(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"))


class WorkspaceNameValidator(QValidator):
    def validate(self, string, pos):
        try:
            if len(string) == 0:
                return QValidator.Intermediate, string, pos
            EntryName(string)
            return QValidator.Acceptable, string, pos
        except ValueError:
            return QValidator.Invalid, string, pos


class NotEmptyValidator(QValidator):
    def validate(self, string, pos):
        return QValidator.Acceptable if len(string) else QValidator.Invalid, string, pos
