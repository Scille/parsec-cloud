# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    HashDigest,
    PublicKey,
    authenticated_cmds,
    invited_cmds,
)
from parsec.components.invite import process_claimer_step, process_greeter_step


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
    greeter_step_0 = authenticated_cmds.latest.invite_greeter_step.GreeterStepWaitPeer(
        public_key=greeter_key
    )
    index, data, convert = process_greeter_step(greeter_step_0)
    assert index == 0
    assert data == greeter_key.encode() == greeter_snapshot[index]
    assert convert(
        claimer_snapshot[index]
    ) == authenticated_cmds.latest.invite_greeter_step.ClaimerStepWaitPeer(public_key=claimer_key)

    claimer_step_0 = invited_cmds.latest.invite_claimer_step.ClaimerStepWaitPeer(
        public_key=claimer_key
    )
    index, data, convert = process_claimer_step(claimer_step_0)
    assert index == 0
    assert data == claimer_key.encode() == claimer_snapshot[index]
    assert convert(
        greeter_snapshot[index]
    ) == invited_cmds.latest.invite_claimer_step.GreeterStepWaitPeer(public_key=greeter_key)

    # Step 1
    greeter_step_1 = authenticated_cmds.latest.invite_greeter_step.GreeterStepGetHashedNonce()
    index, data, convert = process_greeter_step(greeter_step_1)
    assert index == 1
    assert data == b"" == greeter_snapshot[index]
    assert convert(
        claimer_snapshot[index]
    ) == authenticated_cmds.latest.invite_greeter_step.ClaimerStepSendHashedNonce(
        hashed_nonce=hashed_nonce
    )

    claimer_step_1 = invited_cmds.latest.invite_claimer_step.ClaimerStepSendHashedNonce(
        hashed_nonce=hashed_nonce
    )
    index, data, convert = process_claimer_step(claimer_step_1)
    assert index == 1
    assert data == hashed_nonce.digest == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepGetHashedNonce()
    )

    # Step 2
    greeter_step_2 = authenticated_cmds.latest.invite_greeter_step.GreeterStepSendNonce(
        greeter_nonce=greeter_nonce
    )
    index, data, convert = process_greeter_step(greeter_step_2)
    assert index == 2
    assert data == greeter_nonce == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepGetNonce()
    )

    claimer_step_2 = invited_cmds.latest.invite_claimer_step.ClaimerStepGetNonce()
    index, data, convert = process_claimer_step(claimer_step_2)
    assert index == 2
    assert data == b"" == claimer_snapshot[index]
    assert convert(
        greeter_snapshot[index]
    ) == invited_cmds.latest.invite_claimer_step.GreeterStepSendNonce(greeter_nonce=greeter_nonce)

    # Step 3
    greeter_step_3 = authenticated_cmds.latest.invite_greeter_step.GreeterStepGetNonce()
    index, data, convert = process_greeter_step(greeter_step_3)
    assert index == 3
    assert data == b"" == greeter_snapshot[index]
    assert convert(
        claimer_snapshot[index]
    ) == authenticated_cmds.latest.invite_greeter_step.ClaimerStepSendNonce(
        claimer_nonce=claimer_nonce
    )

    claimer_step_3 = invited_cmds.latest.invite_claimer_step.ClaimerStepSendNonce(
        claimer_nonce=claimer_nonce
    )
    index, data, convert = process_claimer_step(claimer_step_3)
    assert index == 3
    assert data == claimer_nonce == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepGetNonce()
    )

    # Step 4
    greeter_step_4 = authenticated_cmds.latest.invite_greeter_step.GreeterStepWaitPeerTrust()
    index, data, convert = process_greeter_step(greeter_step_4)
    assert index == 4
    assert data == b"" == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepSignifyTrust()
    )

    claimer_step_4 = invited_cmds.latest.invite_claimer_step.ClaimerStepSignifyTrust()
    index, data, convert = process_claimer_step(claimer_step_4)
    assert index == 4
    assert data == b"" == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepWaitPeerTrust()
    )

    # Step 5
    greeter_step_5 = authenticated_cmds.latest.invite_greeter_step.GreeterStepSignifyTrust()
    index, data, convert = process_greeter_step(greeter_step_5)
    assert index == 5
    assert data == b"" == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepWaitPeerTrust()
    )

    claimer_step_5 = invited_cmds.latest.invite_claimer_step.ClaimerStepWaitPeerTrust()
    index, data, convert = process_claimer_step(claimer_step_5)
    assert index == 5
    assert data == b"" == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepSignifyTrust()
    )

    # Step 6
    greeter_step_6 = authenticated_cmds.latest.invite_greeter_step.GreeterStepGetPayload()
    index, data, convert = process_greeter_step(greeter_step_6)
    assert index == 6
    assert data == b"" == greeter_snapshot[index]
    assert convert(
        claimer_snapshot[index]
    ) == authenticated_cmds.latest.invite_greeter_step.ClaimerStepSendPayload(
        claimer_payload=claimer_payload
    )

    claimer_step_6 = invited_cmds.latest.invite_claimer_step.ClaimerStepSendPayload(
        claimer_payload=claimer_payload
    )
    index, data, convert = process_claimer_step(claimer_step_6)
    assert index == 6
    assert data == claimer_payload == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepGetPayload()
    )

    # Step 7
    greeter_step_7 = authenticated_cmds.latest.invite_greeter_step.GreeterStepSendPayload(
        greeter_payload=greeter_payload
    )
    index, data, convert = process_greeter_step(greeter_step_7)
    assert index == 7
    assert data == greeter_payload == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepGetPayload()
    )

    claimer_step_7 = invited_cmds.latest.invite_claimer_step.ClaimerStepGetPayload()
    index, data, convert = process_claimer_step(claimer_step_7)
    assert index == 7
    assert data == b"" == claimer_snapshot[index]
    assert convert(
        greeter_snapshot[index]
    ) == invited_cmds.latest.invite_claimer_step.GreeterStepSendPayload(
        greeter_payload=greeter_payload
    )

    # Step 8
    greeter_step_8 = (
        authenticated_cmds.latest.invite_greeter_step.GreeterStepWaitPeerAcknowledgment()
    )
    index, data, convert = process_greeter_step(greeter_step_8)
    assert index == 8
    assert data == b"" == greeter_snapshot[index]
    assert (
        convert(claimer_snapshot[index])
        == authenticated_cmds.latest.invite_greeter_step.ClaimerStepAcknowledge()
    )

    claimer_step_8 = invited_cmds.latest.invite_claimer_step.ClaimerStepAcknowledge()
    index, data, convert = process_claimer_step(claimer_step_8)
    assert index == 8
    assert data == b"" == claimer_snapshot[index]
    assert (
        convert(greeter_snapshot[index])
        == invited_cmds.latest.invite_claimer_step.GreeterStepWaitPeerAcknowledgment()
    )
