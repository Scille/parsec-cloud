# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from parsec.components.async_enrollment import (
    BaseAsyncEnrollmentComponent,
)
from parsec.components.postgresql import AsyncpgPool


class PGAsyncEnrollmentComponent(BaseAsyncEnrollmentComponent):
    def __init__(self, pool: AsyncpgPool) -> None:
        self.pool = pool
