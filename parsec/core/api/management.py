from nacl.public import PrivateKey, PublicKey, SealedBox
from nacl.signing import SigningKey

from parsec.schema import BaseCmdSchema, fields
from parsec.core.app import Core, ClientContext
from parsec.core.backend_connection import BackendNotAvailable, backend_send_anonymous_cmd
from parsec.utils import to_jsonb64, from_jsonb64


# TODO: move complex stuff out of the api


class cmd_USER_INVITE_Schema(BaseCmdSchema):
    user_id = fields.String(required=True)


class cmd_USER_CLAIM_Schema(BaseCmdSchema):
    # TODO: change id to user_id/device_name ?
    id = fields.String(required=True)
    invitation_token = fields.String(required=True)
    password = fields.String(required=True)


class cmd_DEVICE_CONFIGURE_Schema(BaseCmdSchema):
    # TODO: add regex validation
    device_id = fields.String(required=True)
    password = fields.String(required=True)
    configure_device_token = fields.String(required=True)


class cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema(BaseCmdSchema):
    configuration_try_id = fields.String(required=True)


async def user_invite(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_USER_INVITE_Schema().load_or_abort(req)
    try:
        rep = await core.backend_connection.send({"cmd": "user_invite", "user_id": msg["user_id"]})
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    return rep


async def user_claim(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if core.auth_device:
        return {"status": "already_logged", "reason": "Already logged"}

    msg = cmd_USER_CLAIM_Schema().load_or_abort(req)
    user_privkey = PrivateKey.generate()
    device_signkey = SigningKey.generate()
    user_id, device_name = msg["id"].split("@")
    try:
        rep = await backend_send_anonymous_cmd(
            core.backend_addr,
            {
                "cmd": "user_claim",
                "user_id": user_id,
                "device_name": device_name,
                "invitation_token": msg["invitation_token"],
                "broadcast_key": to_jsonb64(user_privkey.public_key.encode()),
                "device_verify_key": to_jsonb64(device_signkey.verify_key.encode()),
            },
        )
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    core.devices_manager.register_new_device(
        msg["id"], user_privkey.encode(), device_signkey.encode(), msg["password"]
    )
    return rep


async def _backend_passthrough(core: Core, req: dict) -> dict:
    try:
        rep = await core.backend_connection.send(req)
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    return rep


async def device_declare(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    return await _backend_passthrough(core, req)


async def device_configure(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    msg = cmd_DEVICE_CONFIGURE_Schema().load_or_abort(req)

    user_id, device_name = msg["device_id"].split("@")
    user_privkey_cypherkey_privkey = PrivateKey.generate()
    device_signkey = SigningKey.generate()

    try:
        rep = await backend_send_anonymous_cmd(
            core.backend_addr,
            {
                "cmd": "device_configure",
                "user_id": user_id,
                "device_name": device_name,
                "configure_device_token": msg["configure_device_token"],
                "device_verify_key": to_jsonb64(device_signkey.verify_key.encode()),
                "user_privkey_cypherkey": to_jsonb64(
                    user_privkey_cypherkey_privkey.public_key.encode()
                ),
            },
        )
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    if rep["status"] != "ok":
        return rep

    cyphered = from_jsonb64(rep["cyphered_user_privkey"])
    box = SealedBox(user_privkey_cypherkey_privkey)
    user_privkey_raw = box.decrypt(cyphered)
    user_privkey = PrivateKey(user_privkey_raw)

    core.devices_manager.register_new_device(
        msg["device_id"], user_privkey.encode(), device_signkey.encode(), msg["password"]
    )

    return {"status": "ok"}


async def device_get_configuration_try(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    return await _backend_passthrough(core, req)


async def device_accept_configuration_try(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema().load_or_abort(req)

    conf_try = core._config_try_pendings.get(msg["configuration_try_id"])
    if not conf_try:
        return {"status": "unknown_configuration_try_id", "reason": "Unknown configuration try id"}

    user_privkey_cypherkey_raw = from_jsonb64(conf_try["user_privkey_cypherkey"])
    box = SealedBox(PublicKey(user_privkey_cypherkey_raw))
    cyphered_user_privkey = box.encrypt(core.auth_device.user_privkey.encode())

    try:
        rep = await core.backend_connection.send(
            {
                "cmd": "device_accept_configuration_try",
                "configuration_try_id": msg["configuration_try_id"],
                "cyphered_user_privkey": to_jsonb64(cyphered_user_privkey),
            }
        )
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    if rep != "ok":
        return rep

    return {"status": "ok"}
