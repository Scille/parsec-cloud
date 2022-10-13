# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.api.protocol import APIV1_ANONYMOUS_CMDS
from parsec.core.backend_connection import apiv1_backend_anonymous_cmds_factory

from tests.core.backend_connection.common import ALL_CMDS


@pytest.mark.trio
async def test_anonymous_cmds_has_right_methods(running_backend, coolorg):
    async with apiv1_backend_anonymous_cmds_factory(coolorg.addr) as cmds:
        for method_name in APIV1_ANONYMOUS_CMDS:
            assert hasattr(cmds, method_name)
        for method_name in ALL_CMDS - APIV1_ANONYMOUS_CMDS:
            assert not hasattr(cmds, method_name)
