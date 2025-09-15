# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import override

from parsec._parsec import (
    OrganizationID,
    VlobID,
)
from parsec.components.editics import BaseEditicsComponent, EditicsCreateOrJoinBadOutcome
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import MemoryDatamodel, MemoryEditicsSession
from parsec.config import BackendConfig


class MemoryEditicsComponent(BaseEditicsComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        config: BackendConfig,
        event_bus: EventBus,
    ) -> None:
        super().__init__(config)
        self._data = data
        self._event_bus = event_bus

    @override
    async def editics_create_or_join_session(
        self,
        encrypted_session_key: bytes,
        key_index: int,
        file_id: VlobID,
        workspace_id: VlobID,
        organization_id: OrganizationID,
    ) -> tuple[bytes, int] | EditicsCreateOrJoinBadOutcome:
        try:
            sessions = self._data.organizations[organization_id].editics_sessions
        except KeyError:
            return EditicsCreateOrJoinBadOutcome.ORGANIZATION_NOT_FOUND

        try:
            current_session = sessions[file_id]
            assert current_session.workspace_id == workspace_id
        except KeyError:
            current_session = MemoryEditicsSession(
                workspace_id, file_id, encrypted_session_key, key_index
            )
            sessions[file_id] = current_session

        return (current_session.encrypted_session_key, current_session.key_index)
