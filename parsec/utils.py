import base64
import json
import os
import inspect
from functools import wraps
from uuid import UUID
from pendulum import Pendulum
from nacl.secret import SecretBox
import nacl.utils
from raven.handlers.logbook import SentryHandler


def generate_sym_key():
    return nacl.utils.random(SecretBox.KEY_SIZE)


def to_jsonb64(raw: bytes):
    return base64.encodebytes(raw).decode().replace("\\n", "")


def from_jsonb64(msg: str):
    return base64.decodebytes(msg.encode())


def _json_serial(obj):
    """JSON serializer for objects not serializable by default json code"""
    if isinstance(obj, Pendulum):
        serial = obj.isoformat()
        return serial

    if isinstance(obj, UUID):
        return obj.hex

    elif isinstance(obj, bytes):
        return to_jsonb64(obj)

    raise TypeError("Type %s not serializable" % type(obj))


def ejson_dumps(obj):
    return json.dumps(obj, default=_json_serial, sort_keys=True)


def ejson_loads(raw):
    return json.loads(raw)


class ParsecError(Exception):
    status = "error"

    def __init__(self, reason=None, **kwargs):
        if reason:
            self.kwargs = {**kwargs, "reason": reason}
        else:
            self.kwargs = kwargs

    def to_dict(self):
        return {"status": self.status, **self.kwargs}


def abort(status="bad_message", **kwargs):
    """
    Raises:
        ParsecError
    """
    error = ParsecError(**kwargs)
    error.status = status
    raise error


def get_sentry_handler():
    sentry_url = os.getenv("SENTRY_URL")
    if sentry_url:
        return SentryHandler(sentry_url, level="WARNING")


def _sync_wrap_method(method):
    if inspect.iscoroutinefunction(method):

        @wraps(method)
        async def wrapper(self, *args, **kwargs):
            return await self._trio_portal.run(method, self, *args, **kwargs)

    else:
        return method


class BaseSync:
    def __init__(self, portal, internal):
        self._trio_portal = portal
        self._internal = internal


def sync_wrap(cls, methods):
    nmspc = {mname: _sync_wrap_method(getattr(cls, mname)) for mname in methods}
    sync_cls = type(f"Sync{cls.__name__}", (BaseSync,), nmspc)
    return sync_cls
