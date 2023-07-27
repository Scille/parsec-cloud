# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from __future__ import annotations

from . import organization_bootstrap, ping, pki_enrollment_info, pki_enrollment_submit

class AnyCmdReq:
    @classmethod
    def load(
        cls, raw: bytes
    ) -> (
        organization_bootstrap.Req | ping.Req | pki_enrollment_info.Req | pki_enrollment_submit.Req
    ): ...

__all__ = [
    "AnyCmdReq",
    "organization_bootstrap",
    "ping",
    "pki_enrollment_info",
    "pki_enrollment_submit",
]
