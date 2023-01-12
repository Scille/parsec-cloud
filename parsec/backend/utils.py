# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

from enum import Enum
from functools import wraps
from typing import TYPE_CHECKING, Any, Awaitable, Callable, Dict, Sequence, TypeVar, Union

from quart import Websocket
from trio import CancelScope
from typing_extensions import Final, Literal, ParamSpec

from parsec._parsec import AuthenticatedAnyCmdReq, ClientType, InvitedAnyCmdReq, ProtocolError
from parsec.api.protocol import InvalidMessageError
from parsec.api.version import API_V3_VERSION
from parsec.serde.packing import packb, unpackb
from parsec.utils import open_service_nursery

if TYPE_CHECKING:
    from parsec.backend.client_context import BaseClientContext

    Ctx = TypeVar("Ctx", bound=BaseClientContext)
    T = TypeVar("T")
    P = ParamSpec("P")
    CmdReq = Union[InvitedAnyCmdReq, AuthenticatedAnyCmdReq]

PEER_EVENT_MAX_WAIT = 3  # 5mn
ALLOWED_API_VERSIONS = {
    API_V3_VERSION.version,
}


# Enumeration used to check access rights for a given kind of operation
OperationKind = Enum("OperationKind", "DATA_READ DATA_WRITE MAINTENANCE")


# TODO: temporary hack that should be removed once all cmds are typed, at this point we
# will be able to do this handling directly into `BackendApp._handle_client_websocket_loop`
def api_typed_msg_adapter(
    req_cls: Any, rep_cls: Any
) -> Callable[
    [Callable[[T, Ctx, Any], Awaitable[Any]]],
    Callable[[T, Ctx, dict[str, object]], Awaitable[dict[str, object]]],
]:
    def _api_typed_msg_adapter(
        fn: Callable[[T, Ctx, Any], Awaitable[Any]]
    ) -> Callable[[T, Ctx, dict[str, object]], Awaitable[dict[str, object]]]:
        @wraps(fn)
        async def wrapper(self: T, client_ctx: Ctx, msg: dict[str, object]) -> dict[str, object]:
            # Here packb&unpackb should never fail given they are only undoing
            # work we've just done in another layer
            if client_ctx.TYPE == ClientType.INVITED:
                from parsec._parsec import InvitedAnyCmdReq

                typed_req = InvitedAnyCmdReq.load(packb(msg))
            elif client_ctx.TYPE == ClientType.ANONYMOUS:
                from parsec._parsec import AnonymousAnyCmdReq

                typed_req = AnonymousAnyCmdReq.load(packb(msg))
            else:
                from parsec._parsec import AuthenticatedAnyCmdReq

                typed_req = AuthenticatedAnyCmdReq.load(packb(msg))

            assert isinstance(typed_req, req_cls)
            typed_rep = await fn(self, client_ctx, typed_req)
            return unpackb(typed_rep.dump())

        return wrapper

    return _api_typed_msg_adapter


def api(
    cmd: str,
    *,
    cancel_on_client_sending_new_cmd: bool = False,
    client_types: Sequence[ClientType] = (ClientType.AUTHENTICATED,),
) -> Callable[[Callable[P, T]], Callable[P, T]]:
    def wrapper(fn: Callable[P, T]) -> Callable[P, T]:
        assert not hasattr(fn, "_api_info")
        fn._api_info = {  # type: ignore[attr-defined]
            "cmd": cmd,
            "client_types": client_types,
            "cancel_on_client_sending_new_cmd": cancel_on_client_sending_new_cmd,
        }
        return fn

    return wrapper


def collect_apis(*components: object) -> Dict[ClientType, Dict[str, Callable[..., Any]]]:
    apis: Dict[ClientType, Dict[str, Callable[..., Any]]] = {}
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


def catch_protocol_errors(
    fn: Callable[P, Awaitable[dict[str, object]]]
) -> Callable[P, Awaitable[dict[str, object]]]:
    @wraps(fn)
    async def wrapper(*args: P.args, **kwargs: P.kwargs) -> dict[str, object]:
        try:
            return await fn(*args, **kwargs)

        except InvalidMessageError as exc:
            return {
                "status": "bad_message",
                "errors": exc.errors,
                "reason": "Invalid message.",
            }

        except ProtocolError as exc:
            return {"status": "bad_message", "reason": str(exc)}

    return wrapper


class CancelledByNewCmd(Exception):
    def __init__(self, new_raw_req: Union[None, bytes, str]) -> None:
        self.new_raw_req = new_raw_req


async def run_with_cancel_on_client_sending_new_cmd(
    websocket: Websocket, fn: Callable[P, Awaitable[T]], *args: P.args, **kwargs: P.kwargs
) -> dict[str, object]:
    """
    This is kind of a special case here:
    unlike other requests this one is going to (potentially) take
    a long time to complete. In the meantime we must monitor the
    connection with the client in order to make sure it is still
    online and handles websocket pings
    """

    rep = None

    async def _keep_websocket_breathing() -> None:
        # If a command is received, the client is violating the
        # request/reply pattern. We consider this as an order to stop
        # listening events.
        raw_req = await websocket.receive()
        raise CancelledByNewCmd(raw_req)

    async def _do_fn(cancel_scope: CancelScope) -> None:
        nonlocal rep
        rep = await fn(*args, **kwargs)
        cancel_scope.cancel()

    async with open_service_nursery() as nursery:
        nursery.start_soon(_do_fn, nursery.cancel_scope)
        nursery.start_soon(_keep_websocket_breathing)

    assert isinstance(rep, dict)
    return rep


# Unset singleton used as default value in function parameter when `None`
# can be a valid value.
# We implement this as an enum to satisfy type checker (see
# https://github.com/python/typing/issues/689#issuecomment-561425237)
UnsetType = Enum("UnsetType", "Unset")
Unset: Final = UnsetType.Unset
UnsetType = Literal[UnsetType.Unset]
