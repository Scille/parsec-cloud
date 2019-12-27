# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
from functools import wraps

from parsec.api.protocol import ProtocolError, InvalidMessageError


def anonymous_api(fn):
    fn._anonymous_api_allowed = True
    return fn


def check_anonymous_api_allowed(fn):
    if not getattr(fn, "_anonymous_api_allowed", False):
        raise RuntimeError(
            f"Trying to add {fn!r} to the anonymous api "
            "(need to use @anonymous_api decorator for this)."
        )


def catch_protocol_errors(fn):
    @wraps(fn)
    async def wrapper(*args, **kwargs):
        try:
            return await fn(*args, **kwargs)

        except InvalidMessageError as exc:
            return {"status": "bad_message", "errors": exc.errors, "reason": "Invalid message."}

        except ProtocolError as exc:
            return {"status": "bad_message", "reason": str(exc)}

    return wrapper


class CancelledByNewRequest(Exception):
    def __init__(self, new_raw_req):
        self.new_raw_req = new_raw_req


async def run_with_breathing_transport(transport, fn, *args, **kwargs):
    """
    This is kind of a special case here:
    unlike other requests this one is going to (potentially) take
    a long time to complete. In the meantime we must monitor the
    connection with the client in order to make sure it is still
    online and handles websocket pings
    """

    rep = None

    async def _keep_transport_breathing():
        # If a command is received, the client is violating the
        # request/reply pattern. We consider this as an order to stop
        # listening events.
        raw_req = await transport.recv()
        raise CancelledByNewRequest(raw_req)

    async def _do_fn(cancel_scope):
        nonlocal rep
        rep = await fn(*args, **kwargs)
        cancel_scope.cancel()

    # No need for a service nursery here given no await is done by the main
    # task from within the async with block
    async with trio.open_nursery() as nursery:
        nursery.start_soon(_do_fn, nursery.cancel_scope)
        nursery.start_soon(_keep_transport_breathing)

    return rep
