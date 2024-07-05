# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import PrivateKey, authenticated_cmds


async def test_greeter_step_wait_peer_round_trip() -> None:
    key = PrivateKey.generate()
    step = authenticated_cmds.v4.invite_greeter_step.GreeterStepWaitPeer(key.public_key)
    new_step = authenticated_cmds.v4.invite_greeter_step.GreeterStep.load(step.dump())
    assert isinstance(new_step, authenticated_cmds.v4.invite_greeter_step.GreeterStepWaitPeer)
    assert new_step.public_key == key.public_key
