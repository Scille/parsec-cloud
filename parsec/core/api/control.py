from marshmallow import validate
import os

from parsec.schema import BaseCmdSchema, fields
from parsec.core.app import Core, ClientContext
from parsec.core.fuse_manager import FuseNotAvailable, FuseAlreadyStarted, FuseNotStarted
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.devices_manager import DeviceLoadingError


class _PathOnlySchema(BaseCmdSchema):
    path = fields.String(required=True)


PathOnlySchema = _PathOnlySchema()


class _cmd_LOGIN_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    password = fields.String(missing=None)


cmd_LOGIN_Schema = _cmd_LOGIN_Schema()


class _cmd_FUSE_START_Schema(BaseCmdSchema):
    if os.name == "nt":
        mountpoint = fields.String(required=True, validate=validate.Regexp(r"^[A-Z]:$"))
    else:
        mountpoint = fields.String(required=True)


cmd_FUSE_START_Schema = _cmd_FUSE_START_Schema()


async def login(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if core.auth_device:
        return {"status": "already_logged", "reason": "Already logged"}

    msg = cmd_LOGIN_Schema.load(req)
    try:
        device = core.local_devices_manager.load_device(msg["id"], msg["password"])
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


async def list_available_logins(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    devices = core.local_devices_manager.list_available_devices()
    return {"status": "ok", "devices": devices}


async def get_core_state(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    status = {"status": "ok", "login": None, "backend_online": False}
    if core.auth_device:
        status["login"] = core.auth_device.id
        try:
            await core.backend_cmds_sender.ping()
            status["backend_online"] = True
        except BackendNotAvailable:
            status["backend_online"] = False
    return status


# TODO: create a fuse module to handle fusepy/libfuse availability


async def fuse_start(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_FUSE_START_Schema.load(req)

    try:
        await core.fuse_manager.start_mountpoint(msg["mountpoint"])
    except FuseNotAvailable as exc:
        return {"status": "fuse_not_available", "reason": str(exc)}

    except FuseAlreadyStarted as exc:
        return {"status": "fuse_already_started", "reason": str(exc)}

    return {"status": "ok"}


async def fuse_stop(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    BaseCmdSchema.load(req)  # empty msg expected

    try:
        await core.fuse_manager.stop_mountpoint()
    except FuseNotAvailable as exc:
        return {"status": "fuse_not_available", "reason": str(exc)}

    except FuseNotStarted as exc:
        return {"status": "fuse_not_started", "reason": str(exc)}

    return {"status": "ok"}


async def fuse_open(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = PathOnlySchema.load(req)

    try:
        core.fuse_manager.open_file(msg["path"])
    except FuseNotAvailable as exc:
        return {"status": "fuse_not_available", "reason": str(exc)}

    except FuseNotStarted as exc:
        return {"status": "fuse_not_started", "reason": str(exc)}

    return {"status": "ok"}
