import pytest

from parsec.utils import to_jsonb64
from parsec.handshake import (
    HandshakeFormatError, HandshakeBadIdentity,
    ServerHandshake, ClientHandshake
)


def test_good_handshake(alice):
    sh = ServerHandshake()

    ch = ClientHandshake(alice)
    assert sh.state == 'stalled'

    challenge_req = sh.build_challenge_req()
    assert sh.state == 'challenge'

    answer_req = ch.process_challenge_req(challenge_req)

    sh.process_answer_req(answer_req)
    assert sh.state == 'answer'
    assert sh.identity == alice.id
    result_req = sh.build_result_req(alice.verifykey)
    assert sh.state == 'result'

    ch.process_result_req(result_req)


@pytest.mark.parametrize('req', [
    {},
    {'handshake': 'foo', 'challenge': b'1234567890'},
    {'challenge': b'1234567890'},
    {'handshake': 'challenge', 'challenge': b'1234567890', 'foo': 'bar'},
    {'handshake': 'challenge', 'challenge': None},
    {'handshake': 'challenge', 'challenge': 42},
])
def test_process_challenge_req_bad_format(alice, req):
    ch = ClientHandshake(alice)
    with pytest.raises(HandshakeFormatError):
        ch.process_challenge_req(req)


@pytest.mark.parametrize('req', [
    {},
    {'handshake': 'foo', 'result': 'ok'},
    {'result': 'ok'},
    {'handshake': 'result', 'result': 'ok', 'foo': 'bar'},
    {'handshake': 'result', 'result': 'error'},
])
def test_process_result_req_bad_format(alice, req):
    ch = ClientHandshake(alice)
    with pytest.raises(HandshakeFormatError):
        ch.process_result_req(req)


def test_process_result_req_bad_identity(alice):
    ch = ClientHandshake(alice)
    with pytest.raises(HandshakeBadIdentity):
        ch.process_result_req({
            'handshake': 'result',
            'result': 'bad_identity',
        })


@pytest.mark.parametrize('req', [
    {},
    {'handshake': 'foo', 'identity': 'alice@test', 'answer': b'1234567890'},
    {'handshake': 'answer', 'identity': 'alice@test', 'answer': b'1234567890', 'foo': 'bar'},
    {'handshake': 'answer', 'identity': 'alice@test', 'answer': 42},
    {'handshake': 'answer', 'identity': 'alice@test', 'answer': None},
    {'handshake': 'answer', 'answer': b'1234567890'},
    {'handshake': 'answer', 'identity': 'alice@test'},
])
def test_process_answer_req_bad_format(req):
    sh = ServerHandshake()
    sh.build_challenge_req()
    with pytest.raises(HandshakeFormatError):
        sh.process_answer_req(req)


def test_build_result_req_bad_key(alice, bob):
    sh = ServerHandshake()
    sh.build_challenge_req()
    sh.process_answer_req({
        'handshake': 'answer',
        'identity': alice.id,
        'answer': to_jsonb64(alice.signkey.sign(sh.challenge))
    })
    with pytest.raises(HandshakeFormatError):
        sh.build_result_req(bob.verifykey)


def test_build_result_req_bad_challenge(alice):
    sh = ServerHandshake()
    sh.build_challenge_req()
    sh.process_answer_req({
        'handshake': 'answer',
        'identity': alice.id,
        'answer': to_jsonb64(alice.signkey.sign(sh.challenge + b'-dummy'))
    })
    with pytest.raises(HandshakeFormatError):
        sh.build_result_req(alice.verifykey)


def test_build_bad_identity_result_req(alice):
    sh = ServerHandshake()
    sh.build_challenge_req()
    sh.process_answer_req({
        'handshake': 'answer',
        'identity': alice.id,
        'answer': to_jsonb64(alice.signkey.sign(sh.challenge + b'-dummy'))
    })
    req = sh.build_bad_identity_result_req()
    assert req == {
        'handshake': 'result',
        'result': 'bad_identity',
    }
