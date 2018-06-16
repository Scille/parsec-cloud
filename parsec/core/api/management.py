from nacl.public import PrivateKey, PublicKey, SealedBox
from nacl.signing import SigningKey

from parsec.schema import UnknownCheckedSchema, BaseCmdSchema, fields, validate
from parsec.core.fs.data import new_access
from parsec.core.app import Core, ClientContext
from parsec.core.backend_connection import BackendNotAvailable, backend_send_anonymous_cmd
from parsec.core import devices_manager
from parsec.core.devices_manager import DeviceSavingError, user_manifest_access_schema
from parsec.utils import to_jsonb64, from_jsonb64


class BackendGetConfigurationTrySchema(UnknownCheckedSchema):
    status = fields.CheckedConstant("ok", required=True)
    device_name = fields.String(required=True)
    configuration_status = fields.String(required=True)
    device_verify_key = fields.Base64Bytes(required=True)
    exchange_cipherkey = fields.Base64Bytes(required=True)


backend_get_configuration_try_schema = BackendGetConfigurationTrySchema()


class cmd_USER_INVITE_Schema(BaseCmdSchema):
    user_id = fields.String(
        validate=validate.Regexp(devices_manager.USER_ID_PATTERN), required=True
    )


class cmd_USER_CLAIM_Schema(BaseCmdSchema):
    # TODO: change id to user_id/device_name ?
    id = fields.String(validate=validate.Regexp(devices_manager.DEVICE_ID_PATTERN), required=True)
    invitation_token = fields.String(required=True)
    password = fields.String(required=True)


class cmd_DEVICE_CONFIGURE_Schema(BaseCmdSchema):
    # TODO: add regex validation
    device_id = fields.String(required=True)
    password = fields.String(required=True)
    configure_device_token = fields.String(required=True)


class cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema(BaseCmdSchema):
    config_try_id = fields.String(required=True)


async def user_invite(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_USER_INVITE_Schema().load(req)
    try:
        rep = await core.backend_cmds_sender.send({"cmd": "user_invite", "user_id": msg["user_id"]})
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    return rep


async def user_claim(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if core.auth_device:
        return {"status": "already_logged", "reason": "Already logged"}

    msg = cmd_USER_CLAIM_Schema().load(req)
    user_privkey = PrivateKey.generate()
    device_signkey = SigningKey.generate()
    user_manifest_access = new_access()
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

    if rep["status"] != "ok":
        return rep

    try:
        core.devices_manager.register_new_device(
            msg["id"],
            user_privkey.encode(),
            device_signkey.encode(),
            user_manifest_access,
            msg["password"],
        )
    except DeviceSavingError:
        return {"status": "already_exists", "reason": "User config already exists"}

    return rep


async def _backend_passthrough(core: Core, req: dict) -> dict:
    try:
        rep = await core.backend_cmds_sender.send(req)
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    return rep


async def device_declare(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    return await _backend_passthrough(core, req)


async def device_configure(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    msg = cmd_DEVICE_CONFIGURE_Schema().load(req)

    user_id, device_name = msg["device_id"].split("@")
    exchange_cipherkey_privkey = PrivateKey.generate()
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
                "exchange_cipherkey": to_jsonb64(exchange_cipherkey_privkey.public_key.encode()),
            },
        )
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    if rep["status"] != "ok":
        return rep

    ciphered = from_jsonb64(rep["ciphered_user_privkey"])
    box = SealedBox(exchange_cipherkey_privkey)
    user_privkey_raw = box.decrypt(ciphered)
    user_privkey = PrivateKey(user_privkey_raw)

    ciphered = from_jsonb64(rep["ciphered_user_manifest_access"])
    user_manifest_access_raw = box.decrypt(ciphered)
    user_manifest_access, errors = user_manifest_access_schema.loads(user_manifest_access_raw)
    # TODO: improve data validation
    assert not errors

    core.devices_manager.register_new_device(
        msg["device_id"],
        user_privkey.encode(),
        device_signkey.encode(),
        user_manifest_access,
        msg["password"],
    )

    return {"status": "ok"}


async def device_get_configuration_try(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    return await _backend_passthrough(core, req)


async def device_accept_configuration_try(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    msg = cmd_DEVICE_ACCEPT_CONFIGURATION_TRY_Schema().load(req)

    try:
        rep = await core.backend_cmds_sender.send(
            {"cmd": "device_get_configuration_try", "config_try_id": msg["config_try_id"]}
        )
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    data, errors = backend_get_configuration_try_schema.load(rep)
    if errors:
        return {
            "status": "backend_error",
            "reason": "Bad response from backend: %r (%r)" % (rep, errors),
        }

    exchange_cipherkey_raw = data["exchange_cipherkey"]
    box = SealedBox(PublicKey(exchange_cipherkey_raw))
    ciphered_user_privkey = box.encrypt(core.auth_device.user_privkey.encode())

    user_manifest_access_raw, _ = user_manifest_access_schema.dumps(
        core.auth_device.user_manifest_access
    )
    ciphered_user_manifest_access = box.encrypt(user_manifest_access_raw.encode())

    try:
        rep = await core.backend_cmds_sender.send(
            {
                "cmd": "device_accept_configuration_try",
                "config_try_id": msg["config_try_id"],
                "ciphered_user_privkey": to_jsonb64(ciphered_user_privkey),
                "ciphered_user_manifest_access": to_jsonb64(ciphered_user_manifest_access),
            }
        )
    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    if rep != "ok":
        return rep

    return {"status": "ok"}
