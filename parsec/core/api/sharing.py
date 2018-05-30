from nacl.public import SealedBox, PublicKey

from parsec.schema import BaseCmdSchema, fields
from parsec.core.app import Core, ClientContext
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs.utils import is_placeholder_access, normalize_path
from parsec.utils import to_jsonb64, from_jsonb64, ejson_dumps


class cmd_SHARE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    recipient = fields.String(required=True)


class RetrySharing(Exception):
    pass


async def share(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    # TODO: super rough stuff...
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}
    req = cmd_SHARE_Schema().load(req)
    while True:
        try:
            return await _share(core, req['recipient'], req['path'])
        except RetrySharing:
            continue
        except BackendNotAvailable:
            return {"status": "backend_not_availabled", "reason": "Backend not available"}

async def _share(core, recipient, path):
    # TODO: leaky abstraction...
    _, entry_name = normalize_path(path)
    access, _ = await core.fs._local_tree.retrieve_entry(path)

    # Cannot share a placeholder !
    if is_placeholder_access(access):
        await core.fs.sync(path)
        raise RetrySharing()

    sharing_msg = {
        "type": "share",
        "author": core.auth_device.id,
        "content": access,
        "name": entry_name,
    }

    recipient = recipient
    rep = await core.backend_connection.send({"cmd": "user_get", "user_id": recipient})
    if rep["status"] != "ok":
        # TODO: better cooking of the answer
        return rep

    broadcast_key = PublicKey(from_jsonb64(rep["broadcast_key"]))
    box = SealedBox(broadcast_key)
    sharing_msg_clear = ejson_dumps(sharing_msg).encode("utf8")
    sharing_msg_signed = core.auth_device.device_signkey.sign(sharing_msg_clear)
    sharing_msg_encrypted = box.encrypt(sharing_msg_signed)

    rep = await core.backend_connection.send(
        {
            "cmd": "message_new",
            "recipient": recipient,
            "body": to_jsonb64(sharing_msg_encrypted),
        }
    )
    if rep["status"] != "ok":
        # TODO: better cooking of the answer
        return rep

    return {"status": "ok"}
