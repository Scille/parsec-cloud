import pytest
from effect2.testing import perform_sequence, const

from parsec.tools import to_jsonb64
from parsec.core.backend import BackendCmd
from parsec.core.backend_message import (
    Message, perform_message_new, perform_message_get, EBackendMessageGet, EBackendMessageNew)


def test_perform_message_get():
    eff = perform_message_get(EBackendMessageGet('John', 1))
    backend_response = {
        'status': 'ok',
        'messages': [
            {'count': 1, 'body': to_jsonb64(b'body 1')},
            {'count': 2, 'body': to_jsonb64(b'body 2')}
        ]
    }
    sequence = [
        (BackendCmd('message_get', {'offset': 1, 'recipient': 'John'}), const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == [Message(1, b'body 1'), Message(2, b'body 2')]


def test_perform_message_new():
    eff = perform_message_new(EBackendMessageNew('John', b'my body'))
    backend_response = {'status': 'ok'}
    sequence = [
        (BackendCmd('message_new', {'recipient': 'John', 'body': to_jsonb64(b'my body')}),
            const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None
