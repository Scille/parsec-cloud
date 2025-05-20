# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec.components.account import (
    BaseAccountComponent,
)
from parsec.components.postgresql import AsyncpgPool
from parsec.config import BackendConfig


class PGAccountComponent(BaseAccountComponent):
    def __init__(self, pool: AsyncpgPool, config: BackendConfig) -> None:
        super().__init__(config=config)
        self.pool = pool
