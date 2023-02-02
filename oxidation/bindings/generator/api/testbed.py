# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

from typing import Optional

from .common import *


async def test_new_testbed(template: Ref[str], test_server: Optional[Ref[BackendAddr]]) -> Path:
    ...


async def test_drop_testbed(path: Ref[Path]) -> None:
    ...
