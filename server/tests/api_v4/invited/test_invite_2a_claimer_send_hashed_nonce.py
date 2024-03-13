# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import HashDigest, invited_cmds
from tests.common import Backend, CoolorgRpcClients


async def test_enrollment_wrong_state(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.invited_alice_dev3.invite_2a_claimer_send_hashed_nonce(
        claimer_hashed_nonce=HashDigest.from_data(b"hello-world")
    )

    assert rep == invited_cmds.v4.invite_2a_claimer_send_hashed_nonce.RepEnrollmentWrongState()
