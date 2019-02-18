# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.protocole.base import packb, unpackb, InvalidMessageError
from parsec.api.protocole.handshake import (
    HandshakeFailedChallenge,
    HandshakeBadIdentity,
    HandshakeRVKMismatch,
    ServerHandshake,
    ClientHandshake,
    AnonymousClientHandshake,
)


def test_good_handshake(alice):
    sh = ServerHandshake()

    ch = ClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    assert sh.state == "stalled"

    challenge_req = sh.build_challenge_req()
    assert sh.state == "challenge"

    answer_req = ch.process_challenge_req(challenge_req)

    sh.process_answer_req(answer_req)
    assert sh.state == "answer"
    assert sh.organization_id == alice.organization_id
    assert sh.device_id == alice.device_id
    assert not sh.is_anonymous()
    result_req = sh.build_result_req(alice.verify_key)
    assert sh.state == "result"

    ch.process_result_req(result_req)


@pytest.mark.parametrize("check_rvk", (True, False))
def test_good_anonymous_handshake(coolorg, check_rvk):
    sh = ServerHandshake()

    if check_rvk:
        ch = AnonymousClientHandshake(coolorg.organization_id, coolorg.root_verify_key)
    else:
        ch = AnonymousClientHandshake(coolorg.organization_id)
    assert sh.state == "stalled"

    challenge_req = sh.build_challenge_req()
    assert sh.state == "challenge"

    answer_req = ch.process_challenge_req(challenge_req)

    sh.process_answer_req(answer_req)
    assert sh.state == "answer"
    assert sh.organization_id == coolorg.organization_id
    assert sh.root_verify_key == (coolorg.root_verify_key if check_rvk else None)
    assert sh.device_id is None
    assert sh.is_anonymous()
    result_req = sh.build_result_req()
    assert sh.state == "result"

    ch.process_result_req(result_req)


# 1) Server build challenge (nothing more to test...)


# 2) Client process challenge


@pytest.mark.parametrize(
    "req",
    [
        {},
        {"handshake": "foo", "challenge": b"1234567890"},
        {"challenge": b"1234567890"},
        {"handshake": "challenge", "challenge": b"1234567890", "foo": "bar"},
        {"handshake": "challenge", "challenge": None},
        {"handshake": "challenge", "challenge": 42},
    ],
)
def test_process_challenge_req_bad_format(alice, req):
    ch = ClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    with pytest.raises(InvalidMessageError):
        ch.process_challenge_req(packb(req))


# 3) Server process answer


@pytest.mark.parametrize(
    "req",
    [
        {},
        {
            "handshake": "answer",
            "organization_id": "<good>",
            "device_id": "<good>",
            "answer": "MTIzNDU2Nzg5MA==",
        },
        {
            "handshake": "answer",
            "organization_id": "<good>",
            "rvk": "<good>",
            "answer": "MTIzNDU2Nzg5MA==",
        },
        {
            "handshake": "answer",
            "organization_id": "<good>",
            "device_id": "<good>",
            "rvk": "<good>",
        },
        {
            "handshake": "answer",
            "organization_id": "<good>",
            "device_id": "<good>",
            "rvk": "<good>",
            "answer": 42,
        },
        {
            "handshake": "answer",
            "device_id": "<good>",
            "rvk": "<good>",
            "answer": "MTIzNDU2Nzg5MA==",
            "foo": "bar",
        },
        {
            "handshake": "answer",
            "organization_id": "<good>",
            "device_id": "<good>",
            "rvk": "<good>",
            "answer": "MTIzNDU2Nzg5MA==",
            "foo": "bar",
        },
        {
            "handshake": "answer",
            "organization_id": "<good>",
            "device_id": "dummy",
            "rvk": "<good>",
            "answer": "MTIzNDU2Nzg5MA==",
        },
        {
            "handshake": "answer",
            "organization_id": "<good>",
            "device_id": "<good>",
            "rvk": b"dummy",
            "answer": "MTIzNDU2Nzg5MA==",
        },
    ],
)
def test_process_answer_req_bad_format(req, alice):
    for key, good_value in [
        ("organization_id", alice.organization_id),
        ("device_id", alice.device_id),
        ("rvk", alice.root_verify_key.encode()),
    ]:
        if req.get(key) == "<good>":
            req[key] = good_value
    sh = ServerHandshake()
    sh.build_challenge_req()
    with pytest.raises(InvalidMessageError):
        sh.process_answer_req(packb(req))


# 4) Server build result


def test_build_result_req_bad_key(alice, bob):
    sh = ServerHandshake()
    sh.build_challenge_req()
    answer = {
        "handshake": "answer",
        "organization_id": alice.organization_id,
        "device_id": alice.device_id,
        "rvk": alice.root_verify_key.encode(),
        "answer": alice.signing_key.sign(sh.challenge),
    }
    sh.process_answer_req(packb(answer))
    with pytest.raises(HandshakeFailedChallenge):
        sh.build_result_req(bob.verify_key)


def test_build_result_req_bad_challenge(alice):
    sh = ServerHandshake()
    sh.build_challenge_req()
    answer = {
        "handshake": "answer",
        "organization_id": alice.organization_id,
        "device_id": alice.device_id,
        "rvk": alice.root_verify_key.encode(),
        "answer": alice.signing_key.sign(sh.challenge + b"-dummy"),
    }
    sh.process_answer_req(packb(answer))
    with pytest.raises(HandshakeFailedChallenge):
        sh.build_result_req(alice.verify_key)


@pytest.mark.parametrize(
    "method,expected_result",
    [
        ("build_bad_format_result_req", "bad_format"),
        ("build_bad_identity_result_req", "bad_identity"),
        ("build_rvk_mismatch_result_req", "rvk_mismatch"),
        ("build_revoked_device_result_req", "revoked_device"),
    ],
)
def test_build_bad_outcomes(alice, method, expected_result):
    sh = ServerHandshake()
    sh.build_challenge_req()
    answer = {
        "handshake": "answer",
        "organization_id": alice.organization_id,
        "device_id": alice.device_id,
        "rvk": alice.root_verify_key.encode(),
        "answer": alice.signing_key.sign(sh.challenge),
    }
    sh.process_answer_req(packb(answer))
    req = getattr(sh, method)()
    assert unpackb(req) == {"handshake": "result", "result": expected_result}


# 5) Client process result


@pytest.mark.parametrize(
    "req",
    [
        {},
        {"handshake": "foo", "result": "ok"},
        {"result": "ok"},
        {"handshake": "result", "result": "ok", "foo": "bar"},
        {"handshake": "result", "result": "error"},
    ],
)
def test_process_result_req_bad_format(alice, req):
    ch = ClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    with pytest.raises(InvalidMessageError):
        ch.process_result_req(packb(req))


@pytest.mark.parametrize(
    "result,exc_cls",
    [
        ("bad_identity", HandshakeBadIdentity),
        ("rvk_mismatch", HandshakeRVKMismatch),
        ("dummy", InvalidMessageError),
    ],
)
def test_process_result_req_bad_outcome(alice, result, exc_cls):
    ch = ClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    with pytest.raises(exc_cls):
        ch.process_result_req(packb({"handshake": "result", "result": result}))


# TODO: test with revoked device
# TODO: test with user with all devices revoked
