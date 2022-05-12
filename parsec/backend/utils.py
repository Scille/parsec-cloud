# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import trio
from enum import Enum
from functools import wraps
from typing_extensions import Final, Literal
from typing import Sequence, Dict, Callable

from parsec.api.protocol import ProtocolError, InvalidMessageError
from parsec.api.version import API_V1_VERSION, API_V2_VERSION

PEER_EVENT_MAX_WAIT = 3  # 5mn
ALLOWED_API_VERSIONS = {API_V1_VERSION.version, API_V2_VERSION.version}


# Enumeration used to check access rights for a given kind of operation
OperationKind = Enum("OperationKind", "DATA_READ DATA_WRITE MAINTENANCE")

ClientType = Enum(
    "ClientType", "AUTHENTICATED INVITED ANONYMOUS APIV1_ANONYMOUS APIV1_ADMINISTRATION"
)


def api(
    cmd: str,
    *,
    long_request: bool = False,
    client_types: Sequence[ClientType] = (ClientType.AUTHENTICATED,),
):
    def wrapper(fn):
        if long_request:

            @wraps(fn)
            async def wrapped(self, client_ctx, *args, **kwargs):
                return await run_with_breathing_transport(
                    client_ctx.transport, fn, self, client_ctx, *args, **kwargs
                )

        else:
            wrapped = fn

        assert not hasattr(wrapped, "_api_info")
        wrapped._api_info = {"cmd": cmd, "client_types": client_types}
        return wrapped

    return wrapper


def collect_apis(*components) -> Dict[ClientType, Dict[str, Callable]]:
    apis: Dict[ClientType, Dict[str, Callable]] = {}
    for component in components:
        for methname in dir(component):
            meth = getattr(component, methname)
            info = getattr(meth, "_api_info", None)
            if not info:
                continue

            for client_type in info["client_types"]:
                if client_type not in apis:
                    apis[client_type] = {}

                assert info["cmd"] not in apis[client_type]
                apis[client_type][info["cmd"]] = meth
    return apis


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

    async with trio.open_service_nursery() as nursery:
        nursery.start_soon(_do_fn, nursery.cancel_scope)
        nursery.start_soon(_keep_transport_breathing)

    return rep


# Unset singleton used as default value in function parameter when `None`
# can be a valid value.
# We implement this as an enum to satisfy type checker (see
# https://github.com/python/typing/issues/689#issuecomment-561425237)
UnsetType = Enum("UnsetType", "Unset")
Unset: Final = UnsetType.Unset
UnsetType = Literal[UnsetType.Unset]
