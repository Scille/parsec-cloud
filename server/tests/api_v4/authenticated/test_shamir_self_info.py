# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    ShamirRecoveryBriefCertificate,
    authenticated_cmds,
)
from tests.common import CoolorgRpcClients
from tests.common.client import setup_shamir_for_coolorg


async def test_authenticated_shamir_recovery_self_info_ok(
    coolorg: CoolorgRpcClients, with_postgresql: bool
) -> None:
    if with_postgresql:
        pytest.skip("TODO: postgre not implemented yet")

    # Check if there is no shamir
    rep = await coolorg.alice.shamir_recovery_self_info()
    assert rep == authenticated_cmds.v4.shamir_recovery_self_info.RepOk(None)

    # setup shamir
    await setup_shamir_for_coolorg(coolorg)

    rep = await coolorg.alice.shamir_recovery_self_info()

    assert isinstance(rep, authenticated_cmds.v4.shamir_recovery_self_info.RepOk)
    assert isinstance(rep.self_info, bytes)

    brief = ShamirRecoveryBriefCertificate.unsecure_load(rep.self_info)
    assert brief.author == coolorg.alice.device_id
    assert brief.user_id == coolorg.alice.user_id
    # assert brief.timestamp==dt
    assert brief.threshold == 1
    assert brief.per_recipient_shares == {coolorg.mallory.user_id: 2}
