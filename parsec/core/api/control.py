from multiprocessing import Process

from parsec.networking import ClientContext
from parsec.schema import BaseCmdSchema, fields
from parsec.core.app import Core
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.devices_manager import DeviceLoadingError
from parsec.utils import ParsecError, to_jsonb64, from_jsonb64, ejson_dumps

try:
    from parsec.ui import fuse
except NameError:
    fuse = None


class PathOnlySchema(BaseCmdSchema):
    path = fields.String(required=True)


class cmd_LOGIN_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    password = fields.String(missing=None)


class cmd_FUSE_START_Schema(BaseCmdSchema):
    mountpoint = fields.String(required=True)


async def login(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if core.auth_device:
        return {"status": "already_logged", "reason": "Already logged"}

    msg = cmd_LOGIN_Schema().load_or_abort(req)
    try:
        device = core.devices_manager.load_device(msg["id"], msg["password"])
    except DeviceLoadingError:
        return {"status": "unknown_user", "reason": "Unknown user"}

    await core.login(device)
    return {"status": "ok"}


async def logout(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    await core.logout()
    return {"status": "ok"}


async def info(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    return {
        "status": "ok",
        # TODO: replace by `logged_in`
        "loaded": bool(core.auth_device),
        # TODO: replace by `device_id` ?
        "id": core.auth_device.id if core.auth_device else None,
    }


async def list_available_logins(
    req: dict, client_ctx: ClientContext, core: Core
) -> dict:
    devices = core.devices_manager.list_available_devices()
    return {"status": "ok", "devices": devices}


async def get_core_state(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    status = {"status": "ok", "login": None, "backend_online": False}
    if core.auth_device:
        status["login"] = core.auth_device.id
        try:
            await core.backend_connection.ping()
            status["backend_online"] = True
        except BackendNotAvailable:
            status["backend_online"] = False
    return status


# TODO: create a fuse module to handle fusepy/libfuse availability


async def fuse_start(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    raise NotImplementedError()


#     if not core.auth_device:
#         return {"status": "login_required", "reason": "Login required"}

#     msg = cmd_FUSE_START_Schema().load_or_abort(req)
#     if core.fuse_process:
#         return {"status": "fuse_already_started", "reason": "Fuse already started"}

#     core.mountpoint = msg["mountpoint"]
#     if os.name == "posix":
#         try:
#             os.makedirs(core.mountpoint)
#         except FileExistsError:
#             pass
#     core.fuse_process = Process(
#         target=fuse.start_fuse, args=(core.config.addr, core.mountpoint)
#     )
#     core.fuse_process.start()
#     if os.name == "nt":
#         if not os.path.isabs(core.mountpoint):
#             core.mountpoint = os.path.join(os.getcwd(), core.mountpoint)
#         await trio.sleep(1)
#         subprocess.Popen(
#             "net use p: \\\\localhost\\"
#             + core.mountpoint[0]
#             + "$"
#             + core.mountpoint[2:],
#             shell=True,
#         )
#     return {"status": "ok"}


async def fuse_stop(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    raise NotImplementedError()


#     if not core.auth_device:
#         return {"status": "login_required", "reason": "Login required"}

#     BaseCmdSchema().load_or_abort(req)  # empty msg expected
#     if not core.fuse_process:
#         return {"status": "fuse_not_started", "reason": "Fuse not started"}

#     core.fuse_process.terminate()
#     core.fuse_process.join()
#     core.fuse_process = None
#     core.mountpoint = None
#     if os.name == "nt":
#         subprocess.call("net use p: /delete /y", shell=True)
#     return {"status": "ok"}


async def fuse_open(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    raise NotImplementedError()


#     if not core.auth_device:
#         return {"status": "login_required", "reason": "Login required"}

#     msg = PathOnlySchema().load_or_abort(req)
#     if not core.fuse_process:
#         return {"status": "fuse_not_started", "reason": "Fuse not started"}

#     webbrowser.open(os.path.join(core.mountpoint, msg["path"][1:]))
#     return {"status": "ok"}
