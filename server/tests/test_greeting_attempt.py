# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    GreeterOrClaimer,
    GreetingAttemptID,
    HashDigest,
    PrivateKey,
    PublicKey,
    authenticated_cmds,
    generate_sas_code_nonce,
    invited_cmds,
)
from parsec.asgi.rpc import CustomHttpStatus
from parsec.components.invite import process_claimer_step, process_greeter_step
from tests.common import Backend, CoolorgRpcClients, RpcTransportError


def test_step_processing():
    """
    The load and dump of the step data into the server database is not protected by the usual JSON schema
    since this process is internal to the server. Instead, this logic is implemented in the `process_greeter_step`
    and `process_claimer_step` functions.

    This test contains snapshot data to test against change in this logic. Such a change would require a database
    migration, so make sure to have a migration plan if you end up changing the snapshot data.
    """
    # Greeter data
    # spell-checker: disable
    greeter_key = PublicKey(
        b"\xa1\xcfKq\xb2s]\xb1\x03\xa1u/\x9f\xbdi]D4\xf0\x95\x0b6\xdf\xadSC\xa4|D%jh"
    )
    greeter_nonce = b"\xd6\x17\xb7\x04\xaf\xd6\x17\xde0:\x81\x95u\x0f\xcb_\xe8\xf8.t=\x96j?\x8f\xf4\x90'\xc2\xb7\xc2\x8e\xcb\x07\xc3\x0e\x88\xf4K})\xb0\xbf\x08d\xad@\xa5\xe3\xf1\xf9X\xe9\x89\xcb\xf5\xd1\x1b\xb3\x7f\xc9X`+"
    greeter_payload = b"<greeter_payload>"
    # spell-checker: enable

    # Claimer data
    # spell-checker: disable
    claimer_key = PublicKey(
        b"\x9dTj\x00\xcc$=\n\x9an0@Se\xfd\xa4\x87\xcd\x93\xdd-\xc9\xd0\xa1\xa8wF\x82\x9fE\xef\x15"
    )
    claimer_nonce = b"\xc7\x86U\x7f\xee\x12\xb4\xb8\xbf\x91\xb5tJ5?Dhts\x14ob\xdbg\x11\xdbL\xd9i\x7f\xb5\x10n\x16K\xb0\xf9q\x06\xf2\x88\xd0\xbd\x17\x97\xe6\x08<\xbd\xab\x80\xbd\x91\xf7\xfe-\x92D<\xe4\xf6''\xcc"
    hashed_nonce = HashDigest.from_data(claimer_nonce)
    claimer_payload = b"<claimer_payload>"
    # spell-checker: disable

    # Snapshot data
    # Do not change without a database migration plan
    # spell-checker: disable
    greeter_snapshot = [
        b"\xa1\xcfKq\xb2s]\xb1\x03\xa1u/\x9f\xbdi]D4\xf0\x95\x0b6\xdf\xadSC\xa4|D%jh",
        b"",
        b"\xd6\x17\xb7\x04\xaf\xd6\x17\xde0:\x81\x95u\x0f\xcb_\xe8\xf8.t=\x96j?\x8f\xf4\x90'\xc2\xb7\xc2\x8e\xcb\x07\xc3\x0e\x88\xf4K})\xb0\xbf\x08d\xad@\xa5\xe3\xf1\xf9X\xe9\x89\xcb\xf5\xd1\x1b\xb3\x7f\xc9X`+",
        b"",
        b"",
        b"",
        b"",
        b"<greeter_payload>",
        b"",
    ]
    claimer_snapshot = [
        b"\x9dTj\x00\xcc$=\n\x9an0@Se\xfd\xa4\x87\xcd\x93\xdd-\xc9\xd0\xa1\xa8wF\x82\x9fE\xef\x15",
        b"j\x83\xbft\xfa)\x8a\xae\xc9\xcf9l\r|\x97\x1dk\xc9M\xbc\xfaL\xbc\xbb2\x1d\x82LEr\xb5\xf4",
        b"",
        b"\xc7\x86U\x7f\xee\x12\xb4\xb8\xbf\x91\xb5tJ5?Dhts\x14ob\xdbg\x11\xdbL\xd9i\x7f\xb5\x10n\x16K\xb0\xf9q\x06\xf2\x88\xd0\xbd\x17\x97\xe6\x08<\xbd\xab\x80\xbd\x91\xf7\xfe-\x92D<\xe4\xf6''\xcc",
        b"",
        b"",
        b"<claimer_payload>",
        b"",
        b"",
    ]
    # spell-checker: enable

    # Step 0
    greeter_step_0 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_key
    )
    index, data, convert = process_greeter_step(greeter_step_0)
    assert index == 0
    assert data == greeter_key.encode() == greeter_snapshot[index]
    assert convert(
        claimer_snapshot[index]
    ) == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber0WaitPeer(
        public_key=claimer_key
    )

    claimer_step_0 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=claimer_key
    )
    index, data, convert = process_claimer_step(claimer_step_0)
    assert index == 0
    assert data == claimer_key.encode() == claimer_snapshot[index]
    assert convert(
        greeter_snapshot[index]
    ) == invited_cmds.latest.invite_claimer_step.GreeterStepNumber0WaitPeer(public_key=greeter_key)

    # Step 1
    greeter_step_1 = (
        authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber1GetHashedNonce()
    )
    index, data, convert = process_greeter_step(greeter_step_1)
    assert index == 1
    assert data == b"" == greeter_snapshot[index]
    assert convert(
        claimer_snapshot[index]
    ) == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber1SendHashedNonce(
        hashed_nonce=hashed_nonce
    )

    claimer_step_1 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber1SendHashedNonce(
        hashed_nonce=hashed_nonce
    )
    index, data, convert = process_claimer_step(claimer_step_1)
    assert index == 1
    assert data == hashed_nonce.digest == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepNumber1GetHashedNonce()
    )

    # Step 2
    greeter_step_2 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber2SendNonce(
        greeter_nonce=greeter_nonce
    )
    index, data, convert = process_greeter_step(greeter_step_2)
    assert index == 2
    assert data == greeter_nonce == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber2GetNonce()
    )

    claimer_step_2 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce()
    index, data, convert = process_claimer_step(claimer_step_2)
    assert index == 2
    assert data == b"" == claimer_snapshot[index]
    assert convert(
        greeter_snapshot[index]
    ) == invited_cmds.latest.invite_claimer_step.GreeterStepNumber2SendNonce(
        greeter_nonce=greeter_nonce
    )

    # Step 3
    greeter_step_3 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce()
    index, data, convert = process_greeter_step(greeter_step_3)
    assert index == 3
    assert data == b"" == greeter_snapshot[index]
    assert convert(
        claimer_snapshot[index]
    ) == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber3SendNonce(
        claimer_nonce=claimer_nonce
    )

    claimer_step_3 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber3SendNonce(
        claimer_nonce=claimer_nonce
    )
    index, data, convert = process_claimer_step(claimer_step_3)
    assert index == 3
    assert data == claimer_nonce == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepNumber3GetNonce()
    )

    # Step 4
    greeter_step_4 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber4WaitPeerTrust()
    index, data, convert = process_greeter_step(greeter_step_4)
    assert index == 4
    assert data == b"" == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber4SignifyTrust()
    )

    claimer_step_4 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber4SignifyTrust()
    index, data, convert = process_claimer_step(claimer_step_4)
    assert index == 4
    assert data == b"" == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepNumber4WaitPeerTrust()
    )

    # Step 5
    greeter_step_5 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber5SignifyTrust()
    index, data, convert = process_greeter_step(greeter_step_5)
    assert index == 5
    assert data == b"" == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber5WaitPeerTrust()
    )

    claimer_step_5 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber5WaitPeerTrust()
    index, data, convert = process_claimer_step(claimer_step_5)
    assert index == 5
    assert data == b"" == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepNumber5SignifyTrust()
    )

    # Step 6
    greeter_step_6 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber6GetPayload()
    index, data, convert = process_greeter_step(greeter_step_6)
    assert index == 6
    assert data == b"" == greeter_snapshot[index]
    assert convert(
        claimer_snapshot[index]
    ) == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber6SendPayload(
        claimer_payload=claimer_payload
    )

    claimer_step_6 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber6SendPayload(
        claimer_payload=claimer_payload
    )
    index, data, convert = process_claimer_step(claimer_step_6)
    assert index == 6
    assert data == claimer_payload == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepNumber6GetPayload()
    )

    # Step 7
    greeter_step_7 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber7SendPayload(
        greeter_payload=greeter_payload
    )
    index, data, convert = process_greeter_step(greeter_step_7)
    assert index == 7
    assert data == greeter_payload == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber7GetPayload()
    )

    claimer_step_7 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber7GetPayload()
    index, data, convert = process_claimer_step(claimer_step_7)
    assert index == 7
    assert data == b"" == claimer_snapshot[index]
    assert convert(
        greeter_snapshot[index]
    ) == invited_cmds.latest.invite_claimer_step.GreeterStepNumber7SendPayload(
        greeter_payload=greeter_payload
    )

    # Step 8
    greeter_step_8 = (
        authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber8WaitPeerAcknowledgment()
    )
    index, data, convert = process_greeter_step(greeter_step_8)
    assert index == 8
    assert data == b"" == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber8Acknowledge()
    )

    claimer_step_8 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber8Acknowledge()
    index, data, convert = process_claimer_step(claimer_step_8)
    assert index == 8
    assert data == b"" == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepNumber8WaitPeerAcknowledgment()
    )


@pytest.mark.parametrize("invitation_type", ["user", "device"])
async def test_full_greeting_attempt(
    coolorg: CoolorgRpcClients, backend: Backend, invitation_type: str
) -> None:
    match invitation_type:
        case "user":
            greeter = coolorg.alice
            claimer = coolorg.invited_zack
            invitation_token = claimer.token
        case "device":
            greeter = coolorg.alice
            claimer = coolorg.invited_alice_dev3
            invitation_token = claimer.token
        case _:
            assert False

    # Greeter starts the attempt
    rep = await greeter.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    greeting_attempt = rep.greeting_attempt

    # Greeter performs step 0 first
    greeter_key = PrivateKey.generate()
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
            public_key=greeter_key.public_key
        ),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepNotReady()

    # Claimer starts the attempt
    rep = await claimer.invite_claimer_start_greeting_attempt(
        greeter=greeter.user_id,
    )
    assert rep == invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk(
        greeting_attempt=greeting_attempt
    )

    # Claimer performs step 0 second
    claimer_key = PrivateKey.generate()
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer(
            public_key=claimer_key.public_key
        ),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber0WaitPeer(
            public_key=greeter_key.public_key
        )
    )

    # Claimer perform step 1 first
    claimer_nonce = generate_sas_code_nonce()
    hashed_nonce = HashDigest.from_data(claimer_nonce)
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber1SendHashedNonce(
            hashed_nonce=hashed_nonce
        ),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()

    # Greeter performs step 0 once again
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
            public_key=greeter_key.public_key
        ),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber0WaitPeer(
            public_key=claimer_key.public_key
        )
    )

    # Greeter performs step 1 second
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber1GetHashedNonce(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber1SendHashedNonce(
            hashed_nonce=hashed_nonce
        )
    )

    # Greeter performs step 2 first
    greeter_nonce = generate_sas_code_nonce()
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber2SendNonce(
            greeter_nonce=greeter_nonce
        ),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepNotReady()

    # Claimer performs step 1 once again
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber1SendHashedNonce(
            hashed_nonce=hashed_nonce
        ),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber1GetHashedNonce()
    )

    # Claimer performs step 2 second
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber2SendNonce(
            greeter_nonce=greeter_nonce
        )
    )

    # Claimer performs step 3 first
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber3SendNonce(
            claimer_nonce=claimer_nonce
        ),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()

    # Greeter performs step 2 once again
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber2SendNonce(
            greeter_nonce=greeter_nonce
        ),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber2GetNonce()
    )

    # Greeter performs step 3 second
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber3SendNonce(
            claimer_nonce=claimer_nonce
        )
    )

    # Greeter performs step 4 first
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber4WaitPeerTrust(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepNotReady()

    # Claimer performs step 3 once again
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber3SendNonce(
            claimer_nonce=claimer_nonce
        ),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber3GetNonce()
    )

    # Claimer performs step 4 second
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber4SignifyTrust(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber4WaitPeerTrust()
    )

    # Claimer performs step 5 first
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber5WaitPeerTrust(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()

    # Greeter performs step 4 once again
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber4WaitPeerTrust(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber4SignifyTrust()
    )

    # Greeter performs step 5 second
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber5SignifyTrust(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber5WaitPeerTrust()
    )

    # Greeter performs step 6 first
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber6GetPayload(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepNotReady()

    # Claimer performs step 5 once again
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber5WaitPeerTrust(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber5SignifyTrust()
    )

    # Claimer performs step 6 second
    claimer_payload = b"<claimer_payload>"
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber6SendPayload(
            claimer_payload=claimer_payload
        ),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber6GetPayload()
    )

    # Claimer performs step 7 first
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber7GetPayload(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()

    # Greeter performs step 6 once again
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber6GetPayload(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber6SendPayload(
            claimer_payload=claimer_payload
        )
    )

    # Greeter performs step 7 second
    greeter_payload = b"<greeter_payload>"
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber7SendPayload(
            greeter_payload=greeter_payload
        ),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber7GetPayload()
    )

    # Greeter performs step 8 first
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber8WaitPeerAcknowledgment(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepNotReady()

    # Claimer performs step 7 once again
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber7GetPayload(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber7SendPayload(
            greeter_payload=greeter_payload
        )
    )

    # Claimer performs step 8 second
    rep = await claimer.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber8Acknowledge(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(
        greeter_step=invited_cmds.latest.invite_claimer_step.GreeterStepNumber8WaitPeerAcknowledgment()
    )

    # Greeter performs step 8 once again
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber8WaitPeerAcknowledgment(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber8Acknowledge()
    )

    # Greeter complete the invitation
    rep = await greeter.invite_complete(token=invitation_token)
    assert rep == authenticated_cmds.latest.invite_complete.RepOk()

    # Check that invitation is completed for the greeter
    rep = await greeter.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber8WaitPeerAcknowledgment(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepInvitationCompleted()

    # Check that invitation is completed for the claimer
    with pytest.raises(RpcTransportError) as ctx:
        await claimer.invite_claimer_step(
            greeting_attempt=greeting_attempt,
            claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber8Acknowledge(),
        )
    (response,) = ctx.value.args
    assert response.status_code == CustomHttpStatus.InvitationAlreadyUsedOrDeleted.value


async def test_start_greeting_attempt_automatic_cancellation_greeter_first(
    coolorg: CoolorgRpcClients,
) -> None:
    # This is a scenario where the greeter starts first.
    # See `test_invited_invite_claimer_start_greeting_attempt_ok` for the opposite scenario.
    invitation_token = coolorg.invited_alice_dev3.token

    # Greeter goes first
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    first_greeting_attempt = rep.greeting_attempt

    # Claimer goes second, getting the same greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == first_greeting_attempt

    # Greeter starts again, getting a second greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    second_greeting_attempt = rep.greeting_attempt
    assert second_greeting_attempt != first_greeting_attempt

    # First attempt has been properly cancelled
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        first_greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert isinstance(
        rep,
        invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            origin=GreeterOrClaimer.GREETER,
            reason=CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
            timestamp=rep.timestamp,
        )
    )

    # Claimer follows, getting the second greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == second_greeting_attempt

    # Claimer starts again, getting a third greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    third_greeting_attempt = rep.greeting_attempt
    assert third_greeting_attempt != first_greeting_attempt
    assert third_greeting_attempt != second_greeting_attempt

    # Second attempt has been properly cancelled
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        second_greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert isinstance(
        rep,
        authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            origin=GreeterOrClaimer.CLAIMER,
            reason=CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
            timestamp=rep.timestamp,
        )
    )

    # Greeter follows, getting the third greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == third_greeting_attempt


async def test_start_greeting_attempt_automatic_cancellation_claimer_first(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # This is a scenario where the claimer starts first.
    invitation_token = coolorg.invited_alice_dev3.token

    # Claimer goes first
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    first_greeting_attempt = rep.greeting_attempt

    # Greeter goes second, getting the same greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == first_greeting_attempt

    # Claimer starts again, getting a second greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    second_greeting_attempt = rep.greeting_attempt
    assert second_greeting_attempt != first_greeting_attempt

    # First attempt has been properly cancelled
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        first_greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert isinstance(
        rep,
        authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            origin=GreeterOrClaimer.CLAIMER,
            reason=CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
            timestamp=rep.timestamp,
        )
    )

    # Greeter follows, getting the second greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == second_greeting_attempt

    # Greeter starts again, getting a third greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    third_greeting_attempt = rep.greeting_attempt
    assert third_greeting_attempt != first_greeting_attempt
    assert third_greeting_attempt != second_greeting_attempt

    # Second attempt has been properly cancelled
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        second_greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert isinstance(
        rep,
        invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            origin=GreeterOrClaimer.GREETER,
            reason=CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
            timestamp=rep.timestamp,
        )
    )

    # Claimer follows, getting the third greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == third_greeting_attempt
