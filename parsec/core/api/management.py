from parsec.schema import BaseCmdSchema, fields, validate
from parsec.core.app import Core, ClientContext
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core import devices_manager
from parsec.core.devices_manager import (
    DeviceSavingError,
    DeviceConfigureError,
    DeviceConfigureBackendError,
    DeviceConfigureOutOfDate,
    invite_user,
    accept_device_configuration_try,
    configure_new_device,
    claim_user,
)


class _cmd_USER_INVITE_Schema(BaseCmdSchema):
    user_id = fields.String(
        validate=validate.Regexp(devices_manager.USER_ID_PATTERN), required=True
    )


cmd_USER_INVITE_Schema = _cmd_USER_INVITE_Schema()


class _cmd_USER_CLAIM_Schema(BaseCmdSchema):
    # TODO: change id to user_id/device_name ?
    id = fields.String(validate=validate.Regexp(devices_manager.DEVICE_ID_PATTERN), required=True)
    invitation_token = fields.String(required=True)
    password = fields.String(missing=None)
    use_nitrokey = fields.Boolean(missing=False)
    nitrokey_token_id = fields.Integer(missing=0)
    nitrokey_key_id = fields.Integer(missing=0)


cmd_USER_CLAIM_Schema = _cmd_USER_CLAIM_Schema()


class _cmd_DEVICE_DECLARE_Schema(BaseCmdSchema):
    device_name = fields.String(required=True)


cmd_DEVICE_DECLARE_Schema = _cmd_DEVICE_DECLARE_Schema()


class _cmd_DEVICE_CONFIGURE_Schema(BaseCmdSchema):
    # TODO: add regex validation
    device_id = fields.String(required=True)
    password = fields.String(missing=None)
    use_nitrokey = fields.Boolean(missing=False)
    nitrokey_token_id = fields.Integer(missing=0)
    nitrokey_key_id = fields.Integer(missing=0)
    configure_device_token = fields.String(required=True)


cmd_DEVICE_CONFIGURE_Schema = _cmd_DEVICE_CONFIGURE_Schema()


class _cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema(BaseCmdSchema):
    config_try_id = fields.String(required=True)
    password = fields.String(missing=None)
    nitrokey_pin = fields.String(missing="")
    nitrokey_token_id = fields.Integer(missing=0)
    nitrokey_key_id = fields.Integer(missing=0)


cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema = _cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema()


async def user_invite(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_USER_INVITE_Schema.load(req)
    try:
        invitation_token = await invite_user(core.backend_cmds_sender, msg["user_id"])
    except BackendNotAvailable:
        return {"status": "backend_not_available", "reason": "Backend not available"}
    except DeviceConfigureBackendError as exc:
        return {"stauts": "backend_error", "reason": str(exc)}

    return {"status": "ok", "user_id": msg["user_id"], "invitation_token": invitation_token}


async def user_claim(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if core.auth_device:
        return {"status": "already_logged", "reason": "Already logged"}

    msg = cmd_USER_CLAIM_Schema.load(req)
    user_id, device_name = msg["id"].split("@")
    try:
        user_privkey, device_signkey, user_manifest_access = await claim_user(
            core.backend_addr, user_id, device_name, msg["invitation_token"]
        )
    except BackendNotAvailable:
        return {"status": "backend_not_available", "reason": "Backend not available"}
    except DeviceConfigureOutOfDate:
        return {"status": "out_of_date_error", "reason": "Claim code is too old."}
    except DeviceConfigureBackendError as exc:
        return {"stauts": "backend_error", "reason": str(exc)}

    try:
        core.local_devices_manager.register_new_device(
            msg["id"],
            user_privkey.encode(),
            device_signkey.encode(),
            user_manifest_access,
            msg["password"],
            msg["use_nitrokey"],
            msg["nitrokey_token_id"],
            msg["nitrokey_key_id"],
        )
    except DeviceSavingError:
        return {"status": "already_exists", "reason": "User config already exists"}

    return {"status": "ok"}


async def device_declare(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_DEVICE_DECLARE_Schema.load(req)
    try:
        return await core.backend_cmds_sender.send({"cmd": "device_declare", **msg})
    except BackendNotAvailable as exc:
        return {"status": "backend_not_available", "reason": "Backend not available"}


async def device_configure(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    msg = cmd_DEVICE_CONFIGURE_Schema.load(req)

    try:
        user_privkey, device_signkey, user_manifest_access = await configure_new_device(
            core.backend_addr,
            msg["device_id"],
            msg["configure_device_token"],
            msg["password"],
            msg["use_nitrokey"],
            msg["nitrokey_token_id"],
            msg["nitrokey_key_id"],
        )
    except BackendNotAvailable as exc:
        return {"status": "backend_not_available", "reason": "Backend not available"}
    except DeviceConfigureError as exc:
        return {"status": "device_configure_error", "reason": str(exc)}

    try:
        core.local_devices_manager.register_new_device(
            msg["device_id"],
            user_privkey.encode(),
            device_signkey.encode(),
            user_manifest_access,
            msg["password"],
            msg["use_nitrokey"],
            msg["nitrokey_token_id"],
            msg["nitrokey_key_id"],
        )
    except DeviceSavingError:
        return {"status": "already_exists", "reason": "User config already exists"}

    return {"status": "ok"}


async def device_get_configuration_try(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    try:
        # Passthrough
        return await core.backend_cmds_sender.send(req)
    except BackendNotAvailable as exc:
        return {"status": "backend_not_available", "reason": "Backend not available"}


async def device_accept_configuration_try(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema.load(req)

    try:
        await accept_device_configuration_try(
            core.backend_cmds_sender,
            core.auth_device,
            msg["config_try_id"],
            msg["password"],
            msg["nitrokey_pin"],
            msg["nitrokey_token_id"],
            msg["nitrokey_key_id"],
        )
    except BackendNotAvailable:
        return {"status": "backend_not_available", "reason": "Backend not available"}
    except DeviceConfigureError as exc:
        return {"status": "device_configure_error", "reason": str(exc)}

    return {"status": "ok"}
