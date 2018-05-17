from parsec.schema import BaseCmdSchema, fields
from parsec.core.app import Core, ClientContext
from parsec.core.sharing import SharingError
from parsec.core.backend_connection import BackendNotAvailable
from parsec.backend.exceptions import NotFoundError


class cmd_SHARE_Schema(BaseCmdSchema):
    path = fields.String(required=True)
    recipient = fields.String(required=True)


async def share(req: dict, client_ctx: ClientContext, core: Core) -> dict:
    # TODO: super rough stuff...
    if not core.auth_device:
        return {"status": "login_required", "reason": "Login required"}

    cmd_SHARE_Schema().load_or_abort(req)
    try:
        await core.sharing.share(req["path"], req["recipient"])
    except NotFoundError as exc:
        return {"status": "not_found", "reason": str(exc)}
    except BackendNotAvailable as exc:
        return {"status": "backend_not_available", "reason": "Backend not available"}
    except SharingError as exc:
        return {"status": "sharing_error", "reason": str(exc)}
    return {"status": "ok"}
