# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from enum import Enum
from typing import (
    TYPE_CHECKING,
    Any,
    Awaitable,
    Callable,
    Dict,
    ForwardRef,
    Type,
    TypeVar,
    Union,
    get_type_hints,
)

from quart import Websocket
from trio import CancelScope
from typing_extensions import Final, Literal, ParamSpec

from parsec._parsec import ApiVersion
from parsec.utils import open_service_nursery

if TYPE_CHECKING:
    from parsec.backend.client_context import BaseClientContext

    Ctx = TypeVar("Ctx", bound=BaseClientContext)
    T = TypeVar("T")
    P = ParamSpec("P")


PEER_EVENT_MAX_WAIT = 3  # 5mn
ALLOWED_API_VERSIONS = {
    ApiVersion.API_V3_VERSION.version,
}


# Enumeration used to check access rights for a given kind of operation
OperationKind = Enum("OperationKind", "DATA_READ DATA_WRITE MAINTENANCE")


def api(fn: Callable[P, T]) -> Callable[P, T]:
    assert not hasattr(fn, "_api_info")

    # Unlike req/rp that have an absolute path, `client_ctx` is only specified in the
    # signature by it name, hence we must hack `localns` so that it find something
    # (as we don't really care to find the right element !)
    types = get_type_hints(
        fn,
        localns={
            "AuthenticatedClientContext": "AuthenticatedClientContext",
            "InvitedClientContext": "InvitedClientContext",
            "AnonymousClientContext": "AnonymousClientContext",
        },
    )
    assert types.keys() == {"client_ctx", "req", "return"}
    cmd_mod = types["req"].__module__
    _, _, m_family, _, m_cmd = cmd_mod.split(".")
    assert types["return"].__module__ == cmd_mod
    assert cmd_mod.startswith("parsec._parsec.")
    assert types["req"].__name__ == "Req"
    assert types["return"].__name__ == "Rep"

    if m_family == "authenticated_cmds":
        expected_type_name = "AuthenticatedClientContext"
    elif m_family == "invited_cmds":
        expected_type_name = "InvitedClientContext"
    else:
        assert m_family == "anonymous_cmds"
        expected_type_name = "AnonymousClientContext"
    assert types["client_ctx"] == ForwardRef(
        expected_type_name
    ), f"Expect `client_ctx` to be an `{expected_type_name}`"

    fn._api_info = {  # type: ignore[attr-defined]
        "cmd": m_cmd,
        "req_type": types["req"],
        "cancel_on_client_sending_new_cmd": False,
    }
    return fn


def api_ws_cancel_on_client_sending_new_cmd(fn: Callable[P, T]) -> Callable[P, T]:
    # `@api` must be placed first
    assert hasattr(fn, "_api_info")
    fn._api_info["cancel_on_client_sending_new_cmd"] = True
    return fn


def collect_apis(
    *components: Any, include_ping: bool
) -> Dict[Type[Any], Callable[[BaseClientContext, Any], Any]]:
    apis: Dict[Type[Any], Callable[[BaseClientContext, Any], Any]] = {}
    for component in components:
        for methname in dir(component):
            meth = getattr(component, methname)
            info = getattr(meth, "_api_info", None)
            if not info:
                continue

            # Ping command is only needed for tests
            if info["cmd"] == "ping" and not include_ping:
                continue

            already_present = apis.setdefault(info["req_type"], meth)
            assert already_present is meth

    return apis


class CancelledByNewCmd(Exception):
    def __init__(self, new_raw_req: Union[None, bytes, str]) -> None:
        self.new_raw_req = new_raw_req


async def run_with_cancel_on_client_sending_new_cmd(
    websocket: Websocket, fn: Callable[P, Awaitable[T]], *args: P.args, **kwargs: P.kwargs
) -> T:
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

    assert rep is not None
    return rep


# Unset singleton used as default value in function parameter when `None`
# can be a valid value.
# We implement this as an enum to satisfy type checker (see
# https://github.com/python/typing/issues/689#issuecomment-561425237)
UnsetType = Enum("UnsetType", "Unset")
Unset: Final = UnsetType.Unset
UnsetType = Literal[UnsetType.Unset]
