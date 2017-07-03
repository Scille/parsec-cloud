import attr
from effect2 import Effect, do, TypeDispatcher

from parsec.tools import to_jsonb64, from_jsonb64
from parsec.exceptions import exception_from_status
from parsec.core.backend import BackendCmd


@attr.s
class EBackendMessageNew:
    recipient = attr.ib()
    body = attr.ib()


@attr.s
class EBackendMessageGet:
    recipient = attr.ib()
    offset = attr.ib(default=0)


@attr.s
class Message:
    count = attr.ib()
    body = attr.ib()


@do
def perform_message_new(intent):
    msg = {'recipient': intent.recipient, 'body': to_jsonb64(intent.body)}
    ret = yield Effect(BackendCmd('message_new', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])


@do
def perform_message_get(intent):
    msg = {'recipient': intent.recipient, 'offset': intent.offset}
    ret = yield Effect(BackendCmd('message_get', msg))
    status = ret['status']
    if status != 'ok':
        raise exception_from_status(status)(ret['label'])
    return [Message(msg['count'], from_jsonb64(msg['body'])) for msg in ret['messages']]


def backend_message_dispatcher_factory():
    return TypeDispatcher({
        EBackendMessageGet: perform_message_get,
        EBackendMessageNew: perform_message_new
    })
