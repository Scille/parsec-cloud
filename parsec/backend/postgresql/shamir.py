# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from typing import Any

from parsec._parsec import (
    DeviceID,
    OrganizationID,
    ShamirRecoverySetup,
    ShamirRevealToken,
    VerifyKey,
)
from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.shamir import BaseShamirComponent


class PGShamirComponent(BaseShamirComponent):
    def __init__(self, dbh: PGHandler, *args: Any, **kwargs: Any) -> None:
        super().__init__(*args, **kwargs)
        self.dbh = dbh

    async def recovery_others_list(
        self,
        organization_id: OrganizationID,
        author_id: DeviceID,
    ) -> list[tuple[bytes, bytes]]:
        raise NotImplementedError()

    async def recovery_self_info(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
    ) -> bytes | None:
        raise NotImplementedError()

    async def recovery_setup(
        self,
        organization_id: OrganizationID,
        author: DeviceID,
        author_verify_key: VerifyKey,
        setup: ShamirRecoverySetup | None,
    ) -> None:
        raise NotImplementedError()

    async def recovery_reveal(
        self,
        reveal_token: ShamirRevealToken,
    ) -> bytes | None:
        raise NotImplementedError()
