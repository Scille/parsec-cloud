from nacl.public import SealedBox, PublicKey

from parsec.networking import ClientContext
from parsec.schema import BaseCmdSchema, fields
from parsec.core.app import Core
from parsec.core.backend_connection import BackendNotAvailable
from parsec.utils import to_jsonb64, from_jsonb64, ejson_dumps


class cmd_SHARE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    recipient = fields.String(required=True)


async def share(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    # TODO: super rough stuff...
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    try:
        cmd_SHARE_Schema().load_or_abort(req)
        entry = await core.fs.fetch_path(req["path"])
        # Cannot share a placeholder !
        if entry.is_placeholder:
            # TODO: use minimal_sync_if_placeholder ?
            await entry.sync()
        sharing_msg = {
            "type": "share",
            "content": entry._access.dump(with_type=False),
            "name": entry.name,
        }

        recipient = req["recipient"]
        rep = await core.backend_connection.send(
            {"cmd": "user_get", "user_id": recipient}
        )
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

    except BackendNotAvailable:
        return {"status": "backend_not_availabled", "reason": "Backend not available"}

    return {"status": "ok"}
