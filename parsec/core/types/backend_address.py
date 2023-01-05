# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Union

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
