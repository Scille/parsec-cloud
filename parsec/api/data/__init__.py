# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.api.data.base import DataError
from parsec.api.data.certif import (
    UserCertificateContent,
    DeviceCertificateContent,
    RevokedDeviceCertificateContent,
    RealmRoleCertificateContent,
)

__api_data_version__ = (1, 0)


__all__ = (
    "__api_data_version__",
    "DataError",
    "UserCertificateContent",
    "DeviceCertificateContent",
    "RevokedDeviceCertificateContent",
    "RealmRoleCertificateContent",
)
