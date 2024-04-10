# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

from typing import (
    TYPE_CHECKING,
    Any,
    Awaitable,
    Callable,
    ForwardRef,
    Type,
    TypeVar,
    get_type_hints,
)

from parsec._parsec import ApiVersion

if TYPE_CHECKING:
    from parsec.client_context import (
        AnonymousClientContext,
        AuthenticatedClientContext,
        InvitedClientContext,
    )

    # TODO: Add constraints to Req/Rep with relation to each-other
    Req = TypeVar("Req")
    Rep = TypeVar("Rep")
    ClientContext = TypeVar(
        "ClientContext", AuthenticatedClientContext, InvitedClientContext, AnonymousClientContext
    )
    ApiFn = Callable[[ClientContext, Req], Awaitable[Rep | None]]
    ApiFnWithSelf = Callable[[Any, ClientContext, Req], Awaitable[Rep | None]]


ALLOWED_API_VERSIONS = {
    ApiVersion.API_V4_VERSION.version,
}


def api(fn: ApiFnWithSelf) -> ApiFnWithSelf:  # pyright: ignore[reportMissingTypeArgument] see TODO for Req/Rep above
    # Unlike req/rep types that have an absolute path, `client_ctx`'s type is only
    # specified in the signature by its name, hence we must hack `localns` so that
    # it contains the different possible type names.
    # Note only the key is important here (as we just want to know which type
    # `client_ctx` is in the given signature).
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
    }
    return fn


def collect_apis(*components: Any, include_ping: bool) -> dict[Type[Any], ApiFn]:  # pyright: ignore[reportMissingTypeArgument] see TODO for Req/Rep above
    apis: dict[Type[Any], ApiFn] = {}  # pyright: ignore[reportMissingTypeArgument] see TODO for Req/Rep above
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
