from parsec.schema import BaseCmdSchema, fields
from parsec.core.app import Core, ClientContext


class cmd_SHARE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    recipient = fields.String(required=True)


async def share(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    # TODO: super rough stuff...
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    cmd_SHARE_Schema().load_or_abort(req)
    await core.sharing.share(req["path"], req["recipient"])
    return {"status": "ok"}
