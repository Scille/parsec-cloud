from parsec.core.fs import FSInvalidPath
from parsec.core.app import Core, ClientContext
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.api.event import (
    event_subscribe,
    event_unsubscribe,
    event_listen,
    event_list_subscribed,
)
from parsec.core.api.management import (
    user_invite,
    user_claim,
    device_declare,
    device_configure,
    device_get_configuration_try,
    device_accept_configuration_try,
)
from parsec.core.api.control import (
    login,
    logout,
    info,
    list_available_logins,
    get_core_state,
    fuse_start,
    fuse_stop,
    fuse_open,
)
from parsec.core.api.fs import (
    file_create,
    file_read,
    file_write,
    flush,
    synchronize,
    stat,
    folder_create,
    move,
    delete,
    file_truncate,
)
from parsec.core.api.sharing import share
from parsec.utils import ParsecError


CMDS_DISPATCH = {
    "event_subscribe": event_subscribe,
    "event_unsubscribe": event_unsubscribe,
    "event_listen": event_listen,
    "event_list_subscribed": event_list_subscribed,
    "user_invite": user_invite,
    "user_claim": user_claim,
    "device_declare": device_declare,
    "device_configure": device_configure,
    "device_get_configuration_try": device_get_configuration_try,
    "device_accept_configuration_try": device_accept_configuration_try,
    "login": login,
    "logout": logout,
    "info": info,
    "list_available_logins": list_available_logins,
    "get_core_state": get_core_state,
    "fuse_start": fuse_start,
    "fuse_stop": fuse_stop,
    "fuse_open": fuse_open,
    "file_create": file_create,
    "file_read": file_read,
    "file_write": file_write,
    "flush": flush,
    "synchronize": synchronize,
    "stat": stat,
    "folder_create": folder_create,
    "move": move,
    "delete": delete,
    "file_truncate": file_truncate,
    "share": share,
}


async def dispatch_request(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    try:
        cmd_func = CMDS_DISPATCH[req["cmd"]]

    except KeyError as exc:
        cmd = req.get("cmd")
        if cmd:
            return {"status": "unknown_command", "reason": "Unknown command `%s`" % cmd}

        else:
            return {"status": "unknown_command", "reason": "Missing field `cmd`"}

    try:
        return await cmd_func(req, client_ctx, core)

    # Protect againsts generic exceptions
    except ParsecError as exc:
        import pdb

        pdb.set_trace()
        return exc.to_dict()

    except FSInvalidPath as exc:
        return {"status": "invalid_path", "reason": str(exc)}

    except BackendNotAvailable as exc:
        return {"status": "backend_not_available", "reason": str(exc)}


__all__ = ("dispatch_request",)
