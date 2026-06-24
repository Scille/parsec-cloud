# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    OrganizationID,
    VlobID,
)
from parsec.components.editics import BaseEditicsComponent, EditicsCreateOrJoinBadOutcome
from parsec.components.postgresql import AsyncpgPool
from parsec.config import BackendConfig


class PGEditicsComponent(BaseEditicsComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig):
        super().__init__(config)
        self.pool = pool

    @override
    async def editics_create_or_join_session(
        self,
        encrypted_session_key: bytes,
        key_index: int,
        file_id: VlobID,
        workspace_id: VlobID,
        organization_id: OrganizationID,
    ) -> tuple[bytes, int] | EditicsCreateOrJoinBadOutcome:
        raise NotImplementedError
