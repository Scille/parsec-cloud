# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from unittest.mock import ANY

from parsec.api.protocol.base import packb, unpackb, InvalidMessageError
from parsec.api.protocol.handshake import (
    HandshakeFailedChallenge,
    HandshakeBadIdentity,
    HandshakeBadAdministrationToken,
    HandshakeRVKMismatch,
    HandshakeRevokedDevice,
    HandshakeAPIVersionError,
    ServerHandshake,
    BaseClientHandshake,
    AuthenticatedClientHandshake,
    AnonymousClientHandshake,
    AdministrationClientHandshake,
)
from parsec import __api_version__


def test_good_handshake(alice):
    sh = ServerHandshake()

    ch = AuthenticatedClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    assert sh.state == "stalled"

    challenge_req = sh.build_challenge_req()
    assert sh.state == "challenge"

    answer_req = ch.process_challenge_req(challenge_req)

    sh.process_answer_req(answer_req)
    assert sh.state == "answer"
    assert sh.answer_type == "authenticated"
    assert sh.answer_data == {
        "answer": ANY,
        "api_version": __api_version__,
        "organization_id": alice.organization_id,
        "device_id": alice.device_id,
        "rvk": alice.root_verify_key,
    }
    result_req = sh.build_result_req(alice.verify_key)
    assert sh.state == "result"

    ch.process_result_req(result_req)
    assert sh.client_api_version == __api_version__


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
    assert sh.answer_type == "anonymous"
    if check_rvk:
        assert sh.answer_data == {
            "api_version": __api_version__,
            "organization_id": coolorg.organization_id,
            "rvk": coolorg.root_verify_key,
        }
    else:
        assert sh.answer_data == {
            "api_version": __api_version__,
            "organization_id": coolorg.organization_id,
            "rvk": None,
        }
    result_req = sh.build_result_req()
    assert sh.state == "result"

    ch.process_result_req(result_req)
    assert sh.client_api_version == __api_version__


def test_good_administration_handshake():
    admin_token = "Xx" * 16
    sh = ServerHandshake()

    ch = AdministrationClientHandshake(admin_token)
    assert sh.state == "stalled"

    challenge_req = sh.build_challenge_req()
    assert sh.state == "challenge"

    answer_req = ch.process_challenge_req(challenge_req)

    sh.process_answer_req(answer_req)
    assert sh.state == "answer"
    assert sh.answer_type == "administration"
    assert sh.answer_data == {"api_version": __api_version__, "token": admin_token}
    result_req = sh.build_result_req()
    assert sh.state == "result"

    ch.process_result_req(result_req)
    assert sh.client_api_version == __api_version__


# 1) Server build challenge (nothing more to test...)


# 2) Client process challenge


@pytest.mark.parametrize(
    "req",
    [
        {},
        {"handshake": "foo", "challenge": b"1234567890", "api_version": __api_version__},
        {"handshake": "challenge", "challenge": b"1234567890"},
        {"challenge": b"1234567890"},
        {"challenge": b"1234567890", "api_version": __api_version__},
        {
            "handshake": "challenge",
            "challenge": b"1234567890",
            "api_version": __api_version__,
            "foo": "bar",
        },
        {"handshake": "challenge", "challenge": None},
        {"handshake": "challenge", "challenge": None, "api_version": __api_version__},
        {"handshake": "challenge", "challenge": 42, "api_version": __api_version__},
        {"handshake": "challenge", "challenge": b"1234567890"},
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "nosemver"},
    ],
)
def test_process_challenge_req_bad_format(alice, req):
    ch = AuthenticatedClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    with pytest.raises(InvalidMessageError):
        ch.process_challenge_req(packb(req))


# 2-b) Client check API version


@pytest.mark.parametrize(
    "req",
    [
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "0.1.1"},
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "0.100.0"},
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "2.0.0"},
    ],
)
def test_process_challenge_req_bad_semver(alice, req, monkeypatch):
    monkeypatch.setattr("parsec.api.protocol.handshake.__api_version__", "1.1.1")
    ch = AuthenticatedClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    with pytest.raises(HandshakeAPIVersionError):
        ch.process_challenge_req(packb(req))


@pytest.mark.parametrize(
    "req",
    [
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "1.0.0"},
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "1.1.0"},
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "1.1.2"},
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "1.1.10"},
        {"handshake": "challenge", "challenge": b"1234567890", "api_version": "1.2.0"},
    ],
)
def test_process_challenge_req_good_semver(alice, req, monkeypatch):
    monkeypatch.setattr("parsec.api.protocol.handshake.__api_version__", "1.1.1")
    ch = AuthenticatedClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    ch.process_challenge_req(packb(req))


# 3) Server process answer


@pytest.mark.parametrize(
    "req",
    [
        {},
        {"handshake": "answer", "type": "dummy"},  # Invalid type
        # Authenticated answer
        {
            "handshake": "answer",
            "type": "authenticated",
            "organization_id": "<good>",
            "device_id": "<good>",
            # Missing rvk
            "answer": b"good answer",
        },
        {
            "handshake": "answer",
            "type": "authenticated",
            "organization_id": "<good>",
            # Missing device_id
            "rvk": "<good>",
            "answer": b"good answer",
        },
        {
            "handshake": "answer",
            "type": "authenticated",
            "organization_id": "<good>",
            "device_id": "<good>",
            "rvk": "<good>",
            # Missing answer
        },
        {
            "handshake": "answer",
            "type": "authenticated",
            "organization_id": "<good>",
            "device_id": "<good>",
            "rvk": "<good>",
            "answer": 42,  # Bad type
        },
        {
            "handshake": "answer",
            "type": "authenticated",
            "organization_id": "<good>",
            "device_id": "<good>",
            "rvk": "<good>",
            "answer": b"good answer",
            "foo": "bar",  # Unknown field
        },
        {
            "handshake": "answer",
            "type": "authenticated",
            "organization_id": "<good>",
            "device_id": "dummy",  # Invalid DeviceID
            "rvk": "<good>",
            "answer": b"good answer",
        },
        {
            "handshake": "answer",
            "type": "authenticated",
            "organization_id": "<good>",
            "device_id": "<good>",
            "rvk": b"dummy",  # Invalid VerifyKey
            "answer": b"good answer",
        },
        # Anonymous answer
        {
            "handshake": "answer",
            "type": "anonymous",
            "organization_id": "<good>",
            "rvk": b"dummy",  # Invalid VerifyKey
        },
        {
            "handshake": "answer",
            "type": "anonymous",
            "organization_id": "d@mmy",  # Invalid OrganizationID
            "rvk": "<good>",
        },
        {
            "handshake": "answer",
            "type": "anonymous",
            "organization_id": "<good>",
            "rvk": "<good>",
            "dummy": "whatever",  # Unknown field
        },
        # Admin answer
        {
            "handshake": "answer",
            "type": "administration",
            # Missing token
        },
        {
            "handshake": "answer",
            "type": "administration",
            "token": "<good>",
            "dummy": "whatever",  # Unknown field
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
        "type": "authenticated",
        "api_version": __api_version__,
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
        "type": "authenticated",
        "api_version": __api_version__,
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
        ("build_bad_administration_token_result_req", "bad_admin_token"),
    ],
)
def test_build_bad_outcomes(alice, method, expected_result):
    sh = ServerHandshake()
    sh.build_challenge_req()
    answer = {
        "handshake": "answer",
        "type": "authenticated",
        "api_version": __api_version__,
        "organization_id": alice.organization_id,
        "device_id": alice.device_id,
        "rvk": alice.root_verify_key.encode(),
        "answer": alice.signing_key.sign(sh.challenge),
    }
    sh.process_answer_req(packb(answer))
    req = getattr(sh, method)()
    assert unpackb(req) == {"handshake": "result", "result": expected_result, "help": ANY}


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
def test_process_result_req_bad_format(req):
    ch = BaseClientHandshake()
    with pytest.raises(InvalidMessageError):
        ch.process_result_req(packb(req))


@pytest.mark.parametrize(
    "result,exc_cls",
    [
        ("bad_identity", HandshakeBadIdentity),
        ("rvk_mismatch", HandshakeRVKMismatch),
        ("revoked_device", HandshakeRevokedDevice),
        ("bad_admin_token", HandshakeBadAdministrationToken),
        ("dummy", InvalidMessageError),
    ],
)
def test_process_result_req_bad_outcome(result, exc_cls):
    ch = BaseClientHandshake()
    with pytest.raises(exc_cls):
        ch.process_result_req(packb({"handshake": "result", "result": result}))


# TODO: test with revoked device
# TODO: test with user with all devices revoked
