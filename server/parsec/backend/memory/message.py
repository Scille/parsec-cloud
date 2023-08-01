# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from collections import defaultdict
from copy import deepcopy
from typing import Any, Callable, Coroutine, List, Tuple

from parsec._parsec import (
    BackendEventMessageReceived,
    DateTime,
    DeviceID,
    OrganizationID,
    UserID,
)
from parsec.backend.memory.user import MemoryUserComponent
from parsec.backend.message import BaseMessageComponent


class MemoryMessageComponent(BaseMessageComponent):
    def __init__(self, send_event: Callable[..., Coroutine[Any, Any, None]]) -> None:
        self._send_event = send_event
        self._organizations: dict[
            OrganizationID, dict[UserID, List[Tuple[DeviceID, DateTime, bytes, int]]]
        ] = defaultdict(lambda: defaultdict(list))
        self._user_component: MemoryUserComponent

    def register_components(self, user: MemoryUserComponent, **other_components: Any) -> None:
        self._user_component = user

    async def send(
        self,
        organization_id: OrganizationID,
        sender: DeviceID,
        recipient: UserID,
        timestamp: DateTime,
        body: bytes,
    ) -> None:
        messages = self._organizations[organization_id]
        certificate_index = self._user_component.get_current_certificate_index(organization_id)
        messages[recipient].append((sender, timestamp, body, certificate_index))
        index = len(messages[recipient])
        await self._send_event(
            BackendEventMessageReceived(
                organization_id=organization_id,
                author=sender,
                recipient=recipient,
                index=index,
                message=body,
            )
        )

    async def get(
        self, organization_id: OrganizationID, recipient: UserID, offset: int
    ) -> List[Tuple[DeviceID, DateTime, bytes, int]]:
        messages = self._organizations[organization_id]
        return messages[recipient][offset:]

    def test_duplicate_organization(self, id: OrganizationID, new_id: OrganizationID) -> None:
        self._organizations[new_id] = deepcopy(self._organizations[id])

    def test_drop_organization(self, id: OrganizationID) -> None:
        self._organizations.pop(id, None)
