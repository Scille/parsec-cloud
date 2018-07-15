from parsec.schema import BaseCmdSchema, fields
from parsec.core.app import Core, ClientContext
from parsec.core.fs.sharing import SharingRecipientError, SharingBackendMessageError, SharingError


class cmd_SHARE_Schema(BaseCmdSchema):
    path = fields.Path(required=True)
    recipient = fields.String(required=True)


cmd_share_schema = cmd_SHARE_Schema()


async def share(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    req = cmd_share_schema.load(req)
    try:
        await core.fs.share(req["path"], req["recipient"])
    except OSError as exc:
        return {"status": "invalid_path", "reason": str(exc)}
    except SharingRecipientError as exc:
        return {"status": "bad_recipient", "reason": str(exc)}
    except SharingBackendMessageError as exc:
        return {"status": "backend_error", "reason": str(exc)}
    except SharingError as exc:
        return {"status": "sharing_error", "reason": str(exc)}

    return {"status": "ok"}
