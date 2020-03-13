# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.protocol.base import packb, unpackb
from parsec.api.protocol import ADMINISTRATION_CMDS, AUTHENTICATED_CMDS, ANONYMOUS_CMDS


async def check_forbidden_cmds(backend_sock, cmds):
    for cmd in cmds:
        await backend_sock.send(packb({"cmd": cmd}))
        rep = await backend_sock.recv()
        assert unpackb(rep) == {"status": "unknown_command", "reason": "Unknown command"}


@pytest.mark.trio
async def test_administration_has_limited_access(administration_backend_sock):
    await check_forbidden_cmds(
        administration_backend_sock, (ANONYMOUS_CMDS | AUTHENTICATED_CMDS) - ADMINISTRATION_CMDS
    )


@pytest.mark.trio
async def test_anonymous_has_limited_access(anonymous_backend_sock):
    await check_forbidden_cmds(
        anonymous_backend_sock, (ADMINISTRATION_CMDS | AUTHENTICATED_CMDS) - ANONYMOUS_CMDS
    )


@pytest.mark.trio
async def test_authenticated_has_limited_access(alice_backend_sock):
    await check_forbidden_cmds(
        alice_backend_sock, (ADMINISTRATION_CMDS | AUTHENTICATED_CMDS) - AUTHENTICATED_CMDS
    )
