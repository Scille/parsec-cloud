import pytest

from parsec.utils import to_jsonb64
from parsec.api.protocole.handshake import (
    HandshakeFormatError,
    HandshakeBadIdentity,
    ServerHandshake,
    ClientHandshake,
    AnonymousClientHandshake,
)


def test_good_handshake(alice):
    sh = ServerHandshake()

    ch = ClientHandshake(alice.device_id, alice.signing_key)
    assert sh.state == "stalled"

    challenge_req = sh.build_challenge_req()
    assert sh.state == "challenge"

    answer_req = ch.process_challenge_req(challenge_req)

    sh.process_answer_req(answer_req)
    assert sh.state == "answer"
    assert sh.identity == alice.device_id
    assert not sh.is_anonymous()
    result_req = sh.build_result_req(alice.verify_key)
    assert sh.state == "result"

    ch.process_result_req(result_req)


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
    ch = ClientHandshake(alice.device_id, alice.signing_key)
    with pytest.raises(HandshakeFormatError):
        ch.process_challenge_req(req)


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
    ch = ClientHandshake(alice.device_id, alice.signing_key)
    with pytest.raises(HandshakeFormatError):
        ch.process_result_req(req)


def test_process_result_req_bad_identity(alice):
    ch = ClientHandshake(alice.device_id, alice.signing_key)
    with pytest.raises(HandshakeBadIdentity):
        ch.process_result_req({"handshake": "result", "result": "bad_identity"})


@pytest.mark.parametrize(
    "req",
    [
        {},
        {"handshake": "foo", "identity": "alice@test", "answer": "MTIzNDU2Nzg5MA=="},
        {
            "handshake": "answer",
            "identity": "alice@test",
            "answer": "MTIzNDU2Nzg5MA==",
            "foo": "bar",
        },
        {"handshake": "answer", "identity": "alice@test", "answer": 42},
        {"handshake": "answer", "identity": "dummy", "answer": "MTIzNDU2Nzg5MA=="},
    ],
)
def test_process_answer_req_bad_format(req):
    sh = ServerHandshake()
    sh.build_challenge_req()
    with pytest.raises(HandshakeFormatError):
        sh.process_answer_req(req)


def test_build_result_req_bad_key(alice, bob):
    sh = ServerHandshake()
    sh.build_challenge_req()
    sh.process_answer_req(
        {
            "handshake": "answer",
            "identity": alice.device_id,
            "answer": to_jsonb64(alice.signing_key.sign(sh.challenge)),
        }
    )
    with pytest.raises(HandshakeFormatError):
        sh.build_result_req(bob.verify_key)


def test_build_result_req_answer_is_none(alice):
    sh = ServerHandshake()
    sh.build_challenge_req()
    sh.process_answer_req({"handshake": "answer", "identity": alice.device_id, "answer": None})
    with pytest.raises(HandshakeFormatError):
        sh.build_result_req(alice.verify_key)


def test_build_result_req_bad_challenge(alice):
    sh = ServerHandshake()
    sh.build_challenge_req()
    sh.process_answer_req(
        {
            "handshake": "answer",
            "identity": alice.device_id,
            "answer": to_jsonb64(alice.signing_key.sign(sh.challenge + b"-dummy")),
        }
    )
    with pytest.raises(HandshakeFormatError):
        sh.build_result_req(alice.verify_key)


def test_build_bad_identity_result_req(alice):
    sh = ServerHandshake()
    sh.build_challenge_req()
    sh.process_answer_req(
        {
            "handshake": "answer",
            "identity": alice.device_id,
            "answer": to_jsonb64(alice.signing_key.sign(sh.challenge + b"-dummy")),
        }
    )
    req = sh.build_bad_identity_result_req()
    assert req == {"handshake": "result", "result": "bad_identity"}


def test_build_bad_format_result_req(alice):
    sh = ServerHandshake()
    sh.build_challenge_req()
    sh.process_answer_req(
        {
            "handshake": "answer",
            "identity": alice.device_id,
            "answer": to_jsonb64(alice.signing_key.sign(sh.challenge + b"-dummy")),
        }
    )
    req = sh.build_bad_format_result_req()
    assert req == {"handshake": "result", "result": "bad_format"}


def test_good_anonymous_handshake():
    sh = ServerHandshake()

    ch = AnonymousClientHandshake()
    assert sh.state == "stalled"

    challenge_req = sh.build_challenge_req()
    assert sh.state == "challenge"

    answer_req = ch.process_challenge_req(challenge_req)

    sh.process_answer_req(answer_req)
    assert sh.state == "answer"
    assert sh.identity is None
    assert sh.is_anonymous()
    result_req = sh.build_result_req()
    assert sh.state == "result"

    ch.process_result_req(result_req)


# TODO: test with revoked device
# TODO: test with user with all devices revoked
