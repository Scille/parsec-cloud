# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from parsec.components.editics import BaseEditicsComponent
from parsec.components.memory.datamodel import MemoryDatamodel
from parsec.config import BackendConfig


class MemoryEditicsComponent(BaseEditicsComponent):
    def __init__(self, data: MemoryDatamodel, config: BackendConfig) -> None:
        super().__init__(config)
        self._data = data
